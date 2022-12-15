# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [analysis_container.proto](#analysis_container-proto)
    - [Container](#-Container)
    - [Container.Device](#-Container-Device)
    - [Container.Image](#-Container-Image)
    - [Container.Port](#-Container-Port)
    - [Container.Volume](#-Container-Volume)
    - [Container.VolumesFrom](#-Container-VolumesFrom)
    - [InteractiveApps](#-InteractiveApps)
  
- [analysis_record.proto](#analysis_record-proto)
    - [AnalysisRecord](#-AnalysisRecord)
    - [AnalysisRecord.BatchStatus](#-AnalysisRecord-BatchStatus)
  
- [analysis_requests.proto](#analysis_requests-proto)
    - [AnalysisRecordList](#-AnalysisRecordList)
    - [AnalysisRecordLookupRequest](#-AnalysisRecordLookupRequest)
    - [AnalysisRecordResponse](#-AnalysisRecordResponse)
    - [AnalysisRecordResponse.StatusCountRecord](#-AnalysisRecordResponse-StatusCountRecord)
  
- [analysis_status.proto](#analysis_status-proto)
    - [AnalysisStatus](#-AnalysisStatus)
  
- [analysis_submission.proto](#analysis_submission-proto)
    - [AnalysisSubmission](#-AnalysisSubmission)
    - [Extra](#-Extra)
    - [FileMetadata](#-FileMetadata)
    - [HTCondorExtraInfo](#-HTCondorExtraInfo)
    - [Job](#-Job)
    - [Step](#-Step)
    - [Step.Component](#-Step-Component)
    - [Step.Config](#-Step-Config)
    - [Step.EnvironmentEntry](#-Step-EnvironmentEntry)
    - [Step.Input](#-Step-Input)
    - [Step.Output](#-Step-Output)
    - [Step.Param](#-Step-Param)
  
- [Scalar Value Types](#scalar-value-types)



<a name="analysis_container-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_container.proto



<a name="-Container"></a>

### Container



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| volumes | [Container.Volume](#Container-Volume) | repeated |  |
| devices | [Container.Device](#Container-Device) | repeated |  |
| volumes_from | [Container.VolumesFrom](#Container-VolumesFrom) | repeated |  |
| name | [string](#string) |  |  |
| network_mode | [string](#string) |  |  |
| cpu_shares | [int64](#int64) |  |  |
| interactive_apps | [InteractiveApps](#InteractiveApps) |  |  |
| memory_limit | [int64](#int64) |  |  |
| min_memory_limit | [int64](#int64) |  |  |
| max_cpu_cores | [float](#float) |  |  |
| min_cpu_cores | [float](#float) |  |  |
| min_disk_space | [int64](#int64) |  |  |
| pids_limit | [int64](#int64) |  |  |
| image | [Container.Image](#Container-Image) |  |  |
| entry_point | [string](#string) |  |  |
| working_dir | [string](#string) |  |  |
| ports | [Container.Port](#Container-Port) | repeated |  |
| skip_tmp_mount | [bool](#bool) |  |  |
| uid | [int32](#int32) |  |  |
| header | [Header](#Header) |  |  |






<a name="-Container-Device"></a>

### Container.Device



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| host_path | [string](#string) |  |  |
| container_path | [string](#string) |  |  |
| cgroup_permissions | [string](#string) |  |  |






<a name="-Container-Image"></a>

### Container.Image



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| tag | [string](#string) |  |  |
| auth | [string](#string) |  |  |
| url | [string](#string) |  |  |
| osg_image_path | [string](#string) |  |  |






<a name="-Container-Port"></a>

### Container.Port



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| host_port | [int32](#int32) |  |  |
| container_port | [int32](#int32) |  |  |
| bind_to_host | [bool](#bool) |  |  |






<a name="-Container-Volume"></a>

### Container.Volume



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| host_path | [string](#string) |  |  |
| container_path | [string](#string) |  |  |
| read_only | [bool](#bool) |  |  |
| mode | [string](#string) |  |  |






<a name="-Container-VolumesFrom"></a>

### Container.VolumesFrom



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tag | [string](#string) |  |  |
| name | [string](#string) |  |  |
| auth | [string](#string) |  |  |
| name_prefix | [string](#string) |  |  |
| url | [string](#string) |  |  |
| host_path | [string](#string) |  |  |
| container_path | [string](#string) |  |  |
| read_only | [bool](#bool) |  |  |






<a name="-InteractiveApps"></a>

### InteractiveApps



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| proxy_image | [string](#string) |  |  |
| proxy_name | [string](#string) |  |  |
| frontend_url | [string](#string) |  |  |
| cas_url | [string](#string) |  |  |
| cas_validate | [string](#string) |  |  |
| ssl_cert_path | [string](#string) |  |  |
| ssl_key_path | [string](#string) |  |  |
| websocket_path | [string](#string) |  |  |
| websocket_port | [string](#string) |  |  |
| websocket_proto | [string](#string) |  |  |
| backend_url | [string](#string) |  |  |
| header | [Header](#Header) |  |  |





 

 

 

 



<a name="analysis_record-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_record.proto



<a name="-AnalysisRecord"></a>

### AnalysisRecord



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [Header](#Header) |  |  |
| id | [string](#string) |  |  |
| description | [string](#string) |  |  |
| name | [string](#string) |  |  |
| can_share | [bool](#bool) |  |  |
| username | [string](#string) |  |  |
| app_id | [string](#string) |  |  |
| batch_status | [AnalysisRecord.BatchStatus](#AnalysisRecord-BatchStatus) |  |  |
| system_id | [string](#string) |  |  |
| app_disabled | [bool](#bool) |  |  |
| batch | [bool](#bool) |  |  |
| enddate | [string](#string) |  |  |
| startdate | [string](#string) |  |  |
| app_description | [string](#string) |  |  |
| interactive_urls | [string](#string) | repeated |  |
| wiki_url | [string](#string) |  |  |
| notify | [bool](#bool) |  |  |
| result_folder_id | [string](#string) |  |  |
| app_name | [string](#string) |  |  |






<a name="-AnalysisRecord-BatchStatus"></a>

### AnalysisRecord.BatchStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total | [int64](#int64) |  |  |
| completed | [int64](#int64) |  |  |
| running | [int64](#int64) |  |  |
| submitted | [int64](#int64) |  |  |





 

 

 

 



<a name="analysis_requests-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_requests.proto



<a name="-AnalysisRecordList"></a>

### AnalysisRecordList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [Header](#Header) |  |  |
| analyses | [AnalysisRecord](#AnalysisRecord) | repeated |  |
| error | [ServiceError](#ServiceError) |  |  |






<a name="-AnalysisRecordLookupRequest"></a>

### AnalysisRecordLookupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| analysis_id | [string](#string) |  |  |
| external_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| username | [string](#string) |  |  |
| header | [Header](#Header) |  |  |
| requesting_user | [string](#string) |  |  |






<a name="-AnalysisRecordResponse"></a>

### AnalysisRecordResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [Header](#Header) |  |  |
| analyses | [AnalysisRecord](#AnalysisRecord) | repeated |  |
| timestamp | [string](#string) |  |  |
| total | [int64](#int64) |  |  |
| status_count | [AnalysisRecordResponse.StatusCountRecord](#AnalysisRecordResponse-StatusCountRecord) | repeated |  |
| error | [ServiceError](#ServiceError) |  |  |






<a name="-AnalysisRecordResponse-StatusCountRecord"></a>

### AnalysisRecordResponse.StatusCountRecord



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [int64](#int64) |  |  |
| status | [string](#string) |  |  |





 

 

 

 



<a name="analysis_status-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_status.proto



<a name="-AnalysisStatus"></a>

### AnalysisStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [Header](#Header) |  |  |
| error | [ServiceError](#ServiceError) |  |  |
| job | [AnalysisSubmission](#AnalysisSubmission) |  |  |
| version | [int32](#int32) |  |  |
| state | [string](#string) |  |  |
| message | [string](#string) |  |  |
| sent_on | [string](#string) |  |  |
| sender | [string](#string) |  |  |





 

 

 

 



<a name="analysis_submission-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_submission.proto



<a name="-AnalysisSubmission"></a>

### AnalysisSubmission



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| app_description | [string](#string) |  |  |
| app_id | [string](#string) |  |  |
| app_name | [string](#string) |  |  |
| archive_logs | [bool](#bool) |  |  |
| batch_id | [string](#string) |  |  |
| condor_id | [string](#string) |  |  |
| condor_log_path | [string](#string) |  |  |
| create_output_subdir | [bool](#bool) |  |  |
| date_submitted | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| date_started | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| date_completed | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| description | [string](#string) |  |  |
| email | [string](#string) |  |  |
| extra | [Extra](#Extra) |  | read all about it |
| execution_target | [string](#string) |  |  |
| exit_code | [int32](#int32) |  |  |
| failure_count | [int64](#int64) |  |  |
| failure_threshold | [int64](#int64) |  |  |
| file_metadata | [FileMetadata](#FileMetadata) | repeated | The - is used instead of _ for backwards compatibility. |
| filter_files | [string](#string) | repeated |  |
| group | [string](#string) |  |  |
| input_path_list_file | [string](#string) |  |  |
| input_tickets_file | [string](#string) |  |  |
| invocation_id | [string](#string) |  | AKA the external ID. |
| irods_base | [string](#string) |  |  |
| name | [string](#string) |  |  |
| nfs_base | [string](#string) |  |  |
| notify | [bool](#bool) |  |  |
| now_date | [string](#string) |  |  |
| output_dir | [string](#string) |  |  |
| output_dir_ticket | [string](#string) |  |  |
| output_ticket_file | [string](#string) |  |  |
| request_type | [string](#string) |  |  |
| run_on_nfs | [bool](#bool) |  | The - is on purpose. |
| skip_parent_metadata | [bool](#bool) |  | The - is on purpose. |
| steps | [Step](#Step) | repeated |  |
| submission_date | [string](#string) |  |  |
| submitter | [string](#string) |  | Yup, the JSON name is completely different from the field name. |
| type | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| user_groups | [string](#string) | repeated |  |
| user_home | [string](#string) |  |  |
| wiki_url | [string](#string) |  |  |
| config_file | [string](#string) |  |  |
| header | [Header](#Header) |  |  |






<a name="-Extra"></a>

### Extra



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ht_condor | [HTCondorExtraInfo](#HTCondorExtraInfo) |  |  |






<a name="-FileMetadata"></a>

### FileMetadata



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| attribute | [string](#string) |  |  |
| value | [string](#string) |  |  |
| unit | [string](#string) |  |  |






<a name="-HTCondorExtraInfo"></a>

### HTCondorExtraInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| extra_requirements | [string](#string) |  |  |






<a name="-Job"></a>

### Job
Since protocol buffers don&#39;t have a way to alias messages, we&#39;re copying the Analysis definition into Job and deprecating it.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| app_description | [string](#string) |  |  |
| app_id | [string](#string) |  |  |
| app_name | [string](#string) |  |  |
| archive_logs | [bool](#bool) |  |  |
| batch_id | [string](#string) |  |  |
| condor_id | [string](#string) |  |  |
| condor_log_path | [string](#string) |  |  |
| create_output_subdir | [bool](#bool) |  |  |
| date_submitted | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| date_started | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| date_completed | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| description | [string](#string) |  |  |
| email | [string](#string) |  |  |
| extra | [Extra](#Extra) |  | read all about it |
| execution_target | [string](#string) |  |  |
| exit_code | [int32](#int32) |  |  |
| failure_count | [int64](#int64) |  |  |
| failure_threshold | [int64](#int64) |  |  |
| file_metadata | [FileMetadata](#FileMetadata) | repeated | The - is used instead of _ for backwards compatibility. |
| filter_files | [string](#string) | repeated |  |
| group | [string](#string) |  |  |
| input_path_list_file | [string](#string) |  |  |
| input_tickets_file | [string](#string) |  |  |
| invocation_id | [string](#string) |  | AKA the external ID. |
| irods_base | [string](#string) |  |  |
| name | [string](#string) |  |  |
| nfs_base | [string](#string) |  |  |
| notify | [bool](#bool) |  |  |
| now_date | [string](#string) |  |  |
| output_dir | [string](#string) |  |  |
| output_dir_ticket | [string](#string) |  |  |
| output_ticket_file | [string](#string) |  |  |
| request_type | [string](#string) |  |  |
| run_on_nfs | [bool](#bool) |  | The - is on purpose. |
| skip_parent_metadata | [bool](#bool) |  | The - is on purpose. |
| steps | [Step](#Step) | repeated |  |
| submission_date | [string](#string) |  |  |
| submitter | [string](#string) |  | Yup, the JSON name is completely different from the field name. |
| type | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| user_groups | [string](#string) | repeated |  |
| user_home | [string](#string) |  |  |
| wiki_url | [string](#string) |  |  |
| config_file | [string](#string) |  |  |






<a name="-Step"></a>

### Step



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| component | [Step.Component](#Step-Component) |  |  |
| config | [Step.Config](#Step-Config) |  |  |
| type | [string](#string) |  |  |
| stdin_path | [string](#string) |  |  |
| stdout_path | [string](#string) |  |  |
| stderr_path | [string](#string) |  |  |
| log_file | [string](#string) |  |  |
| environment | [Step.EnvironmentEntry](#Step-EnvironmentEntry) | repeated |  |
| input | [Step.Input](#Step-Input) | repeated | It&#39;s really annoying, but this is singular for backwards compatibility. |
| output | [Step.Output](#Step-Output) | repeated | It&#39;s really annoying, but this is singular for backwards compatibility. |
| header | [Header](#Header) |  |  |






<a name="-Step-Component"></a>

### Step.Component



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| container | [Container](#Container) |  |  |
| type | [string](#string) |  |  |
| name | [string](#string) |  |  |
| location | [string](#string) |  |  |
| description | [string](#string) |  |  |
| time_limit | [int32](#int32) |  |  |
| restricted | [bool](#bool) |  |  |
| is_interactive | [bool](#bool) |  |  |






<a name="-Step-Config"></a>

### Step.Config



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [Step.Param](#Step-Param) | repeated |  |
| inputs | [Step.Input](#Step-Input) | repeated | Keep it singular in the JSON for backwards-compatibility. |
| outputs | [Step.Output](#Step-Output) | repeated | Keep it singular in the JSON for backwards-compatibility. |






<a name="-Step-EnvironmentEntry"></a>

### Step.EnvironmentEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="-Step-Input"></a>

### Step.Input



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| ticket | [string](#string) |  |  |
| multiplicity | [string](#string) |  |  |
| name | [string](#string) |  |  |
| property | [string](#string) |  |  |
| retain | [bool](#bool) |  |  |
| type | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="-Step-Output"></a>

### Step.Output



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multiplicity | [string](#string) |  |  |
| name | [string](#string) |  |  |
| property | [string](#string) |  |  |
| qual_id | [string](#string) |  |  |
| retain | [bool](#bool) |  |  |
| type | [string](#string) |  |  |






<a name="-Step-Param"></a>

### Step.Param



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| value | [string](#string) |  |  |
| order | [int32](#int32) |  |  |
| type | [string](#string) |  |  |
| path | [string](#string) |  |  |





 

 

 

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

