// @generated
/// *
/// Request a resource by the username of a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByUsername {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
/// *
/// Request a resource by the user ID of a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByUserId {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
/// *
/// Send a message that does not request any parameters. Common for triggering
/// side-effects or for retrieving lists of resources as an administrator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoParams {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
}
/// *
/// Request a resource by its UUID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByUuid {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
    /// The UUID of the resource being requested.
    #[prost(string, tag="2")]
    pub uuid: ::prost::alloc::string::String,
}
/// *
/// Request a resource by its UUID and a username. Useful in situations where a
/// user's ability to access a resource needs to be checked as part of the 
/// request handler logic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByUuidAndUsername {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
    /// The UUID of the resource being requested
    #[prost(string, tag="2")]
    pub uuid: ::prost::alloc::string::String,
    /// The username associated with the request.
    #[prost(string, tag="3")]
    pub username: ::prost::alloc::string::String,
}
/// *
/// Request a resource by its UUID and a user's UUID. Useful when the user's 
/// access to the resource must be verified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByUuidAndUserId {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
    /// The UUID of the resource being requested
    #[prost(string, tag="2")]
    pub uuid: ::prost::alloc::string::String,
    /// The user ID of the user associated with the request.
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
}
/// *
/// Request that two resources be associated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateByUuiDs {
    /// Contains telemetry information.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<super::header::Header>,
    /// The UUID of the parent/owner/primary resource.
    #[prost(string, tag="2")]
    pub parent_uuid: ::prost::alloc::string::String,
    /// The UUID of the child/object/secondary resource.
    #[prost(string, tag="3")]
    pub child_uuid: ::prost::alloc::string::String,
}
include!("requests.serde.rs");
// @@protoc_insertion_point(module)
