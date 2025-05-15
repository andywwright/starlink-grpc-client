use starlink_grpc_client::client::DishClient;
use starlink_grpc_client::space_x::api::device::response::Response as ResponseOneof;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Connect
    let mut client = DishClient::connect("http://dishy.starlink.com:9200").await?;
    
    // 2. Acquire the streaming handle (which logs outbound messages)
    let mut stream = client.stream_status().await?;
    
    println!("⇆ Streaming throughputs (Mb/s)… Ctrl+C to quit");

    // 3. Print only the two throughput fields in Mb/s
    while let Some(item) = stream.next().await {
        match item {
            Ok(status) => {
                if let Some(resp_oneof) = status.raw.response {
                    if let ResponseOneof::DishGetStatus(dgs) = resp_oneof {
                        let down_mbps = dgs.downlink_throughput_bps / 1_000_000.0;
                        let up_mbps   = dgs.uplink_throughput_bps   / 1_000_000.0;
                        println!(
                            "downlink: {:.2} Mb/s | uplink: {:.2} Mb/s",
                            down_mbps, up_mbps,
                        );
                    }
                }
            }
            Err(err) => eprintln!("Stream error: {}", err),
        }
    }

    Ok(())
}
