use thiserror::Error;

/// Errors that can occur when interacting with the dish.
#[derive(Debug, Error)]
pub enum DishError {
    #[error("Failed to connect to Dish: {0}")]
    ConnectionError(tonic::transport::Error),
    #[error("Failed to perform gRPC call: {0}")]
    RpcError(tonic::Status),
}
