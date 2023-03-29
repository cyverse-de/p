// @generated
/// *
/// An app in the system.
///
/// An app is a collection of one or more tools that run in an environment to produce outputs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct App {
    /// The UUID for the app.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The name of the app.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The description of the app.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// The wiki URL of the app.
    #[prost(string, tag="4")]
    pub wiki_url: ::prost::alloc::string::String,
}
/// *
/// Information about when something was integrated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegrationData {
    /// The UUID of the integration data.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The name of the person that integrated stuff.
    #[prost(string, tag="2")]
    pub integrator_name: ::prost::alloc::string::String,
    /// The email of the person that integrated stuff.
    #[prost(string, tag="3")]
    pub integrator_email: ::prost::alloc::string::String,
    /// The user information of the integrator.
    #[prost(message, optional, tag="4")]
    pub user: ::core::option::Option<super::user::User>,
}
/// *
/// App version information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppVersion {
    /// The UUID for the app version information.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The UUID of the app the version information applies to.
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    /// The string representation of the version.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// The ordering of the version.
    #[prost(int64, tag="4")]
    pub version_order: i64,
    /// Whether the version is deleted.
    #[prost(bool, tag="5")]
    pub deleted: bool,
    /// Whether the version is disabled.
    #[prost(bool, tag="6")]
    pub disabled: bool,
    /// Integration information for this version.
    #[prost(message, optional, tag="7")]
    pub integration: ::core::option::Option<IntegrationData>,
    /// The date the version was integrated.
    #[prost(message, optional, tag="8")]
    pub integration_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The date the app version was last edited.
    #[prost(message, optional, tag="9")]
    pub edited_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
include!("apps.serde.rs");
// @@protoc_insertion_point(module)
