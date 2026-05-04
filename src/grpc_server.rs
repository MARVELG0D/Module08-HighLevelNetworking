use tonic::{transport::Server, Request, Response, Status};

// Step 27: Use necessary library
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

// Step 28: Use necessary services
use services::transaction_service_server::{TransactionService, TransactionServiceServer};
use services::{TransactionRequest, TransactionResponse};

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

// Step 29: Define a struct
#[derive(Default)]
pub struct MyTransactionService {}

// Step 30: Implement the TransactionService trait
#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    type get_transaction_historyStream = ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<Self::get_transaction_historyStream>, Status> {
        println!("Received transaction history request: {:?}", request);

        // Channel Setup
        let (tx, rx) = mpsc::channel(4);

        // Stream Generation
        tokio::spawn(async move {
            for i in 1..=30 {
                let response = TransactionResponse {
                    transaction_id: format!("txn_{}", i),
                    status: "Completed".to_string(),
                    timestamp: "2026-05-04T12:00:00Z".to_string(),
                };

                // Send the transaction to the channel
                if tx.send(Ok(response)).await.is_err() {
                    println!("Client disconnected");
                    break;
                }

                // Add a slight delay to simulate real-world streaming
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default(); // Initialize the new service

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .add_service(TransactionServiceServer::new(transaction_service)) // Add it here
        .serve(addr)
        .await?;

    Ok(())
}