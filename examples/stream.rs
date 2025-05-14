use starlink_grpc_client::client::DishClient;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DishClient::connect("http://dishy.starlink.com:9200").await?;
    let mut stream = client.stream_status().await?;
    println!("Listening for status updates… (press Ctrl+C to quit)");

    while let Some(status) = stream.next().await {
        println!("{:#?}", status?);
    }

    Ok(())
}
