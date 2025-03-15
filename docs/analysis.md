# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [analysis_record.proto](#analysis_record-proto)
    - [Analysis](#analysis-Analysis)
    - [AnalysisType](#analysis-AnalysisType)
    - [BatchStatus](#analysis-BatchStatus)
  
- [analysis_requests.proto](#analysis_requests-proto)
    - [AnalysisRecordList](#analysis-AnalysisRecordList)
    - [AnalysisRecordLookupRequest](#analysis-AnalysisRecordLookupRequest)
    - [AnalysisRecordResponse](#analysis-AnalysisRecordResponse)
    - [AnalysisRecordResponse.StatusCountRecord](#analysis-AnalysisRecordResponse-StatusCountRecord)
  
- [analysis_status.proto](#analysis_status-proto)
    - [AnalysisStatus](#analysis-AnalysisStatus)
  
- [analysis_submission.proto](#analysis_submission-proto)
    - [AnalysisSubmission](#analysis-AnalysisSubmission)
    - [Extra](#analysis-Extra)
    - [FileMetadata](#analysis-FileMetadata)
    - [HTCondorExtraInfo](#analysis-HTCondorExtraInfo)
    - [Job](#analysis-Job)
    - [Step](#analysis-Step)
    - [Step.Component](#analysis-Step-Component)
    - [Step.Config](#analysis-Step-Config)
    - [Step.EnvironmentEntry](#analysis-Step-EnvironmentEntry)
    - [Step.Input](#analysis-Step-Input)
    - [Step.Output](#analysis-Step-Output)
    - [Step.Param](#analysis-Step-Param)
  
- [Scalar Value Types](#scalar-value-types)



<a name="analysis_record-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_record.proto



<a name="analysis-Analysis"></a>

### Analysis
An analysis in the system.

An analysis is an app that was run by a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The UUID for the analysis |
| description | [string](#string) |  | The description of the analysis provided by the user. |
| name | [string](#string) |  | The name of the analysis provided by the user. |
| app | [apps.App](#apps-App) |  | App information about the analysis. |
| app_version | [apps.AppVersion](#apps-AppVersion) |  | App version information for the analysis. |
| kind | [AnalysisType](#analysis-AnalysisType) |  | The analysis type, which tells which environment to run the analysis in. |
| result_folder_path | [string](#string) |  | The path to the folder containing analysis outputs. |
| start_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date the analysis was submitted. |
| end_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date the analyses finished. |
| planned_end_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date the analysis was scheduled to end. |
| status | [string](#string) |  | The status of the analysis |
| deleted | [bool](#bool) |  | Whether the analysis was deleted. |
| notify | [bool](#bool) |  | Whether notifications should be emitted on status changes. |
| user | [user.User](#user-User) |  | The user that submitted the analysis. |
| subdomain | [string](#string) |  | The subdomain assigned to the analysis for VICE. |
| parent_id | [string](#string) |  | The UUID of the analysis that spawned this analysis. Used for batch analyses. |
| millicores_reserved | [double](#double) |  | The number of millicores reserved for the analysis. |






<a name="analysis-AnalysisType"></a>

### AnalysisType
Corresponds to the job_types table in the database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The UUID for the analysis type. |
| name | [string](#string) |  | The name of the analysis type. |
| system_id | [string](#string) |  | The kind of system the analysis should run on. |






<a name="analysis-BatchStatus"></a>

### BatchStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total | [int64](#int64) |  |  |
| completed | [int64](#int64) |  |  |
| running | [int64](#int64) |  |  |
| submitted | [int64](#int64) |  |  |





 

 

 

 



<a name="analysis_requests-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_requests.proto



<a name="analysis-AnalysisRecordList"></a>

### AnalysisRecordList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  |  |
| analyses | [Analysis](#analysis-Analysis) | repeated |  |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  |  |






<a name="analysis-AnalysisRecordLookupRequest"></a>

### AnalysisRecordLookupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| analysis_id | [string](#string) |  |  |
| external_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| username | [string](#string) |  |  |
| header | [header.Header](#header-Header) |  |  |
| requesting_user | [string](#string) |  |  |






<a name="analysis-AnalysisRecordResponse"></a>

### AnalysisRecordResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  |  |
| analyses | [Analysis](#analysis-Analysis) | repeated |  |
| timestamp | [string](#string) |  |  |
| total | [int64](#int64) |  |  |
| status_count | [AnalysisRecordResponse.StatusCountRecord](#analysis-AnalysisRecordResponse-StatusCountRecord) | repeated |  |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  |  |






<a name="analysis-AnalysisRecordResponse-StatusCountRecord"></a>

### AnalysisRecordResponse.StatusCountRecord



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [int64](#int64) |  |  |
| status | [string](#string) |  |  |





 

 

 

 



<a name="analysis_status-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_status.proto



<a name="analysis-AnalysisStatus"></a>

### AnalysisStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  |  |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  |  |
| job | [AnalysisSubmission](#analysis-AnalysisSubmission) |  |  |
| version | [int32](#int32) |  |  |
| state | [string](#string) |  |  |
| message | [string](#string) |  |  |
| sent_on | [string](#string) |  |  |
| sender | [string](#string) |  |  |





 

 

 

 



<a name="analysis_submission-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## analysis_submission.proto



<a name="analysis-AnalysisSubmission"></a>

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
| extra | [Extra](#analysis-Extra) |  | read all about it |
| execution_target | [string](#string) |  |  |
| exit_code | [int32](#int32) |  |  |
| failure_count | [int64](#int64) |  |  |
| failure_threshold | [int64](#int64) |  |  |
| file_metadata | [FileMetadata](#analysis-FileMetadata) | repeated | The - is used instead of _ for backwards compatibility. |
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
| steps | [Step](#analysis-Step) | repeated |  |
| submission_date | [string](#string) |  |  |
| submitter | [string](#string) |  | Yup, the JSON name is completely different from the field name. |
| type | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| user_groups | [string](#string) | repeated |  |
| user_home | [string](#string) |  |  |
| wiki_url | [string](#string) |  |  |
| config_file | [string](#string) |  |  |
| mount_data_store | [bool](#bool) |  |  |






<a name="analysis-Extra"></a>

### Extra



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ht_condor | [HTCondorExtraInfo](#analysis-HTCondorExtraInfo) |  |  |






<a name="analysis-FileMetadata"></a>

### FileMetadata



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| attribute | [string](#string) |  |  |
| value | [string](#string) |  |  |
| unit | [string](#string) |  |  |






<a name="analysis-HTCondorExtraInfo"></a>

### HTCondorExtraInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| extra_requirements | [string](#string) |  |  |






<a name="analysis-Job"></a>

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
| extra | [Extra](#analysis-Extra) |  | read all about it |
| execution_target | [string](#string) |  |  |
| exit_code | [int32](#int32) |  |  |
| failure_count | [int64](#int64) |  |  |
| failure_threshold | [int64](#int64) |  |  |
| file_metadata | [FileMetadata](#analysis-FileMetadata) | repeated | The - is used instead of _ for backwards compatibility. |
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
| steps | [Step](#analysis-Step) | repeated |  |
| submission_date | [string](#string) |  |  |
| submitter | [string](#string) |  | Yup, the JSON name is completely different from the field name. |
| type | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| user_groups | [string](#string) | repeated |  |
| user_home | [string](#string) |  |  |
| wiki_url | [string](#string) |  |  |
| config_file | [string](#string) |  |  |
| mount_data_store | [bool](#bool) |  |  |






<a name="analysis-Step"></a>

### Step



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| component | [Step.Component](#analysis-Step-Component) |  |  |
| config | [Step.Config](#analysis-Step-Config) |  |  |
| type | [string](#string) |  |  |
| stdin_path | [string](#string) |  |  |
| stdout_path | [string](#string) |  |  |
| stderr_path | [string](#string) |  |  |
| log_file | [string](#string) |  |  |
| environment | [Step.EnvironmentEntry](#analysis-Step-EnvironmentEntry) | repeated |  |
| input | [Step.Input](#analysis-Step-Input) | repeated | It&#39;s really annoying, but this is singular for backwards compatibility. |
| output | [Step.Output](#analysis-Step-Output) | repeated | It&#39;s really annoying, but this is singular for backwards compatibility. |






<a name="analysis-Step-Component"></a>

### Step.Component



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| container | [containers.Container](#containers-Container) |  |  |
| type | [string](#string) |  |  |
| name | [string](#string) |  |  |
| location | [string](#string) |  |  |
| description | [string](#string) |  |  |
| time_limit | [int32](#int32) |  |  |
| restricted | [bool](#bool) |  |  |
| is_interactive | [bool](#bool) |  |  |






<a name="analysis-Step-Config"></a>

### Step.Config



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [Step.Param](#analysis-Step-Param) | repeated |  |
| inputs | [Step.Input](#analysis-Step-Input) | repeated | Keep it singular in the JSON for backwards-compatibility. |
| outputs | [Step.Output](#analysis-Step-Output) | repeated | Keep it singular in the JSON for backwards-compatibility. |






<a name="analysis-Step-EnvironmentEntry"></a>

### Step.EnvironmentEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="analysis-Step-Input"></a>

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






<a name="analysis-Step-Output"></a>

### Step.Output



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multiplicity | [string](#string) |  |  |
| name | [string](#string) |  |  |
| property | [string](#string) |  |  |
| qual_id | [string](#string) |  |  |
| retain | [bool](#bool) |  |  |
| type | [string](#string) |  |  |






<a name="analysis-Step-Param"></a>

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

