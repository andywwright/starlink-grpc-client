// src/client.rs
use futures::{stream::unfold, Stream, TryStreamExt};
use tonic::{transport::Channel, Request};
use std::{sync::{Arc, Mutex}, time::{Duration, Instant}, collections::VecDeque};

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

    /// Polling stream: emits one status per second without logging.
    pub async fn stream_status(
        &mut self,
    ) -> Result<impl Stream<Item = Result<DishStatus, DishError>>, DishError> {
        let outbound = unfold((), |()| async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let msg = ToDevice {
                message: Some(ToDeviceMessage::Request(RawRequest {
                    id: 1,
                    epoch_id: 0,
                    target_id: "".into(),
                    request: Some(RequestOneof::GetStatus(GetStatusRequest {})),
                })),
            };
            Some((msg, ()))
        });

        let response_stream = self
            .inner
            .stream(Request::new(outbound))
            .await
            .map_err(DishError::RpcError)?
            .into_inner();

        let mapped = response_stream
            .map_ok(|from| match from.message.expect("missing message") {
                FromDeviceMessage::Response(raw) => DishStatus { raw },
                _ => unreachable!("Only Response variants expected"),
            })
            .map_err(DishError::RpcError);

        Ok(mapped)
    }

    /// Polling stream with outbound logging and real RTT measurement.
    /// Yields `(DishStatus, Duration)` where `Duration` is measured RTT.
    pub async fn stream_status_logged(
        &mut self,
    ) -> Result<impl Stream<Item = Result<(DishStatus, Duration), DishError>>, DishError> {
        // Shared queue for send timestamps
        let times = Arc::new(Mutex::new(VecDeque::new()));
        let times_clone = times.clone();

        // Build outbound stream: send one request per second,
        // record send time, and log the outgoing message.
        let outbound = unfold((), move |()| {
            let times_clone = times_clone.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let send_time = Instant::now();
                let msg = ToDevice {
                    message: Some(ToDeviceMessage::Request(RawRequest {
                        id: 1,
                        epoch_id: 0,
                        target_id: "".into(),
                        request: Some(RequestOneof::GetStatus(GetStatusRequest {})),
                    })),
                };
                println!("→ Outbound request: GetStatusRequest");
                times_clone.lock().unwrap().push_back(send_time);
                Some((msg, ()))
            }
        });

        // Initiate the bidi stream
        let response = self
            .inner
            .stream(Request::new(outbound))
            .await
            .map_err(DishError::RpcError)?;

        // Map incoming messages into (DishStatus, Duration)
        let mapped = response
            .into_inner() // tonic::Streaming<FromDevice>
            .map_ok(move |from| {
                let recv_time = Instant::now();
                let send_time = times.lock().unwrap().pop_front().unwrap_or(recv_time);
                let rtt = recv_time.duration_since(send_time);
                let raw = match from.message.expect("missing message") {
                    FromDeviceMessage::Response(raw) => raw,
                    _ => unreachable!("Only Response expected"),
                };
                (DishStatus { raw }, rtt)
            })
            .map_err(DishError::RpcError);

        Ok(mapped)
    }
}
