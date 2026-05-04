use tonic::{transport::Server, Request, Response, Status};

// Step 12: Define the public module for the generated proto code
pub mod services {
    tonic::include_proto!("services");
}

// Step 13: Use the services needed
use services::payment_service_server::{PaymentService, PaymentServiceServer};
use services::{PaymentRequest, PaymentResponse};

// Step 14: Define the Struct
#[derive(Default)]
pub struct MyPaymentService {}

// Step 15: Implement the service trait
#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);

        let req = request.into_inner();

        let response = PaymentResponse {
            success: true,
            message: format!("Payment of {} processed for user {}", req.amount, req.user_id),
        };

        Ok(Response::new(response))
    }
}

// Step 16: Implement PaymentService in main function, define the port and start the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())
}