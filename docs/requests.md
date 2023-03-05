# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [requests.proto](#requests-proto)
    - [AssociateByUUIDs](#requests-AssociateByUUIDs)
    - [ByUUID](#requests-ByUUID)
    - [ByUUIDAndUserID](#requests-ByUUIDAndUserID)
    - [ByUUIDAndUsername](#requests-ByUUIDAndUsername)
    - [ByUserID](#requests-ByUserID)
    - [ByUsername](#requests-ByUsername)
    - [NoParams](#requests-NoParams)
  
- [Scalar Value Types](#scalar-value-types)



<a name="requests-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## requests.proto



<a name="requests-AssociateByUUIDs"></a>

### AssociateByUUIDs
Request that two resources be associated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information. |
| parent_uuid | [string](#string) |  | The UUID of the parent/owner/primary resource. |
| child_uuid | [string](#string) |  | The UUID of the child/object/secondary resource. |






<a name="requests-ByUUID"></a>

### ByUUID
Request a resource by its UUID.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| uuid | [string](#string) |  | The UUID of the resource being requested. |






<a name="requests-ByUUIDAndUserID"></a>

### ByUUIDAndUserID
Request a resource by its UUID and a user&#39;s UUID. Useful when the user&#39;s 
access to the resource must be verified.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| uuid | [string](#string) |  | The UUID of the resource being requested |
| user_id | [string](#string) |  | The user ID of the user associated with the request. |






<a name="requests-ByUUIDAndUsername"></a>

### ByUUIDAndUsername
Request a resource by its UUID and a username. Useful in situations where a
user&#39;s ability to access a resource needs to be checked as part of the 
request handler logic.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| uuid | [string](#string) |  | The UUID of the resource being requested |
| username | [string](#string) |  | The username associated with the request. |






<a name="requests-ByUserID"></a>

### ByUserID
Request a resource by the user ID of a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| user_id | [string](#string) |  |  |






<a name="requests-ByUsername"></a>

### ByUsername
Request a resource by the username of a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| username | [string](#string) |  |  |






<a name="requests-NoParams"></a>

### NoParams
Send a message that does not request any parameters. Common for triggering
side-effects or for retrieving lists of resources as an administrator.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |





 

 

 

 



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

