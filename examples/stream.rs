use starlink_grpc_client::client::DishClient;
use starlink_grpc_client::space_x::api::device::response::Response as ResponseOneof;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to your Dish gRPC endpoint
    let mut client = DishClient::connect("http://dishy.starlink.com:9200").await?;
    let mut stream = client.stream_status().await?;
    println!("Streaming downlink/uplink throughputs (Mb/s)… Ctrl+C to quit");

    while let Some(item) = stream.next().await {
        match item {
            Ok(status) => {
                // Efficiently match and print only the DishGetStatus variant
                if let Some(ResponseOneof::DishGetStatus(dgs)) = status.raw.response {
                    let down = dgs.downlink_throughput_bps / 1_000_000.0;
                    let up   = dgs.uplink_throughput_bps   / 1_000_000.0;
                    println!("downlink: {:.2} Mb/s | uplink: {:.2} Mb/s", down, up);
                }
            }
            Err(err) => {
                eprintln!("Stream error: {}", err);
            }
        }
    }

    Ok(())
}
