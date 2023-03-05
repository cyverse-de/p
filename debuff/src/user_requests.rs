// @generated
/// *
/// A request for user information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLookupRequest {
    /// Whether to include user logins in the response.
    #[prost(bool, tag="5")]
    pub include_logins: bool,
    /// Whether to include user preferences in the response.
    #[prost(bool, tag="6")]
    pub include_preferences: bool,
    /// Whether to include saved searches in the response.
    #[prost(bool, tag="7")]
    pub include_saved_searches: bool,
    /// Paging limit.
    #[prost(uint32, tag="8")]
    pub login_limit: u32,
    /// Paging offset
    #[prost(uint32, tag="9")]
    pub login_offset: u32,
    /// Contains telemetry information
    #[prost(message, optional, tag="10")]
    pub header: ::core::option::Option<super::header::Header>,
    /// How to uniquely identify the user being looked up.
    #[prost(oneof="user_lookup_request::LookupIds", tags="1, 2, 3")]
    pub lookup_ids: ::core::option::Option<user_lookup_request::LookupIds>,
}
/// Nested message and enum types in `UserLookupRequest`.
pub mod user_lookup_request {
    /// How to uniquely identify the user being looked up.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LookupIds {
        #[prost(string, tag="1")]
        Username(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        UserId(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        AnalysisId(::prost::alloc::string::String),
    }
}
include!("user_requests.serde.rs");
// @@protoc_insertion_point(module)
