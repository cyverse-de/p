// @generated
/// *
/// A user's information.
#[derive(validator::Validate)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    #[validate(custom = "crate::custom_validator::validate_uuid")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub preferences: ::core::option::Option<user::Preferences>,
    #[prost(message, repeated, tag="4")]
    pub logins: ::prost::alloc::vec::Vec<user::Login>,
    #[prost(uint32, tag="7")]
    pub login_count: u32,
    #[prost(message, optional, tag="8")]
    pub saved_searches: ::core::option::Option<user::SavedSearches>,
    #[prost(message, optional, tag="9")]
    pub header: ::core::option::Option<super::header::Header>,
    #[prost(message, optional, tag="10")]
    pub error: ::core::option::Option<super::svcerror::ServiceError>,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[derive(validator::Validate)]
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Preferences {
        #[prost(string, tag="1")]
        #[validate(custom = "crate::custom_validator::validate_uuid")]
        pub uuid: ::prost::alloc::string::String,
        /// should come across as JSON.
        #[prost(string, tag="2")]
        pub preferences: ::prost::alloc::string::String,
    }
    #[derive(validator::Validate)]
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Login {
        #[prost(string, tag="1")]
        #[validate(custom = "crate::custom_validator::validate_uuid")]
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
    #[derive(validator::Validate)]
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SavedSearches {
        #[prost(string, tag="1")]
        #[validate(custom = "crate::custom_validator::validate_uuid")]
        pub uuid: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub saved_searches: ::prost::alloc::string::String,
    }
}
include!("user.serde.rs");
// @@protoc_insertion_point(module)
