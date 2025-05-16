#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtPoPLinkReport {
    #[prost(message, optional, tag = "1")]
    pub slot_timestamp: ::core::option::Option<super::common::TimestampInfo>,
    #[prost(uint32, tag = "2")]
    pub pop_id: u32,
    #[prost(uint32, tag = "3")]
    pub pop_rack_id: u32,
    #[prost(message, repeated, tag = "4")]
    pub stats: ::prost::alloc::vec::Vec<UtPoPLinkStats>,
    #[prost(string, tag = "5")]
    pub pop_version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtPoPLinkStats {
    #[prost(message, optional, tag = "1")]
    pub measurement_timestamp: ::core::option::Option<super::common::TimestampInfo>,
    #[prost(string, tag = "2")]
    pub ut_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub pop_rx_sdu_cnt: i64,
    #[prost(int64, tag = "4")]
    pub sdu_loss_cnt: i64,
    #[prost(uint64, tag = "5")]
    pub uplink_bytes_last_15s: u64,
    #[prost(uint64, tag = "6")]
    pub downlink_bytes_last_15s: u64,
    #[prost(uint64, tag = "7")]
    pub uplink_cplane_acl_other_violations_last_15s: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RateLimitReason {
    Unknown = 0,
    NoLimit = 1,
    PolicyLimit = 2,
    UserCustomLimit = 3,
    OverageLimit = 5,
}
impl RateLimitReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RateLimitReason::Unknown => "UNKNOWN",
            RateLimitReason::NoLimit => "NO_LIMIT",
            RateLimitReason::PolicyLimit => "POLICY_LIMIT",
            RateLimitReason::UserCustomLimit => "USER_CUSTOM_LIMIT",
            RateLimitReason::OverageLimit => "OVERAGE_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "NO_LIMIT" => Some(Self::NoLimit),
            "POLICY_LIMIT" => Some(Self::PolicyLimit),
            "USER_CUSTOM_LIMIT" => Some(Self::UserCustomLimit),
            "OVERAGE_LIMIT" => Some(Self::OverageLimit),
            _ => None,
        }
    }
}
