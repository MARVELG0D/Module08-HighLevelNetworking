# Module08-HighLevelNetworking

# Reflection

**1. What are the key differences between unary, server streaming, and bi-directional streaming RPC methods, and in what scenarios would each be most suitable?**
* **Unary:** 1 request, 1 response. Best for standard CRUD operations (e.g., fetching a user profile).
* **Server Streaming:** 1 request, continuous responses. Best for live data feeds (e.g., stock price updates).
* **Bi-directional:** Continuous two-way communication over one connection. Best for real-time interactivity (e.g., chat apps, multiplayer games).

**2. What are the potential security considerations involved in implementing a gRPC service in Rust?**
* **Encryption:** Must use TLS to encrypt data in transit.
* **Authentication:** Use gRPC interceptors to verify user identity via tokens (like JWTs) in the request headers.
* **Authorization:** Ensure the authenticated user actually has permission to perform the requested action (Role-Based Access Control).

**3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC?**
* **State Management:** Safely managing shared state (like a list of connected users) across asynchronous tasks without causing deadlocks.
* **Disconnections:** Gracefully handling unexpected client drops and cleaning up dead connections.
* **Backpressure:** Preventing memory exhaustion if a client sends messages faster than the server can process them.

**4. What are the advantages and disadvantages of using `tokio_stream::wrappers::ReceiverStream`?**
* **Advantage:** It makes it incredibly easy to convert standard Tokio channels (`mpsc`) into gRPC streams without complex boilerplate.
* **Disadvantage:** It tightly couples your streaming logic to the Tokio async runtime, making it hard to switch out later.

**5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity?**
* **Layering:** Separate the gRPC network code from the actual business logic.
* **Dependency Injection:** Pass databases and external services via traits so they can be easily mocked during testing.
* **Shared Crates:** Put the generated Protobuf structs into a shared module/crate so multiple microservices can use the exact same types.

**6. In the `MyPaymentService` implementation, what additional steps might be necessary for complex logic?**
* **Idempotency:** Add unique transaction IDs to ensure a user isn't double-charged if their network drops and they retry.
* **External APIs:** Integrate with actual payment gateways (like Stripe).
* **Database Transactions:** Ensure updates (like deducting balances) are atomic (ACID) and roll back completely if an error occurs.

**7. What impact does gRPC have on the architecture of distributed systems?**
It enforces a **"contract-first"** design. Because all services rely on `.proto` files, it guarantees strict consistency and makes interoperability between different programming languages seamless and type-safe.

**8. What are the advantages and disadvantages of HTTP/2 (gRPC) compared to HTTP/1.1 (REST)?**
* **Advantages:** HTTP/2 is faster, supports multiplexing (multiple requests at once), and uses efficient binary framing instead of heavy text.
* **Disadvantages:** It is binary, not text. This makes it harder to debug manually in the browser or via terminal compared to plain JSON in HTTP/1.1.

**9. How does the request-response model of REST APIs contrast with gRPC bidirectional streaming?**
REST relies on the client constantly asking the server for updates (polling), which creates network overhead and latency. gRPC bidirectional streaming keeps a single connection open, allowing the server to push updates instantly with near-zero latency.

**10. What are the implications of gRPC's schema-based approach (Protobuf) vs JSON in REST?**
* **Protobuf:** Strictly typed, highly compressed, and very fast to process. It prevents accidental data mismatches but lacks flexibility.
* **JSON:** Highly flexible, schema-less, and easy for humans to read, but it is larger in payload size and much slower to parse.
