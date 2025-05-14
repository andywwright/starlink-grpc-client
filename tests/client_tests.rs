use starlink_grpc_client::client::DishClient;

#[tokio::test]
async fn test_dish_client_connection_fails_on_invalid_endpoint() {
    let result = DishClient::connect("http://invalid-endpoint:1234").await;
    assert!(result.is_err(), "Expected connection to fail on invalid endpoint");
}
