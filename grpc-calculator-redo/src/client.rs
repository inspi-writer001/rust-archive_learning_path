use proto::admin_client::AdminClient;
use proto::calculator_client::CalculatorClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50052";

    let mut client = CalculatorClient::connect(url).await?;
    let mut admin_client = AdminClient::connect(url).await?;

    let req = proto::CalculationRequest { a: 4, b: 21 };
    let request = tonic::Request::new(req);

    let req_1 = proto::CalculationRequest { a: 2, b: 2 };
    let request_1 = tonic::Request::new(req_1);

    let response = client.add(request).await?;
    let response_1 = client.say_age(request_1).await?;

    let get_count = admin_client
        .get_count(tonic::Request::new(proto::GetCountRequest {}))
        .await?;
    println!("Response: {:?}", response.get_ref().result);
    println!("Second Response: {:?}", response_1.get_ref().response);
    println!(
        "Get current count Response: {:?}",
        get_count.get_ref().count
    );
    Ok(())
}
