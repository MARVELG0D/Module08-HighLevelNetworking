pub mod services {
    tonic::include_proto!("services");
}

use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

// Payment Service
use services::payment_service_server::{PaymentService, PaymentServiceServer};
use services::{PaymentRequest, PaymentResponse};

// Transaction Service
use services::transaction_service_server::{TransactionService, TransactionServiceServer};
use services::{TransactionRequest, TransactionResponse};

// Chat Service
use services::chat_service_server::{ChatService, ChatServiceServer};
use services::ChatMessage;

// --- 1. Payment Service Implementation ---
#[derive(Default)]
pub struct MyPaymentService {}

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

// --- 2. Transaction Service Implementation ---
#[derive(Default)]
pub struct MyTransactionService {}

#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    type get_transaction_historyStream = ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<Self::get_transaction_historyStream>, Status> {
        println!("Received transaction history request: {:?}", request);

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for i in 1..=30 {
                let response = TransactionResponse {
                    transaction_id: format!("txn_{}", i),
                    status: "Completed".to_string(),
                    timestamp: "2026-05-04T12:00:00Z".to_string(),
                };

                if tx.send(Ok(response)).await.is_err() {
                    println!("Client disconnected");
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// --- 3. Chat Service Implementation ---
#[derive(Default)]
pub struct MyChatService {}

#[tonic::async_trait]
impl ChatService for MyChatService {
    type chatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::chatStream>, Status> {
        println!("Received chat request");

        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(10);

        tokio::spawn(async move {
            while let Some(message) = stream.message().await.unwrap_or_else(|_| None) {
                println!("User {}: {}", message.user_id, message.message);

                let reply = ChatMessage {
                    user_id: "Server".to_string(),
                    message: format!("Echo: {}", message.message),
                };

                if tx.send(Ok(reply)).await.is_err() {
                    println!("Client disconnected");
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// --- SERVER MAIN FUNCTION ---
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default();
    let chat_service = MyChatService::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .add_service(TransactionServiceServer::new(transaction_service))
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}