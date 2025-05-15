use futures::{stream, Stream, TryStreamExt};
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

    /// Opens a long‐lived stream: polls `getStatus` every second,
    /// logging only the innermost request type name.
    pub async fn stream_status(
        &mut self,
    ) -> Result<impl Stream<Item = Result<DishStatus, DishError>>, DishError> {
        // Build a client stream that emits a GetStatusRequest every second.
        let outbound = stream::unfold((), |()| async {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let msg = ToDevice {
                message: Some(ToDeviceMessage::Request(RawRequest {
                    id: 1,
                    epoch_id: 0,
                    target_id: "".into(),
                    request: Some(RequestOneof::GetStatus(GetStatusRequest {})),
                })),
            };

            // Inspect and print only the inner variant name:
            if let Some(ToDeviceMessage::Request(raw_req)) = &msg.message {
                match &raw_req.request {
                    Some(RequestOneof::GetStatus(_)) => {
                        println!("→ Outbound request: GetStatusRequest");
                    }
                    _ => {
                        println!("→ Outbound request: <other>");
                    }
                }
            }

            Some((msg, ()))
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
                _ => unreachable!("Only Response variants expected"),
            })
            .map_err(DishError::RpcError);

        Ok(mapped)
    }
}
