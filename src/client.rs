use futures::{stream::{unfold}, Stream, TryStreamExt};
use tonic::{transport::Channel, Request};

use crate::error::DishError;
use crate::models::DishStatus;

use crate::space_x::api::device::device_client::DeviceClient as GrpcDeviceClient;
use crate::space_x::api::device::{
    Request as RawRequest,
    GetStatusRequest,
    ToDevice,
};
use crate::space_x::api::device::request::Request as RequestOneof;
use crate::space_x::api::device::to_device::Message as ToDeviceMessage;
use crate::space_x::api::device::from_device::Message as FromDeviceMessage;

pub struct DishClient {
    inner: GrpcDeviceClient<Channel>,
}

impl DishClient {
    /// Connects to the Dish's gRPC endpoint.
    pub async fn connect(endpoint: &str) -> Result<Self, DishError> {
        let inner = GrpcDeviceClient::connect(endpoint.to_string())
            .await
            .map_err(DishError::ConnectionError)?;
        Ok(Self { inner })
    }

    /// Performs a one-off status query.
    pub async fn get_status(&mut self) -> Result<DishStatus, DishError> {
        let raw_req = RawRequest {
            id: 0,
            epoch_id: 0,
            target_id: "".into(),
            request: Some(RequestOneof::GetStatus(GetStatusRequest {})),
        };

        let response = self
            .inner
            .handle(raw_req)
            .await
            .map_err(DishError::RpcError)?
            .into_inner();

        Ok(DishStatus { raw: response })
    }

    /// Opens a long-lived stream: polls `getStatus` every second,
    /// yielding each `DishStatus`.
    pub async fn stream_status(
        &mut self,
    ) -> Result<impl Stream<Item = Result<DishStatus, DishError>>, DishError> {
        self.inner_stream(false).await
    }

    /// Opens a stream with outbound logging enabled (for debugging).
    pub async fn stream_status_logged(
        &mut self,
    ) -> Result<impl Stream<Item = Result<DishStatus, DishError>>, DishError> {
        self.inner_stream(true).await
    }

    /// Internal helper to build and execute the polling stream.
    async fn inner_stream(
        &mut self,
        log_outbound: bool,
    ) -> Result<impl Stream<Item = Result<DishStatus, DishError>>, DishError> {
        // Build a polling stream that emits a GetStatusRequest every second.
        let outbound = unfold((), move |()| async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let msg = ToDevice {
                message: Some(ToDeviceMessage::Request(RawRequest {
                    id: 1,
                    epoch_id: 0,
                    target_id: "".into(),
                    request: Some(RequestOneof::GetStatus(GetStatusRequest {})),
                })),
            };
            if log_outbound {
                println!("→ Outbound request: GetStatusRequest");
            }
            Some((msg, ()))
        });

        // Initiate the bidirectional Stream RPC
        let response = self
            .inner
            .stream(Request::new(outbound))
            .await
            .map_err(DishError::RpcError)?;

        // Map each FromDevice::Response into your DishStatus wrapper
        let stream = response
            .into_inner()
            .map_ok(|msg| match msg.message.expect("missing message") {
                FromDeviceMessage::Response(raw) => DishStatus { raw },
                _ => unreachable!("Only Response variants expected"),
            })
            .map_err(DishError::RpcError);

        Ok(stream)
    }
}
