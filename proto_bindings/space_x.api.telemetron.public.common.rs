#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampInfo {
    #[prost(enumeration = "Epoch", tag = "1")]
    pub epoch: i32,
    #[prost(int64, tag = "2")]
    pub nanoseconds: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Epoch {
    Unix = 0,
    Gps = 1,
}
impl Epoch {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Epoch::Unix => "UNIX",
            Epoch::Gps => "GPS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNIX" => Some(Self::Unix),
            "GPS" => Some(Self::Gps),
            _ => None,
        }
    }
}
