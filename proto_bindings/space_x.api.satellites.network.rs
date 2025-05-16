#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UtDisablementCode {
    UnknownState = 0,
    Okay = 1,
    NoActiveAccount = 2,
    TooFarFromServiceAddress = 3,
    InOcean = 4,
    BlockedCountry = 6,
    DataOverageSandboxPolicy = 7,
    CellIsDisabled = 8,
    RoamRestricted = 10,
    UnknownLocation = 11,
    AccountDisabled = 12,
    UnsupportedVersion = 13,
    MovingTooFastForPolicy = 14,
    UnderAviationFlyoverLimits = 15,
}
impl UtDisablementCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UtDisablementCode::UnknownState => "UNKNOWN_STATE",
            UtDisablementCode::Okay => "OKAY",
            UtDisablementCode::NoActiveAccount => "NO_ACTIVE_ACCOUNT",
            UtDisablementCode::TooFarFromServiceAddress => "TOO_FAR_FROM_SERVICE_ADDRESS",
            UtDisablementCode::InOcean => "IN_OCEAN",
            UtDisablementCode::BlockedCountry => "BLOCKED_COUNTRY",
            UtDisablementCode::DataOverageSandboxPolicy => "DATA_OVERAGE_SANDBOX_POLICY",
            UtDisablementCode::CellIsDisabled => "CELL_IS_DISABLED",
            UtDisablementCode::RoamRestricted => "ROAM_RESTRICTED",
            UtDisablementCode::UnknownLocation => "UNKNOWN_LOCATION",
            UtDisablementCode::AccountDisabled => "ACCOUNT_DISABLED",
            UtDisablementCode::UnsupportedVersion => "UNSUPPORTED_VERSION",
            UtDisablementCode::MovingTooFastForPolicy => "MOVING_TOO_FAST_FOR_POLICY",
            UtDisablementCode::UnderAviationFlyoverLimits => {
                "UNDER_AVIATION_FLYOVER_LIMITS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_STATE" => Some(Self::UnknownState),
            "OKAY" => Some(Self::Okay),
            "NO_ACTIVE_ACCOUNT" => Some(Self::NoActiveAccount),
            "TOO_FAR_FROM_SERVICE_ADDRESS" => Some(Self::TooFarFromServiceAddress),
            "IN_OCEAN" => Some(Self::InOcean),
            "BLOCKED_COUNTRY" => Some(Self::BlockedCountry),
            "DATA_OVERAGE_SANDBOX_POLICY" => Some(Self::DataOverageSandboxPolicy),
            "CELL_IS_DISABLED" => Some(Self::CellIsDisabled),
            "ROAM_RESTRICTED" => Some(Self::RoamRestricted),
            "UNKNOWN_LOCATION" => Some(Self::UnknownLocation),
            "ACCOUNT_DISABLED" => Some(Self::AccountDisabled),
            "UNSUPPORTED_VERSION" => Some(Self::UnsupportedVersion),
            "MOVING_TOO_FAST_FOR_POLICY" => Some(Self::MovingTooFastForPolicy),
            "UNDER_AVIATION_FLYOVER_LIMITS" => Some(Self::UnderAviationFlyoverLimits),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountDisablementReason {
    NoRestriction = 0,
    KnowYourCustomerRequired = 1,
}
impl AccountDisablementReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountDisablementReason::NoRestriction => "NO_RESTRICTION",
            AccountDisablementReason::KnowYourCustomerRequired => {
                "KNOW_YOUR_CUSTOMER_REQUIRED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_RESTRICTION" => Some(Self::NoRestriction),
            "KNOW_YOUR_CUSTOMER_REQUIRED" => Some(Self::KnowYourCustomerRequired),
            _ => None,
        }
    }
}
