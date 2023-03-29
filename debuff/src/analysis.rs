// @generated
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
/// *
/// Corresponds to the job_types table in the database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisType {
    /// The UUID for the analysis type.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The name of the analysis type.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The kind of system the analysis should run on.
    #[prost(string, tag="3")]
    pub system_id: ::prost::alloc::string::String,
}
/// *
/// An analysis in the system.
///
/// An analysis is an app that was run by a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Analysis {
    /// The UUID for the analysis
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The description of the analysis provided by the user.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The name of the analysis provided by the user.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// App information about the analysis.
    #[prost(message, optional, tag="4")]
    pub app: ::core::option::Option<super::apps::App>,
    /// App version information for the analysis.
    #[prost(message, optional, tag="5")]
    pub app_version: ::core::option::Option<super::apps::AppVersion>,
    /// The analysis type, which tells which environment to run the analysis in.
    #[prost(message, optional, tag="6")]
    pub r#type: ::core::option::Option<AnalysisType>,
    /// The path to the folder containing analysis outputs.
    #[prost(string, tag="7")]
    pub result_folder_path: ::prost::alloc::string::String,
    /// The date the analysis was submitted.
    #[prost(message, optional, tag="8")]
    pub start_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The date the analyses finished.
    #[prost(message, optional, tag="9")]
    pub end_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The date the analysis was scheduled to end.
    #[prost(message, optional, tag="10")]
    pub planned_end_date: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The status of the analysis
    #[prost(string, tag="11")]
    pub status: ::prost::alloc::string::String,
    /// Whether the analysis was deleted.
    #[prost(bool, tag="12")]
    pub deleted: bool,
    /// Whether notifications should be emitted on status changes.
    #[prost(bool, tag="13")]
    pub notify: bool,
    /// The user that submitted the analysis.
    #[prost(message, optional, tag="14")]
    pub user: ::core::option::Option<super::user::User>,
    /// The subdomain assigned to the analysis for VICE.
    #[prost(string, tag="15")]
    pub subdomain: ::prost::alloc::string::String,
    /// The UUID of the analysis that spawned this analysis. Used for batch analyses.
    #[prost(string, tag="16")]
    pub parent_id: ::prost::alloc::string::String,
    /// The number of millicores reserved for the analysis.
    #[prost(double, tag="17")]
    pub millicores_reserved: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisRecordLookupRequest {
    #[prost(message, optional, tag="5")]
    pub header: ::core::option::Option<super::header::Header>,
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
    pub header: ::core::option::Option<super::header::Header>,
    #[prost(message, repeated, tag="2")]
    pub analyses: ::prost::alloc::vec::Vec<Analysis>,
    #[prost(string, tag="3")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub total: i64,
    #[prost(message, repeated, tag="5")]
    pub status_count: ::prost::alloc::vec::Vec<analysis_record_response::StatusCountRecord>,
    #[prost(message, optional, tag="6")]
    pub error: ::core::option::Option<super::svcerror::ServiceError>,
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
    pub header: ::core::option::Option<super::header::Header>,
    #[prost(message, repeated, tag="2")]
    pub analyses: ::prost::alloc::vec::Vec<Analysis>,
    #[prost(message, optional, tag="7")]
    pub error: ::core::option::Option<super::svcerror::ServiceError>,
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
    pub header: ::core::option::Option<super::header::Header>,
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
    pub header: ::core::option::Option<super::header::Header>,
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
    pub header: ::core::option::Option<super::header::Header>,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<super::svcerror::ServiceError>,
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
include!("analysis.serde.rs");
// @@protoc_insertion_point(module)
