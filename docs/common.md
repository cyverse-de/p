# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [header.proto](#header-proto)
    - [Header](#header-Header)
    - [Header.MapEntry](#header-Header-MapEntry)
    - [Header.Value](#header-Header-Value)
  
- [svcerror.proto](#svcerror-proto)
    - [Error](#svcerror-Error)
    - [ServiceError](#svcerror-ServiceError)
  
    - [ErrorCode](#svcerror-ErrorCode)
  
- [Scalar Value Types](#scalar-value-types)



<a name="header-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## header.proto



<a name="header-Header"></a>

### Header



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| map | [Header.MapEntry](#header-Header-MapEntry) | repeated |  |






<a name="header-Header-MapEntry"></a>

### Header.MapEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Header.Value](#header-Header-Value) |  |  |






<a name="header-Header-Value"></a>

### Header.Value



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [string](#string) | repeated |  |





 

 

 

 



<a name="svcerror-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## svcerror.proto



<a name="svcerror-Error"></a>

### Error
Represents an error that occurred in the backend. Probably
should not be returned directly by a HTTP&#43;JSON or REST API.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| code | [ErrorCode](#svcerror-ErrorCode) |  | Should roughly correspond to a type of error that can occur in the backend code. |
| message | [string](#string) |  | The message or stack trace generated when the error occurred. |






<a name="svcerror-ServiceError"></a>

### ServiceError
An error returned by a request handler. Kept around for backwards compatibility,
please use Error for new messages.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| error_code | [ErrorCode](#svcerror-ErrorCode) |  | The numeric error code from the error code enum. |
| status_code | [int32](#int32) |  | The status code for the error. |
| message | [string](#string) |  | The error&#39;s message. |





 


<a name="svcerror-ErrorCode"></a>

### ErrorCode
The types of errors that can be retuned by message handlers.

| Name | Number | Description |
| ---- | ------ | ----------- |
| UNSET | 0 | Default value for the error code. Don&#39;t set the error code to this. Use Unspecified if tempted. |
| UNSPECIFIED | 1 | An error occurred, but the kind wasn&#39;t specified or included in the list. |
| INTERNAL | 2 | Internal error. |
| NOT_FOUND | 3 | The requested resource wasn&#39;t found. |
| BAD_REQUEST | 4 | The request was bad/wrong is some way. |
| MARSHAL_FAILURE | 5 | A failure to marshal a response. |
| UNMARSHAL_FAILURE | 6 | A failure to unmarshal a request. |
| PARAMETER_MISSING | 7 | A parameter is missing. |
| PARAMETER_INVALID | 8 | A parameter is invalid. |
| UNAUTHENTICATED | 9 | Operation requires authentication, which was not provided. |
| FORBIDDEN | 10 | Operation is no allowed. |
| TIMEOUT | 11 | Operation timed out. |
| UNSUPPORTED | 12 | Operation is not supported. |
| UNIMPLEMENTED | 13 | Operation has not been implemented. |


 

 

 



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

