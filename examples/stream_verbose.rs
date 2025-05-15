use starlink_grpc_client::client::DishClient;
use starlink_grpc_client::space_x::api::device::response::Response as ResponseOneof;
use tokio::time::{interval, Instant, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Connect
    let mut client = DishClient::connect("http://dishy.starlink.com:9200").await?;

    // 2) Set up a 1-second ticker
    let mut ticker = interval(Duration::from_secs(1));

    println!("⇆ Verbose polling: throughputs + latency… Ctrl+C to quit");

    loop {
        // wait until the next tick
        ticker.tick().await;

        // mark start
        let start = Instant::now();
        // perform a single RPC
        let status = client.get_status().await?;
        // compute latency
        let latency = Instant::now().duration_since(start);

        // unwrap and print
        if let Some(ResponseOneof::DishGetStatus(dgs)) = status.raw.response {
            let down_mbps = dgs.downlink_throughput_bps / 1_000_000.0;
            let up_mbps   = dgs.uplink_throughput_bps   / 1_000_000.0;
            println!(
                "down: {:.2} Mb/s | up: {:.2} Mb/s | rtt: {} ms", 
                down_mbps,
                up_mbps,
                latency.as_millis()
            );
        }
    }
}
