// @generated
// This file is @generated by prost-build.
/// *
/// A tool integrated into the system.
///
/// A tool is part of an app and can run in an execution environment.
/// Mostly correllates to the 'tools' table in the 'de' database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tool {
    /// The UUID for the tool.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The name of the tool.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The version of the tool.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// Attribution information for the tool.
    #[prost(string, tag="4")]
    pub attribution: ::prost::alloc::string::String,
    /// The description of the tool.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// The time limit the tool is allowed to run for by default. Unit is seconds.
    #[prost(int32, tag="6")]
    pub time_limit_seconds: i32,
    /// Whether the tool is restricted.
    #[prost(bool, tag="7")]
    pub restricted: bool,
    /// Whether the tool is interactive (i.e. whether it is intended for use in VICE).
    #[prost(bool, tag="8")]
    pub interactive: bool,
    /// Whether the tool requires a GPU.
    #[prost(bool, tag="9")]
    pub gpu_enabled: bool,
    /// Integration data associated with the tool.
    #[prost(message, optional, tag="10")]
    pub integration_data: ::core::option::Option<super::apps::IntegrationData>,
    /// The container image to use when running the tool.
    #[prost(message, optional, tag="11")]
    pub container_image: ::core::option::Option<super::containers::Image>,
}
include!("tools.serde.rs");
// @@protoc_insertion_point(module)
