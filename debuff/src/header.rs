// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(map="string, message", tag="1")]
    pub map: ::std::collections::HashMap<::prost::alloc::string::String, header::Value>,
}
/// Nested message and enum types in `Header`.
pub mod header {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Value {
        #[prost(string, repeated, tag="1")]
        pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
include!("header.serde.rs");
// @@protoc_insertion_point(module)
