// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Preferences {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// should come across as JSON.
    #[prost(string, tag="2")]
    pub preferences: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Login {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ip_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub login_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub logout_time: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedSearches {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub saved_searches: ::prost::alloc::string::String,
}
/// *
/// A user's information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
include!("user.serde.rs");
// @@protoc_insertion_point(module)
