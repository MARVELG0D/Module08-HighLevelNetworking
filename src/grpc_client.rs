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

    // Step 21: Create and send the request
    let request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let response = client.process_payment(request).await?.into_inner();
    println!("Response: {:?}", response);

    Ok(())
}