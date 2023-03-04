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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractiveApps {
    #[prost(string, tag="1")]
    pub proxy_image: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub proxy_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub frontend_url: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cas_url: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub cas_validate: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub ssl_cert_path: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub ssl_key_path: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub websocket_path: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub websocket_port: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub websocket_proto: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub backend_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag="12")]
    pub header: ::core::option::Option<Header>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub volumes: ::prost::alloc::vec::Vec<container::Volume>,
    #[prost(message, repeated, tag="3")]
    pub devices: ::prost::alloc::vec::Vec<container::Device>,
    #[prost(message, repeated, tag="4")]
    pub volumes_from: ::prost::alloc::vec::Vec<container::VolumesFrom>,
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
    pub image: ::core::option::Option<container::Image>,
    #[prost(string, tag="16")]
    pub entry_point: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub working_dir: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="18")]
    pub ports: ::prost::alloc::vec::Vec<container::Port>,
    #[prost(bool, tag="19")]
    pub skip_tmp_mount: bool,
    #[prost(int32, tag="20")]
    pub uid: i32,
    #[prost(message, optional, tag="21")]
    pub header: ::core::option::Option<Header>,
}
/// Nested message and enum types in `Container`.
pub mod container {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Volume {
        #[prost(string, tag="1")]
        pub host_path: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub container_path: ::prost::alloc::string::String,
        #[prost(bool, tag="3")]
        pub read_only: bool,
        #[prost(string, tag="4")]
        pub mode: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Port {
        #[prost(int32, tag="1")]
        pub host_port: i32,
        #[prost(int32, tag="2")]
        pub container_port: i32,
        #[prost(bool, tag="3")]
        pub bind_to_host: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Device {
        #[prost(string, tag="1")]
        pub host_path: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub container_path: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub cgroup_permissions: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VolumesFrom {
        #[prost(string, tag="1")]
        pub tag: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub auth: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub name_prefix: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub url: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub host_path: ::prost::alloc::string::String,
        #[prost(string, tag="7")]
        pub container_path: ::prost::alloc::string::String,
        #[prost(bool, tag="8")]
        pub read_only: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Image {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub tag: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub auth: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub url: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub osg_image_path: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisRecord {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub can_share: bool,
    #[prost(string, tag="6")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub batch_status: ::core::option::Option<analysis_record::BatchStatus>,
    #[prost(string, tag="9")]
    pub system_id: ::prost::alloc::string::String,
    #[prost(bool, tag="10")]
    pub app_disabled: bool,
    #[prost(bool, tag="11")]
    pub batch: bool,
    #[prost(string, tag="12")]
    pub enddate: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub startdate: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub app_description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="15")]
    pub interactive_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="16")]
    pub wiki_url: ::prost::alloc::string::String,
    #[prost(bool, tag="17")]
    pub notify: bool,
    #[prost(string, tag="18")]
    pub result_folder_id: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub app_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AnalysisRecord`.
pub mod analysis_record {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BatchStatus {
        #[prost(int64, tag="1")]
        pub total: i64,
        #[prost(int64, tag="2")]
        pub completed: i64,
        #[prost(int64, tag="3")]
        pub running: i64,
        #[prost(int64, tag="4")]
        pub submitted: i64,
    }
}
/// *
/// An error returned by a request handler.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceError {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisRecordLookupRequest {
    #[prost(message, optional, tag="5")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag="6")]
    pub requesting_user: ::prost::alloc::string::String,
    #[prost(oneof="analysis_record_lookup_request::LookupIds", tags="1, 2, 3, 4")]
    pub lookup_ids: ::core::option::Option<analysis_record_lookup_request::LookupIds>,
}
/// Nested message and enum types in `AnalysisRecordLookupRequest`.
pub mod analysis_record_lookup_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LookupIds {
        #[prost(string, tag="1")]
        AnalysisId(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        ExternalId(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        UserId(::prost::alloc::string::String),
        #[prost(string, tag="4")]
        Username(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisRecordResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag="2")]
    pub analyses: ::prost::alloc::vec::Vec<AnalysisRecord>,
    #[prost(string, tag="3")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub total: i64,
    #[prost(message, repeated, tag="5")]
    pub status_count: ::prost::alloc::vec::Vec<analysis_record_response::StatusCountRecord>,
    #[prost(message, optional, tag="6")]
    pub error: ::core::option::Option<ServiceError>,
}
/// Nested message and enum types in `AnalysisRecordResponse`.
pub mod analysis_record_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StatusCountRecord {
        #[prost(int64, tag="1")]
        pub count: i64,
        #[prost(string, tag="2")]
        pub status: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisRecordList {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, repeated, tag="2")]
    pub analyses: ::prost::alloc::vec::Vec<AnalysisRecord>,
    #[prost(message, optional, tag="7")]
    pub error: ::core::option::Option<ServiceError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtCondorExtraInfo {
    #[prost(string, tag="1")]
    pub extra_requirements: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    #[prost(message, optional, tag="1")]
    pub ht_condor: ::core::option::Option<HtCondorExtraInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMetadata {
    #[prost(string, tag="1")]
    pub attribute: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub unit: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    #[prost(message, optional, tag="1")]
    pub component: ::core::option::Option<step::Component>,
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<step::Config>,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub stdin_path: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub stdout_path: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub stderr_path: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub log_file: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="8")]
    pub environment: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// It's really annoying, but this is singular for backwards compatibility.
    #[prost(message, repeated, tag="9")]
    pub input: ::prost::alloc::vec::Vec<step::Input>,
    /// It's really annoying, but this is singular for backwards compatibility.
    #[prost(message, repeated, tag="10")]
    pub output: ::prost::alloc::vec::Vec<step::Output>,
    #[prost(message, optional, tag="11")]
    pub header: ::core::option::Option<Header>,
}
/// Nested message and enum types in `Step`.
pub mod step {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Component {
        #[prost(message, optional, tag="1")]
        pub container: ::core::option::Option<super::Container>,
        #[prost(string, tag="2")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub location: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub description: ::prost::alloc::string::String,
        #[prost(int32, tag="6")]
        pub time_limit: i32,
        #[prost(bool, tag="7")]
        pub restricted: bool,
        #[prost(bool, tag="8")]
        pub is_interactive: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Param {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub value: ::prost::alloc::string::String,
        #[prost(int32, tag="4")]
        pub order: i32,
        #[prost(string, tag="5")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub path: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Config {
        #[prost(message, repeated, tag="1")]
        pub params: ::prost::alloc::vec::Vec<Param>,
        /// Keep it singular in the JSON for backwards-compatibility.
        #[prost(message, repeated, tag="2")]
        pub inputs: ::prost::alloc::vec::Vec<Input>,
        /// Keep it singular in the JSON for backwards-compatibility.
        #[prost(message, repeated, tag="3")]
        pub outputs: ::prost::alloc::vec::Vec<Output>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub ticket: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub multiplicity: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub property: ::prost::alloc::string::String,
        #[prost(bool, tag="6")]
        pub retain: bool,
        #[prost(string, tag="7")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag="8")]
        pub value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Output {
        #[prost(string, tag="1")]
        pub multiplicity: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub property: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub qual_id: ::prost::alloc::string::String,
        #[prost(bool, tag="5")]
        pub retain: bool,
        #[prost(string, tag="6")]
        pub r#type: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisSubmission {
    #[prost(string, tag="1")]
    pub app_description: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub app_name: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub archive_logs: bool,
    #[prost(string, tag="5")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub condor_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub condor_log_path: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub create_output_subdir: bool,
    #[prost(message, optional, tag="9")]
    pub date_submitted: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub date_started: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="11")]
    pub date_completed: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="12")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub email: ::prost::alloc::string::String,
    /// read all about it
    #[prost(message, optional, tag="14")]
    pub extra: ::core::option::Option<Extra>,
    #[prost(string, tag="15")]
    pub execution_target: ::prost::alloc::string::String,
    #[prost(int32, tag="16")]
    pub exit_code: i32,
    #[prost(int64, tag="17")]
    pub failure_count: i64,
    #[prost(int64, tag="18")]
    pub failure_threshold: i64,
    /// The - is used instead of _ for backwards compatibility.
    #[prost(message, repeated, tag="19")]
    pub file_metadata: ::prost::alloc::vec::Vec<FileMetadata>,
    #[prost(string, repeated, tag="20")]
    pub filter_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="21")]
    pub group: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub input_path_list_file: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub input_tickets_file: ::prost::alloc::string::String,
    /// AKA the external ID.
    #[prost(string, tag="24")]
    pub invocation_id: ::prost::alloc::string::String,
    #[prost(string, tag="25")]
    pub irods_base: ::prost::alloc::string::String,
    #[prost(string, tag="26")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="27")]
    pub nfs_base: ::prost::alloc::string::String,
    #[prost(bool, tag="28")]
    pub notify: bool,
    #[prost(string, tag="29")]
    pub now_date: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub output_dir: ::prost::alloc::string::String,
    #[prost(string, tag="31")]
    pub output_dir_ticket: ::prost::alloc::string::String,
    #[prost(string, tag="32")]
    pub output_ticket_file: ::prost::alloc::string::String,
    #[prost(string, tag="33")]
    pub request_type: ::prost::alloc::string::String,
    /// The - is on purpose.
    #[prost(bool, tag="34")]
    pub run_on_nfs: bool,
    /// The - is on purpose. 
    #[prost(bool, tag="35")]
    pub skip_parent_metadata: bool,
    #[prost(message, repeated, tag="36")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
    #[prost(string, tag="37")]
    pub submission_date: ::prost::alloc::string::String,
    /// Yup, the JSON name is completely different from the field name.
    #[prost(string, tag="38")]
    pub submitter: ::prost::alloc::string::String,
    #[prost(string, tag="39")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="40")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="41")]
    pub user_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="42")]
    pub user_home: ::prost::alloc::string::String,
    #[prost(string, tag="43")]
    pub wiki_url: ::prost::alloc::string::String,
    #[prost(string, tag="44")]
    pub config_file: ::prost::alloc::string::String,
    #[prost(message, optional, tag="45")]
    pub header: ::core::option::Option<Header>,
}
/// Since protocol buffers don't have a way to alias messages, we're copying the Analysis definition into Job and deprecating it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    #[prost(string, tag="1")]
    pub app_description: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub app_name: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub archive_logs: bool,
    #[prost(string, tag="5")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub condor_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub condor_log_path: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub create_output_subdir: bool,
    #[prost(message, optional, tag="9")]
    pub date_submitted: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub date_started: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="11")]
    pub date_completed: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="12")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub email: ::prost::alloc::string::String,
    /// read all about it
    #[prost(message, optional, tag="14")]
    pub extra: ::core::option::Option<Extra>,
    #[prost(string, tag="15")]
    pub execution_target: ::prost::alloc::string::String,
    #[prost(int32, tag="16")]
    pub exit_code: i32,
    #[prost(int64, tag="17")]
    pub failure_count: i64,
    #[prost(int64, tag="18")]
    pub failure_threshold: i64,
    /// The - is used instead of _ for backwards compatibility.
    #[prost(message, repeated, tag="19")]
    pub file_metadata: ::prost::alloc::vec::Vec<FileMetadata>,
    #[prost(string, repeated, tag="20")]
    pub filter_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="21")]
    pub group: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub input_path_list_file: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub input_tickets_file: ::prost::alloc::string::String,
    /// AKA the external ID.
    #[prost(string, tag="24")]
    pub invocation_id: ::prost::alloc::string::String,
    #[prost(string, tag="25")]
    pub irods_base: ::prost::alloc::string::String,
    #[prost(string, tag="26")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="27")]
    pub nfs_base: ::prost::alloc::string::String,
    #[prost(bool, tag="28")]
    pub notify: bool,
    #[prost(string, tag="29")]
    pub now_date: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub output_dir: ::prost::alloc::string::String,
    #[prost(string, tag="31")]
    pub output_dir_ticket: ::prost::alloc::string::String,
    #[prost(string, tag="32")]
    pub output_ticket_file: ::prost::alloc::string::String,
    #[prost(string, tag="33")]
    pub request_type: ::prost::alloc::string::String,
    /// The - is on purpose.
    #[prost(bool, tag="34")]
    pub run_on_nfs: bool,
    /// The - is on purpose. 
    #[prost(bool, tag="35")]
    pub skip_parent_metadata: bool,
    #[prost(message, repeated, tag="36")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
    #[prost(string, tag="37")]
    pub submission_date: ::prost::alloc::string::String,
    /// Yup, the JSON name is completely different from the field name.
    #[prost(string, tag="38")]
    pub submitter: ::prost::alloc::string::String,
    #[prost(string, tag="39")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="40")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="41")]
    pub user_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="42")]
    pub user_home: ::prost::alloc::string::String,
    #[prost(string, tag="43")]
    pub wiki_url: ::prost::alloc::string::String,
    #[prost(string, tag="44")]
    pub config_file: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisStatus {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    #[prost(message, optional, tag="3")]
    pub job: ::core::option::Option<AnalysisSubmission>,
    #[prost(int32, tag="4")]
    pub version: i32,
    #[prost(string, tag="5")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub sent_on: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsLookup {
    #[prost(string, tag="1")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsCheckResult {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    #[prost(message, repeated, tag="3")]
    pub lookups: ::prost::alloc::vec::Vec<DnsLookup>,
    #[prost(string, tag="4")]
    pub node: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub date_sent: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LookupType {
    UnsetLookup = 0,
    InternalLookup = 1,
    ExternalLookup = 2,
}
impl LookupType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LookupType::UnsetLookup => "UNSET_LOOKUP",
            LookupType::InternalLookup => "INTERNAL_LOOKUP",
            LookupType::ExternalLookup => "EXTERNAL_LOOKUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSET_LOOKUP" => Some(Self::UnsetLookup),
            "INTERNAL_LOOKUP" => Some(Self::InternalLookup),
            "EXTERNAL_LOOKUP" => Some(Self::ExternalLookup),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    #[prost(string, tag="3")]
    pub node: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub date_sent: ::prost::alloc::string::String,
}
/// *
/// Representation of a resource type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceType {
    /// The unique identifier.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The name of the resource. Will usually be "data.size" and "cpu.hours".
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The units used for the resource. Usually "bytes" or "cpu hours".
    #[prost(string, tag="3")]
    pub unit: ::prost::alloc::string::String,
}
/// *
/// A response type for resource type requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceTypeResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The resource type returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub resource_type: ::core::option::Option<ResourceType>,
}
/// *
/// A response type for the resource type requests that contains a list of 
/// resource type definitions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceTypeList {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// A list of resource types returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub resource_types: ::prost::alloc::vec::Vec<ResourceType>,
}
/// *
/// A representation of a user in the QMS system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QmsUser {
    /// A user's unique identifier in QMS.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// A user's username in QMS.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
/// *
/// A response to a request for info about a QMS user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QmsUserResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The user returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub user: ::core::option::Option<QmsUser>,
}
/// *
///   A response to a request for info about a list of users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QmsUserList {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The user list returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub users: ::prost::alloc::vec::Vec<QmsUser>,
}
/// *
/// A request to add a user to the QMS system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserRequest {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The username for the user being added to the system.
    #[prost(string, tag="3")]
    pub username: ::prost::alloc::string::String,
    /// The name of the plan the user should be subscribed to.
    #[prost(string, tag="4")]
    pub plan_name: ::prost::alloc::string::String,
    /// True if the user paid for the subscription.
    #[prost(bool, tag="5")]
    pub paid: bool,
}
/// *
/// A response to a request to add a user to the QMS system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The unique identfier of the newly added user.
    #[prost(string, tag="3")]
    pub uuid: ::prost::alloc::string::String,
    /// The username of the newly added user.
    #[prost(string, tag="4")]
    pub username: ::prost::alloc::string::String,
    /// The name of the plan the newly added user is subscribed to.
    #[prost(string, tag="5")]
    pub plan_name: ::prost::alloc::string::String,
    /// The unique identifier for the plan the newly added user is subscribed to.
    #[prost(string, tag="6")]
    pub plan_uuid: ::prost::alloc::string::String,
}
/// *
/// Represents a default quota value used in plans. Can be overridden on a
/// per-user basis for a subscription to provide customized quotas. Also referred to
/// as plan quota defaults.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaDefault {
    /// The unique identifier/primary key for the quota default.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The value of the quota default.
    #[prost(float, tag="2")]
    pub quota_value: f32,
    /// The resource type the quota applies to.
    #[prost(message, optional, tag="3")]
    pub resource_type: ::core::option::Option<ResourceType>,
}
/// *
/// A response type for quota default requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaDefaultResponse {
    /// Can container telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Contains error info from the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The quota default object returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub quota_default: ::core::option::Option<QuotaDefault>,
}
/// *
/// A response type for quota default requests that contains a list of quota
/// defaults.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaDefaultList {
    /// Can contain telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Contains error info from the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The list of quota default objkects returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub quota_defaults: ::prost::alloc::vec::Vec<QuotaDefault>,
}
/// *
/// Represents a subscription plan available to users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Unique identifier for the plan.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The name of the plan.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// A description of the plan.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// A list of quota defaults associated with the plan.
    #[prost(message, repeated, tag="4")]
    pub plan_quota_defaults: ::prost::alloc::vec::Vec<QuotaDefault>,
}
/// *
/// A response to a plan request. Contains a single plan.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanResponse {
    /// Contains telemtry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Contains error data returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The plan returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub plan: ::core::option::Option<Plan>,
}
/// *
/// A response to a plan request. Contains a list of plans.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanList {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Contains error data returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// A list of plans returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub plans: ::prost::alloc::vec::Vec<Plan>,
}
/// *
/// A request for plan information specified by the plan's unique identifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanRequest {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The unique identifier of the plan being requested.
    #[prost(string, tag="2")]
    pub plan_id: ::prost::alloc::string::String,
}
/// *
/// A request to add a new plan to the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPlanRequest {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The plan to add to the system.
    #[prost(message, optional, tag="2")]
    pub plan: ::core::option::Option<Plan>,
}
/// *
/// A request to add a quota default to an existing plan.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPlanQuotaDefaultRequest {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The name of the plan to add the quota default to.
    #[prost(string, tag="2")]
    pub plan_name: ::prost::alloc::string::String,
    /// The quota default to add to the plan specified by the plan_name field.
    #[prost(message, optional, tag="3")]
    pub quota_default: ::core::option::Option<QuotaDefault>,
}
/// *
/// Represents a quota in the system, which is the currently configured limit on
/// a resource type a user has associated with their plan. Overrides the quota
/// default associated with the plan the user has.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quota {
    /// The unique identifier.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The quota value (aka limit).
    #[prost(float, tag="2")]
    pub quota: f32,
    /// The resource type the quota value applies to.
    #[prost(message, optional, tag="3")]
    pub resource_type: ::core::option::Option<ResourceType>,
    /// A freeform text field containing info about who created the quota.
    #[prost(string, tag="4")]
    pub created_by: ::prost::alloc::string::String,
    /// When the quota was created.
    #[prost(message, optional, tag="5")]
    pub created_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// A freeform text field containing info about who last modified the quota.
    #[prost(string, tag="6")]
    pub last_modified_by: ::prost::alloc::string::String,
    /// When the quota was last modified.
    #[prost(message, optional, tag="7")]
    pub last_modified_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The unique identifier of the subscription that the quota is associated with.
    #[prost(string, tag="8")]
    pub subscription_id: ::prost::alloc::string::String,
}
/// *
/// A response to a quota request. Contains a single quota object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaResponse {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The quota returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub quota: ::core::option::Option<Quota>,
}
/// *
/// A response to a quota request containing a list of quotas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaList {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// A list of quotas returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub quotas: ::prost::alloc::vec::Vec<Quota>,
}
/// *
/// A request to add a quota to a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddQuotaRequest {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The quota to be added to the system.
    #[prost(message, optional, tag="2")]
    pub quota: ::core::option::Option<Quota>,
}
/// *
/// A representation of how much a user has used a resource type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Usage {
    /// The unique identifier
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// How much the resource has been used.
    #[prost(double, tag="2")]
    pub usage: f64,
    /// The unique identifier for the subscription the usage is associated with.
    #[prost(string, tag="3")]
    pub subscription_id: ::prost::alloc::string::String,
    /// The resource type the usage applies to.
    #[prost(message, optional, tag="4")]
    pub resource_type: ::core::option::Option<ResourceType>,
    /// Who created the usage record. Probably not the name of a user.
    #[prost(string, tag="5")]
    pub created_by: ::prost::alloc::string::String,
    /// When the usage record was created.
    #[prost(message, optional, tag="6")]
    pub created_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Who last modified the usage record. Probably not the name of a user.
    #[prost(string, tag="7")]
    pub last_modified_by: ::prost::alloc::string::String,
    /// When the usage record was last modified.
    #[prost(message, optional, tag="8")]
    pub last_modified_at: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// *
/// A response to a request for a usage record.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// Contains the usage info returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub usage: ::core::option::Option<Usage>,
}
/// *
/// A response to a request for usage info containing multiple usage records.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageList {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// A list of usages returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub usages: ::prost::alloc::vec::Vec<Usage>,
}
/// *
/// Representation of a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// The unique identifier 
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The date the subscription activates.
    #[prost(message, optional, tag="2")]
    pub effective_start_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The date the subscription deactivates/expires.
    #[prost(message, optional, tag="3")]
    pub effective_end_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The user in the QMS system that the subscription is for.
    #[prost(message, optional, tag="4")]
    pub user: ::core::option::Option<QmsUser>,
    /// The plan the user is subscribed to.
    #[prost(message, optional, tag="5")]
    pub plan: ::core::option::Option<Plan>,
    /// The list of quotas applied to the subscription. Initially populated
    /// by quota defaults, but can be overridden.
    #[prost(message, repeated, tag="6")]
    pub quotas: ::prost::alloc::vec::Vec<Quota>,
    /// The list of resource usages that the user has generated while this plan was active.
    #[prost(message, repeated, tag="7")]
    pub usages: ::prost::alloc::vec::Vec<Usage>,
    /// A flag indicating whether or not the user paid for the subscription.
    #[prost(bool, tag="8")]
    pub paid: bool,
}
/// *
/// A response to a request for a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The subscription returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub subscription: ::core::option::Option<Subscription>,
}
/// *
/// A response to a request for a list of subscriptions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionList {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The subscription list returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
}
/// *
/// A request to change a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeSubscriptionRequest {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// A username for the user whose subscription is being changed.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    /// Either a plan's unique identifier or name.
    #[prost(oneof="change_subscription_request::Plan", tags="3, 4")]
    pub plan: ::core::option::Option<change_subscription_request::Plan>,
}
/// Nested message and enum types in `ChangeSubscriptionRequest`.
pub mod change_subscription_request {
    /// Either a plan's unique identifier or name.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Plan {
        #[prost(string, tag="3")]
        Uuid(::prost::alloc::string::String),
        #[prost(string, tag="4")]
        Name(::prost::alloc::string::String),
    }
}
/// *
/// Represents an add-on that can be applied to a subscription to provide a user
/// with a change in a resource quota.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Addon {
    /// The unique identifier.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The name of the add-on.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The description of the add-on.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// The resource type of the add-on.
    #[prost(message, optional, tag="4")]
    pub resource_type: ::core::option::Option<ResourceType>,
    /// How much of the resource type is added to the quota by the add-on.
    #[prost(float, tag="5")]
    pub default_amount: f32,
    /// Whether a user must pay for the add-on. Not whether the user has paid.
    #[prost(bool, tag="6")]
    pub default_paid: bool,
}
/// *
/// A response to an add-on request. Contains a single add-on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddonResponse {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The add-on returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub addon: ::core::option::Option<Addon>,
}
/// *
/// A response to an add-on request that contains a list of add-ons.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddonListResponse {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The list of add-ons returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub addons: ::prost::alloc::vec::Vec<Addon>,
}
/// *
/// A request to add an add-on to the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAddonRequest {
    /// Contains telemetry information.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The add-on to be added.
    #[prost(message, optional, tag="2")]
    pub addon: ::core::option::Option<Addon>,
}
/// *
/// A request to get information about an add-on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddonLookupRequest {
    /// Contains telemetry information.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Either the add-on's id or name.
    #[prost(oneof="addon_lookup_request::Addon", tags="2, 3")]
    pub addon: ::core::option::Option<addon_lookup_request::Addon>,
}
/// Nested message and enum types in `AddonLookupRequest`.
pub mod addon_lookup_request {
    /// Either the add-on's id or name.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Addon {
        #[prost(string, tag="2")]
        Uuid(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        Name(::prost::alloc::string::String),
    }
}
/// *
/// A request to update an add-on. The boolean fields are needed because Go (and
/// probably other languages) needs a way to tell when to set a field to an empty
/// string, since that's the zero value for a string.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAddonRequest {
    /// Contains telemetry information.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The values to set in the update.
    #[prost(message, optional, tag="2")]
    pub addon: ::core::option::Option<Addon>,
    /// Whether to update the name of the addon.
    #[prost(bool, tag="3")]
    pub update_name: bool,
    /// Whether to update the description of the addon.
    #[prost(bool, tag="4")]
    pub update_description: bool,
    /// Whether to update the resource type of the addon.
    #[prost(bool, tag="5")]
    pub update_resource_type: bool,
    /// Whether to update the default amount of the addon.
    #[prost(bool, tag="6")]
    pub update_default_amount: bool,
    /// Whether to update the default paid field of the addon.
    #[prost(bool, tag="7")]
    pub update_default_paid: bool,
}
/// *
/// SubscriptionAddon is an add-on that has been applied to a subscription. It 
/// contains fields that can override the the default_amount and default_paid 
/// fields in the subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionAddon {
    /// The unique identifier for the add-on
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The add-on used with the subscription. May only contain the add-on's 
    /// UUID in some circumstances.
    #[prost(message, optional, tag="2")]
    pub addon: ::core::option::Option<Addon>,
    /// The subscription the add-on was applied to. May only contain the add-on's
    /// UUID in some circumstances.
    #[prost(message, optional, tag="3")]
    pub subscription: ::core::option::Option<Subscription>,
    /// The amount of the resource applied by the add-on. This should default to
    /// the amount contained in the add-on definition, but can be overridden, 
    /// which is why it's a separate field here.
    #[prost(float, tag="4")]
    pub amount: f32,
    /// Whether the subscription add-on costs money. This should default to the 
    /// same paid value contained in the add-on definition, but can be overridden,
    /// which is whay it's a separate field here.
    #[prost(bool, tag="5")]
    pub paid: bool,
}
/// *
/// Contains the information needed to update a subscription add-on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscriptionAddonRequest {
    /// Contains telemetry information.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The subscription add-on information being updated. Does not necessarily
    /// have all fields set and the UUID field should not be set.
    #[prost(message, optional, tag="2")]
    pub subscription_addon: ::core::option::Option<SubscriptionAddon>,
    /// Whether to update the addon_id with the value contained in the subscription
    /// addon. The DE backend currently does not support this. Do a delete->add
    /// instead.
    #[prost(bool, tag="3")]
    pub update_addon_id: bool,
    /// Whether to update the subscription_id field with the value contained
    /// in the subscription addon. The DE backend currently does not support this.
    /// Do a delete->add instead.
    #[prost(bool, tag="4")]
    pub update_subscription_id: bool,
    /// Whether to update the amount field with the value contained in the 
    /// subscription addon.
    #[prost(bool, tag="5")]
    pub update_amount: bool,
    /// Whether to update the paid fields with the value contained in the 
    /// subscription addon.
    #[prost(bool, tag="6")]
    pub update_paid: bool,
}
/// *
/// Contains the subscription add-on returned by the request handler.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionAddonResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The subscription add-on returned by the request handler.
    #[prost(message, optional, tag="3")]
    pub subscription_addon: ::core::option::Option<SubscriptionAddon>,
}
/// *
/// Contains a list of subscription add-ons returned by the request handler.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionAddonListResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information generated by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The list of subscription add-ons returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub subscription_addons: ::prost::alloc::vec::Vec<SubscriptionAddon>,
}
/// *
/// Represents when a user's resource type usage exceeds their configured
/// quota. Usually embedded in request and response message types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Overage {
    /// The type of resource that is in overage. Usually data.size of cpu.hours.
    #[prost(string, tag="1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The configured quota value for the resource type.
    #[prost(float, tag="2")]
    pub quota: f32,
    /// The actual usage value for the resource type.
    #[prost(float, tag="3")]
    pub usage: f32,
}
/// *
/// Returned by handlers in response to overage requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverageResponse {
    /// The header used for passing telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Contains any errors generated by the handler emitting the response.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The overage returned by the handler emitting the response.
    #[prost(message, optional, tag="3")]
    pub overage: ::core::option::Option<Overage>,
}
/// *
/// A response message returned by handlers in response to overage related requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverageList {
    /// The header used for passing telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Contains any errors generated by the handler emitting the response.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The list of overages returned by the handler emitting the response.
    #[prost(message, repeated, tag="3")]
    pub overages: ::prost::alloc::vec::Vec<Overage>,
}
/// *
/// A response message returned by handlers in response to overage related requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsOverage {
    /// The header userd for passing telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Contains any errors generated by the handler emitting the response.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// Whether or not there is an overage.
    #[prost(bool, tag="3")]
    pub is_overage: bool,
}
/// *
/// A request for all of a user's current resource type overages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllUserOveragesRequest {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The user's username in the QMS system.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
/// *
/// A request for a user's overages specific to a particular resource type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserResourceOveragesRequest {
    /// Contains telemetry data.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// A user's username.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    /// The name of the resource type to look up overages for.
    #[prost(string, tag="3")]
    pub resource_name: ::prost::alloc::string::String,
}
/// *
/// A request to check if a user is in overage for a particular resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsOverageRequest {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// A username.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    /// The name of the resource type to check for usage overages by the user.
    #[prost(string, tag="3")]
    pub resource_name: ::prost::alloc::string::String,
}
/// *
/// A request to add a usage to the system for a resource type consumed by the
/// specified user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUsage {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_name: ::prost::alloc::string::String,
    /// Possible values are defined in the database, so we can't use an enum here
    #[prost(string, tag="4")]
    pub update_type: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub usage_value: f64,
    #[prost(string, tag="6")]
    pub resource_unit: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsages {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestByUsername {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestByUserId {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoParamsRequest {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
}
/// *
/// A representation of update operations, which are ways calling code can change
/// quota and usage value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOperation {
    /// The unique identifier 
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The name of the update operation
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// *
/// A representation of an update to a quota or usage value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    /// The unique identifier
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// Determines whether the update is for a "quota" or "usage". 
    #[prost(string, tag="2")]
    pub value_type: ::prost::alloc::string::String,
    /// The value being applied to the usage or quota.
    #[prost(double, tag="3")]
    pub value: f64,
    /// The date the update takes effect.
    #[prost(message, optional, tag="4")]
    pub effective_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The type of operation being done.
    #[prost(message, optional, tag="5")]
    pub operation: ::core::option::Option<UpdateOperation>,
    /// The resource type for the quota or usage being updated.
    #[prost(message, optional, tag="6")]
    pub resource_type: ::core::option::Option<ResourceType>,
    /// The user in the QMS system that the update is for.
    #[prost(message, optional, tag="7")]
    pub user: ::core::option::Option<QmsUser>,
}
/// *
/// A request to add an update to the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUpdateRequest {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The update being added to the system.
    #[prost(message, optional, tag="2")]
    pub update: ::core::option::Option<Update>,
}
/// *
/// A response to requests to add an update to the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUpdateResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The update added to the system.
    #[prost(message, optional, tag="3")]
    pub update: ::core::option::Option<Update>,
}
/// *
/// A request to get the list of updates generated by the specified user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateListRequest {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// The user whose updates have been requested.
    #[prost(message, optional, tag="2")]
    pub user: ::core::option::Option<QmsUser>,
}
/// *
/// A response containing the requested list of updates generated by a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateListResponse {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    /// Error information returned by the request handler.
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<ServiceError>,
    /// The list of updates returned by the request handler.
    #[prost(message, repeated, tag="3")]
    pub updates: ::prost::alloc::vec::Vec<Update>,
}
/// *
/// Request a resource by the username of a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByUsername {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
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
    pub header: ::core::option::Option<Header>,
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
    pub header: ::core::option::Option<Header>,
}
/// *
/// Request a resource by its UUID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByUuid {
    /// Contains telemetry information
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
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
    pub header: ::core::option::Option<Header>,
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
    pub header: ::core::option::Option<Header>,
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
    pub header: ::core::option::Option<Header>,
    /// The UUID of the parent/owner/primary resource.
    #[prost(string, tag="2")]
    pub parent_uuid: ::prost::alloc::string::String,
    /// The UUID of the child/object/secondary resource.
    #[prost(string, tag="3")]
    pub child_uuid: ::prost::alloc::string::String,
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
    #[prost(message, optional, tag="3")]
    pub preferences: ::core::option::Option<user::Preferences>,
    #[prost(message, repeated, tag="4")]
    pub logins: ::prost::alloc::vec::Vec<user::Login>,
    #[prost(uint32, tag="7")]
    pub login_count: u32,
    #[prost(message, optional, tag="8")]
    pub saved_searches: ::core::option::Option<user::SavedSearches>,
    #[prost(message, optional, tag="9")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="10")]
    pub error: ::core::option::Option<ServiceError>,
}
/// Nested message and enum types in `User`.
pub mod user {
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
}
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
    pub header: ::core::option::Option<Header>,
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
include!("debuff.serde.rs");
// @@protoc_insertion_point(module)
