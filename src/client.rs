use tonic::transport::Channel;
use crate::error::DishError;
use crate::models::DishStatus;

use crate::models::space_x::api::device::device_client::DeviceClient;
use crate::models::space_x::api::device::{Request, GetStatusRequest};
use crate::models::space_x::api::device::request::Request as RequestOneof;

/// Client for communicating with the Starlink Dish.
pub struct DishClient {
    inner: DeviceClient<Channel>,
}

impl DishClient {
    /// Connects to the dish at the provided gRPC endpoint.
    pub async fn connect(endpoint: &str) -> Result<Self, DishError> {
        let inner = DeviceClient::connect(endpoint.to_string()).await
            .map_err(DishError::ConnectionError)?;
        Ok(Self { inner })
    }

    /// Fetches the current status of the dish.
    pub async fn get_status(&mut self) -> Result<DishStatus, DishError> {
        let request = Request {
            id: 0,
            epoch_id: 0,
            target_id: "".to_string(),
            request: Some(RequestOneof::GetStatus(GetStatusRequest {})),
        };

        let response = self.inner.handle(request).await
            .map_err(DishError::RpcError)?;

        Ok(DishStatus {
            raw: response.into_inner(),
        })
    }
}
