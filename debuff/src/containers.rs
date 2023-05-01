// @generated
/// *
/// A container volume. Used for mapping host paths into a container.
/// 
/// Correlates to the 'container_volumes' table in the 'de' database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// The absolute path to a file/directory on the container host/k8s node.
    #[prost(string, tag="1")]
    pub host_path: ::prost::alloc::string::String,
    /// The path to mount the host path to inside of the container.
    #[prost(string, tag="2")]
    pub container_path: ::prost::alloc::string::String,
    /// Whether the mount point in the container should be read only.
    #[prost(bool, tag="3")]
    pub read_only: bool,
    /// The mode the mount point should have once mounted.
    #[prost(string, tag="4")]
    pub mode: ::prost::alloc::string::String,
}
/// *
/// A port the container exposes.
///
/// Correlates to the 'container_ports' table in the 'de' database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Port {
    /// The port on the host that the container port should be mapped to. Usually unset.
    #[prost(int32, tag="1")]
    pub host_port: i32,
    /// The port the contained process needs to have open.
    #[prost(int32, tag="2")]
    pub container_port: i32,
    /// Whether to bind the container port to the host port. Normally left to false/null/None.
    #[prost(bool, tag="3")]
    pub bind_to_host: bool,
}
/// *
/// A host device that the container needs access to.
///
/// Correlates to the 'container_devices' table in the 'de' database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// The path to the device on the host.
    #[prost(string, tag="1")]
    pub host_path: ::prost::alloc::string::String,
    /// The path to the device in the container.
    #[prost(string, tag="2")]
    pub container_path: ::prost::alloc::string::String,
    /// The permissions needed to mount the device.
    #[prost(string, tag="3")]
    pub cgroup_permissions: ::prost::alloc::string::String,
}
/// *
/// Another container from which the container should mount volumes.
///
/// Correlates to the 'container_volumes_from' table in the 'de' database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumesFrom {
    /// The image tag for the mountee container.
    #[prost(string, tag="1")]
    pub tag: ::prost::alloc::string::String,
    /// The name of the mountee container.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Authentication information needed to pull the mountee container image.
    #[prost(string, tag="3")]
    pub auth: ::prost::alloc::string::String,
    /// The prefix that should be given to the name of the container.
    #[prost(string, tag="4")]
    pub name_prefix: ::prost::alloc::string::String,
    /// The URL to the mountee container image.
    #[prost(string, tag="5")]
    pub url: ::prost::alloc::string::String,
    /// The path inside the mountee container that should be mounted.
    #[prost(string, tag="6")]
    pub host_path: ::prost::alloc::string::String,
    /// The path to the mount point inside the mounting container.
    #[prost(string, tag="7")]
    pub container_path: ::prost::alloc::string::String,
    /// Whether the mount point is read only.
    #[prost(bool, tag="8")]
    pub read_only: bool,
}
/// *
/// Information about a container image.
///
/// Correlates to the 'container_images' table in the 'de' database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// The UUID of the container image.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The name of the container image.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The container image's tag.
    #[prost(string, tag="3")]
    pub tag: ::prost::alloc::string::String,
    /// Authentication information needed to pull the image.
    #[prost(string, tag="4")]
    pub auth: ::prost::alloc::string::String,
    /// The URL for the image.
    #[prost(string, tag="5")]
    pub url: ::prost::alloc::string::String,
    /// The path to the image in OSG. Might be blank.
    #[prost(string, tag="6")]
    pub osg_image_path: ::prost::alloc::string::String,
}
/// *
/// Contains information needed to launch an app in VICE.
/// 
/// Corresponds roughly to to the 'interactive_apps_proxy_settings' table in the 'de' database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractiveApps {
    /// The image containing the proxy application that requests to a VICE app flow through.
    #[prost(string, tag="1")]
    pub proxy_image: ::prost::alloc::string::String,
    /// The name of the proxy container.
    #[prost(string, tag="2")]
    pub proxy_name: ::prost::alloc::string::String,
    /// The URL the proxy will redirect users to when necessary.
    #[prost(string, tag="3")]
    pub frontend_url: ::prost::alloc::string::String,
    /// The URL to the CAS instance needed for authentication.
    #[prost(string, tag="4")]
    pub cas_url: ::prost::alloc::string::String,
    /// The URL to the CAS authentication validation.
    #[prost(string, tag="5")]
    pub cas_validate: ::prost::alloc::string::String,
    /// The path to the SSL cert in the container.
    #[prost(string, tag="6")]
    pub ssl_cert_path: ::prost::alloc::string::String,
    /// The path to the SSL key in the container.
    #[prost(string, tag="7")]
    pub ssl_key_path: ::prost::alloc::string::String,
    /// Unused.
    #[prost(string, tag="8")]
    pub websocket_path: ::prost::alloc::string::String,
    /// Unused.
    #[prost(string, tag="9")]
    pub websocket_port: ::prost::alloc::string::String,
    /// Unused.
    #[prost(string, tag="10")]
    pub websocket_proto: ::prost::alloc::string::String,
    /// Unused.
    #[prost(string, tag="11")]
    pub backend_url: ::prost::alloc::string::String,
}
/// *
/// A representation of a container for an analysis.
/// 
/// Correlates to the 'container_settings' table in the 'de' database..
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    #[prost(message, repeated, tag="3")]
    pub devices: ::prost::alloc::vec::Vec<Device>,
    #[prost(message, repeated, tag="4")]
    pub volumes_from: ::prost::alloc::vec::Vec<VolumesFrom>,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub network_mode: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub cpu_shares: i64,
    #[prost(message, optional, tag="8")]
    pub interactive_apps: ::core::option::Option<InteractiveApps>,
    #[prost(int64, tag="9")]
    pub memory_limit: i64,
    #[prost(int64, tag="10")]
    pub min_memory_limit: i64,
    #[prost(float, tag="11")]
    pub max_cpu_cores: f32,
    #[prost(float, tag="12")]
    pub min_cpu_cores: f32,
    #[prost(int64, tag="13")]
    pub min_disk_space: i64,
    #[prost(int64, tag="14")]
    pub pids_limit: i64,
    #[prost(message, optional, tag="15")]
    pub image: ::core::option::Option<Image>,
    #[prost(string, tag="16")]
    pub entry_point: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub working_dir: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="18")]
    pub ports: ::prost::alloc::vec::Vec<Port>,
    #[prost(bool, tag="19")]
    pub skip_tmp_mount: bool,
    #[prost(int32, tag="20")]
    pub uid: i32,
}
include!("containers.serde.rs");
// @@protoc_insertion_point(module)
