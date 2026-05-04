pub mod services {
    tonic::include_proto!("services");
}

use tokio::io::{self, AsyncBufReadExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;

use services::payment_service_client::PaymentServiceClient;
use services::PaymentRequest;
use services::transaction_service_client::TransactionServiceClient;
use services::TransactionRequest;
use services::chat_service_client::ChatServiceClient;
use services::ChatMessage;

// --- CLIENT MAIN FUNCTION ---
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 1. Test Payment Service (Unary)
    println!("--- Testing Payment Service ---");
    let mut payment_client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    let payment_request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });
    let payment_response = payment_client.process_payment(payment_request).await?.into_inner();
    println!("Payment Response: {:?}\n", payment_response);

    // 2. Test Transaction Service (Server Streaming)
    println!("--- Testing Transaction Service ---");
    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;
    let transaction_request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });
    println!("Fetching Transaction History...");
    let mut stream = transaction_client.get_transaction_history(transaction_request).await?.into_inner();
    while let Some(transaction) = stream.message().await? {
        println!("Transaction: {:?}", transaction);
    }

    // 3. Test Chat Service (Bi-Directional)
    println!("\n--- Starting Chat Session ---");
    println!("Type your messages below! (Press Ctrl+C to exit)");

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut chat_client = ChatServiceClient::new(channel);
    let (tx, rx) = mpsc::channel(10);

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let message = ChatMessage {
                user_id: "user_123".to_string(),
                message: line,
            };
            if tx.send(message).await.is_err() {
                break;
            }
        }
    });

    let request = tonic::Request::new(ReceiverStream::new(rx));
    let mut response_stream = chat_client.chat(request).await?.into_inner();
    while let Some(message) = response_stream.message().await? {
        println!("Server: {}", message.message);
    }

    Ok(())
}