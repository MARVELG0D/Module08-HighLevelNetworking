use services::transaction_service_client::TransactionServiceClient;
use services::TransactionRequest;

// Step 18: Set up the proto definitions
pub mod services {
    tonic::include_proto!("services");
}

// Step 19: Import necessary modules
use services::payment_service_client::PaymentServiceClient;
use services::PaymentRequest;

// Step 20: Create the main Function and Initialize the Client
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    // Step 35: Create Client Connection
    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;

    // Step 36: Create and Send Request
    let request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    // Step 37: Receive and Process Stream
    println!("\nFetching Transaction History...");
    let mut stream = transaction_client.get_transaction_history(request).await?.into_inner();

    while let Some(transaction) = stream.message().await? {
        println!("Transaction: {:?}", transaction);
    }

    Ok(())
}