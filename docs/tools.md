# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [tools.proto](#tools-proto)
    - [Tool](#tools-Tool)
  
- [Scalar Value Types](#scalar-value-types)



<a name="tools-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## tools.proto



<a name="tools-Tool"></a>

### Tool
A tool integrated into the system.

A tool is part of an app and can run in an execution environment.
Mostly correllates to the &#39;tools&#39; table in the &#39;de&#39; database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The UUID for the tool. |
| name | [string](#string) |  | The name of the tool. |
| version | [string](#string) |  | The version of the tool. |
| attribution | [string](#string) |  | Attribution information for the tool. |
| description | [string](#string) |  | The description of the tool. |
| time_limit_seconds | [int32](#int32) |  | The time limit the tool is allowed to run for by default. Unit is seconds. |
| restricted | [bool](#bool) |  | Whether the tool is restricted. |
| interactive | [bool](#bool) |  | Whether the tool is interactive (i.e. whether it is intended for use in VICE). |
| gpu_enabled | [bool](#bool) |  | Whether the tool requires a GPU. |
| integration_data | [apps.IntegrationData](#apps-IntegrationData) |  | Integration data associated with the tool. |
| container_image | [containers.Image](#containers-Image) |  | The container image to use when running the tool. |





 

 

 

 



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

