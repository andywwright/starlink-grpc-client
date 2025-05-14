use starlink_client::client::DishClient;

#[tokio::main]
async fn main() {
    let mut client = DishClient::connect("http://dishy.starlink.com:9200")
        .await
        .expect("Failed to connect to Dish");

    let status = client.get_status()
        .await
        .expect("Failed to fetch dish status");

    println!("{:#?}", status);
}
