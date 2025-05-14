use futures::{stream, Stream, TryStreamExt};
use tonic::{transport::Channel, Request};

use std::time::Duration;
use tokio::time::sleep;

use crate::error::DishError;
use crate::models::DishStatus;

// One single include! in lib.rs, so none here
use crate::space_x::api::device::device_client::DeviceClient as GrpcDeviceClient;
use crate::space_x::api::device::{
    Request as RawRequest,
    GetStatusRequest,
    ToDevice,
    FromDevice,
};
use crate::space_x::api::device::request::Request as RequestOneof;
use crate::space_x::api::device::to_device::Message as ToDeviceMessage;
use crate::space_x::api::device::from_device::Message as FromDeviceMessage;

pub struct DishClient {
    inner: GrpcDeviceClient<Channel>,
}

impl DishClient {
    /// Connect to the Dish’s gRPC endpoint.
    pub async fn connect(endpoint: &str) -> Result<Self, DishError> {
        let inner = GrpcDeviceClient::connect(endpoint.to_string())
            .await
            .map_err(DishError::ConnectionError)?;
        Ok(Self { inner })
    }

    /// Unary RPC: fetch a single status snapshot.
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

    /// Opens a long-lived stream: polls `getStatus` every second.
    /// Yields a `DishStatus` each time the Dish replies.
    pub async fn stream_status(
        &mut self,
    ) -> Result<impl Stream<Item = Result<DishStatus, DishError>>, DishError> {
        // Build a client stream that emits a GetStatusRequest every second.
        let outbound = stream::unfold((), |_| async {
            sleep(Duration::from_secs(1)).await;
            let msg = ToDevice {
                message: Some(ToDeviceMessage::Request(RawRequest {
                    id: 1,
                    epoch_id: 0,
                    target_id: "".into(),
                    request: Some(RequestOneof::GetStatus(GetStatusRequest {})),
                })),
            };
            Some((msg, ())) // (item, next state)
        });

        // Initiate the bidirectional Stream RPC
        let response_stream = self
            .inner
            .stream(Request::new(outbound))
            .await
            .map_err(DishError::RpcError)?
            .into_inner(); // tonic::Streaming<FromDevice>

        // Map each FromDevice::Response into your DishStatus wrapper
        let mapped = response_stream
            .map_ok(|from| match from.message.expect("missing message") {
                FromDeviceMessage::Response(raw) => DishStatus { raw },
                _ => unreachable!("Only `Response` messages are expected here"),
            })
            .map_err(DishError::RpcError);

        Ok(mapped)
    }
}
