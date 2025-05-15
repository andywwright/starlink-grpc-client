use starlink_grpc_client::client::DishClient;
use starlink_grpc_client::space_x::api::device::response::Response as ResponseOneof;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Connect
    let mut client = DishClient::connect("http://dishy.starlink.com:9200").await?;

    // 2. Use the non-logging stream
    let mut stream = client.stream_status().await?;

    println!("⇆ Streaming throughputs (Mb/s)… Ctrl+C to quit");
    while let Some(item) = stream.next().await {
        match item {
            Ok(status) => {
                if let Some(resp) = status.raw.response {
                    if let ResponseOneof::DishGetStatus(dgs) = resp {
                        let down = dgs.downlink_throughput_bps / 1_000_000.0;
                        let up   = dgs.uplink_throughput_bps   / 1_000_000.0;
                        println!("downlink: {:.2} Mb/s | uplink: {:.2} Mb/s", down, up);
                    }
                }
            }
            Err(e) => eprintln!("Stream error: {}", e),
        }
    }

    Ok(())
}
