// @generated
/// *
/// An error returned by a request handler.
#[derive(validator::Validate)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceError {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
    /// The numeric error code from the error code enum.
    #[prost(enumeration="ErrorCode", tag="2")]
    pub error_code: i32,
    /// The status code for the error.
    #[prost(int32, tag="3")]
    pub status_code: i32,
    /// The error's message.
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
}
/// *
/// The types of errors that can be retuned by message handlers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    /// Default value for the error code. Don't set the error code to this. Use Unspecified if tempted.
    Unset = 0,
    /// An error occurred, but the kind wasn't specified or included in the list.
    Unspecified = 1,
    /// Internal error.
    Internal = 2,
    /// The requested resource wasn't found.
    NotFound = 3,
    /// The request was bad/wrong is some way.
    BadRequest = 4,
    /// A failure to marshal a response.
    MarshalFailure = 5,
    /// A failure to unmarshal a request.
    UnmarshalFailure = 6,
    /// A parameter is missing.
    ParameterMissing = 7,
    /// / A parameter is invalid.
    ParameterInvalid = 8,
}
impl ErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorCode::Unset => "UNSET",
            ErrorCode::Unspecified => "UNSPECIFIED",
            ErrorCode::Internal => "INTERNAL",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::BadRequest => "BAD_REQUEST",
            ErrorCode::MarshalFailure => "MARSHAL_FAILURE",
            ErrorCode::UnmarshalFailure => "UNMARSHAL_FAILURE",
            ErrorCode::ParameterMissing => "PARAMETER_MISSING",
            ErrorCode::ParameterInvalid => "PARAMETER_INVALID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSET" => Some(Self::Unset),
            "UNSPECIFIED" => Some(Self::Unspecified),
            "INTERNAL" => Some(Self::Internal),
            "NOT_FOUND" => Some(Self::NotFound),
            "BAD_REQUEST" => Some(Self::BadRequest),
            "MARSHAL_FAILURE" => Some(Self::MarshalFailure),
            "UNMARSHAL_FAILURE" => Some(Self::UnmarshalFailure),
            "PARAMETER_MISSING" => Some(Self::ParameterMissing),
            "PARAMETER_INVALID" => Some(Self::ParameterInvalid),
            _ => None,
        }
    }
}
include!("svcerror.serde.rs");
// @@protoc_insertion_point(module)
