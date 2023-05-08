// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// *
/// Information about a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceInfo {
    /// A brief description of the service.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    /// The name of the service.
    #[prost(string, tag="2")]
    pub service: ::prost::alloc::string::String,
    /// The service's version number.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
}
/// *
/// An incoming resource type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceTypeIn {
    /// The name of the resource type.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A brief description of the resource type.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
/// *
/// An outgoing resource type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceTypeOut {
    /// The resource type identifier.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The name of the resource type.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// A brief description of the resource type.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
/// *
/// A list of resource types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceTypesOut {
    /// The list of resource types.
    #[prost(message, repeated, tag="1")]
    pub resource_types: ::prost::alloc::vec::Vec<ResourceTypeOut>,
}
/// *
/// An incoming resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceIn {
    /// The resource name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The resource type name.
    #[prost(string, tag="2")]
    pub resource_type: ::prost::alloc::string::String,
}
/// *
/// A modification to a resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUpdate {
    /// The new resource name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// *
/// An outgoing resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceOut {
    /// The resource identifier.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The resource name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The resource type name.
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
}
/// *
/// A list of resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcesOut {
    /// The list of resources.
    #[prost(message, repeated, tag="1")]
    pub resources: ::prost::alloc::vec::Vec<ResourceOut>,
}
/// *
/// An incoming subject.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectIn {
    /// The external subject identifier.
    #[prost(string, tag="1")]
    pub subject_id: ::prost::alloc::string::String,
    /// The subject type.
    #[prost(string, tag="2")]
    pub subject_type: ::prost::alloc::string::String,
}
/// *
/// A list of incoming subjects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectsIn {
    /// The list of subjects.
    #[prost(message, repeated, tag="1")]
    pub subjects: ::prost::alloc::vec::Vec<SubjectIn>,
}
/// *
/// An outgoing subject.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectOut {
    /// The internal subject id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The external subject id.
    #[prost(string, tag="2")]
    pub subject_id: ::prost::alloc::string::String,
    /// The subject type.
    #[prost(string, tag="3")]
    pub subject_type: ::prost::alloc::string::String,
    /// The subject source ID.
    #[prost(string, tag="4")]
    pub subject_source_id: ::prost::alloc::string::String,
}
/// *
/// A list of outgoing subjects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectsOut {
    /// The list of subjects.
    #[prost(message, repeated, tag="1")]
    pub subjects: ::prost::alloc::vec::Vec<SubjectOut>,
}
/// *
/// Information for granting permission to a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionGrantRequest {
    /// The incoming subject.
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<SubjectIn>,
    /// The incoming resource.
    #[prost(message, optional, tag="2")]
    pub resource: ::core::option::Option<ResourceIn>,
    /// The permission level.
    #[prost(enumeration="PermissionLevel", tag="3")]
    pub permission_level: i32,
}
/// *
/// Specifies the permission level to assign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionPutRequest {
    #[prost(enumeration="PermissionLevel", tag="1")]
    pub permission_level: i32,
}
/// *
/// Information about permissions granted to a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    /// The permission id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The outgoing subject.
    #[prost(message, optional, tag="2")]
    pub subject: ::core::option::Option<SubjectOut>,
    /// The outgoing resource.
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<ResourceOut>,
    /// The permission level.
    #[prost(enumeration="PermissionLevel", tag="4")]
    pub permission_level: i32,
}
/// *
/// A list of matching permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionList {
    /// A list of matching permissions.
    #[prost(message, repeated, tag="1")]
    pub permissions: ::prost::alloc::vec::Vec<Permission>,
}
/// *
/// Abbreviated information about permissions granted to a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbbreviatedPermission {
    /// The permission identifier.
    #[prost(string, tag="1")]
    pub permission_id: ::prost::alloc::string::String,
    /// The name of the resource.
    #[prost(string, tag="2")]
    pub resource_name: ::prost::alloc::string::String,
    /// The type of the resource.
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
    /// The permission level.
    #[prost(enumeration="PermissionLevel", tag="4")]
    pub permission_level: i32,
}
/// *
/// A list of abbreviated permissions granted to a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbbreviatedPermissionList {
    /// The list of permissions.
    #[prost(message, repeated, tag="1")]
    pub permissions: ::prost::alloc::vec::Vec<AbbreviatedPermission>,
}
/// *
/// The subject type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubjectType {
    User = 0,
    Group = 1,
}
impl SubjectType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubjectType::User => "user",
            SubjectType::Group => "group",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "user" => Some(Self::User),
            "group" => Some(Self::Group),
            _ => None,
        }
    }
}
/// *
/// A permission level name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PermissionLevel {
    Read = 0,
    Admin = 1,
    Write = 2,
    Own = 3,
}
impl PermissionLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PermissionLevel::Read => "read",
            PermissionLevel::Admin => "admin",
            PermissionLevel::Write => "write",
            PermissionLevel::Own => "own",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "read" => Some(Self::Read),
            "admin" => Some(Self::Admin),
            "write" => Some(Self::Write),
            "own" => Some(Self::Own),
            _ => None,
        }
    }
}
include!("groups.serde.rs");
// @@protoc_insertion_point(module)
