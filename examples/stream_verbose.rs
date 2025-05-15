// examples/stream_verbose.rs
use starlink_grpc_client::client::DishClient;
use starlink_grpc_client::space_x::api::device::response::Response as ResponseOneof;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect and acquire the verbose streaming API
    let mut client = DishClient::connect("http://dishy.starlink.com:9200").await?;
    let mut stream = client.stream_status_logged().await?;

    println!("⇆ Verbose streaming: throughputs (Mb/s) with outbound logs and RTT… Ctrl+C to quit");

    while let Some(item) = stream.next().await {
        match item {
            Ok((status, rtt)) => {
                if let Some(ResponseOneof::DishGetStatus(dgs)) = status.raw.response {
                    let down = dgs.downlink_throughput_bps / 1_000_000.0;
                    let up   = dgs.uplink_throughput_bps   / 1_000_000.0;
                    println!(
                        "down: {:.2} Mb/s | up: {:.2} Mb/s | rtt: {} ms",
                        down,
                        up,
                        rtt.as_millis()
                    );
                }
            }
            Err(e) => eprintln!("Stream error: {}", e),
        }
    }

    Ok(())
}
