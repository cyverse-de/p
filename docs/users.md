# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [user.proto](#user-proto)
    - [Login](#user-Login)
    - [LoginIP](#user-LoginIP)
    - [LoginListWire](#user-LoginListWire)
    - [LoginStorage](#user-LoginStorage)
    - [LoginUserAgent](#user-LoginUserAgent)
    - [LoginWire](#user-LoginWire)
    - [Preferences](#user-Preferences)
    - [SavedSearches](#user-SavedSearches)
    - [SavedSearchesStorage](#user-SavedSearchesStorage)
    - [SavedSearchesWire](#user-SavedSearchesWire)
    - [User](#user-User)
    - [UserRef](#user-UserRef)
  
- [user_requests.proto](#user_requests-proto)
    - [AddLoginRequest](#user_requests-AddLoginRequest)
    - [DeleteSavedSearchesRequest](#user_requests-DeleteSavedSearchesRequest)
    - [DeleteUserPreferencesRequest](#user_requests-DeleteUserPreferencesRequest)
    - [GetLoginsRequest](#user_requests-GetLoginsRequest)
    - [GetSavedSearchesRequest](#user_requests-GetSavedSearchesRequest)
    - [GetUserPreferencesRequest](#user_requests-GetUserPreferencesRequest)
    - [InternalPaginationContinuationToken](#user_requests-InternalPaginationContinuationToken)
    - [LoginsResponse](#user_requests-LoginsResponse)
    - [PageSettings](#user_requests-PageSettings)
    - [SavedSearchesResponse](#user_requests-SavedSearchesResponse)
    - [SetSavedSearchesRequest](#user_requests-SetSavedSearchesRequest)
    - [SetUserPreferencesRequest](#user_requests-SetUserPreferencesRequest)
    - [UserLookupRequest](#user_requests-UserLookupRequest)
    - [UserLookupResponse](#user_requests-UserLookupResponse)
    - [UserPreferencesResponse](#user_requests-UserPreferencesResponse)
    - [UserRequest](#user_requests-UserRequest)
    - [UserResponse](#user_requests-UserResponse)
  
- [Scalar Value Types](#scalar-value-types)



<a name="user-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## user.proto



<a name="user-Login"></a>

### Login
Backwards compatibility version. Use LoginStorage and
LoginWire for new messages.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  |  |
| ip_address | [string](#string) |  |  |
| user_agent | [string](#string) |  |  |
| login_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| logout_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="user-LoginIP"></a>

### LoginIP



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [string](#string) |  |  |






<a name="user-LoginListWire"></a>

### LoginListWire
A list of logins that can go out over the wire.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| logins | [LoginWire](#user-LoginWire) | repeated |  |






<a name="user-LoginStorage"></a>

### LoginStorage
How a login is stored in the backend. Don&#39;t send this over the
wire.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | The UUID of the user. Not the username. Must be set. |
| ip_address | [string](#string) | optional | The IP address of the user that logged in. |
| user_agent | [string](#string) | optional | The user agent string of the user that logged in. |
| login_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional | The time the user logged in. If you&#39;re adding a login, this will be set automatically by the backend if it&#39;s not set. |
| logout_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional | The time the user logged out. Could be unset if the user hasn&#39;t logged out yet. |






<a name="user-LoginUserAgent"></a>

### LoginUserAgent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| full | [string](#string) |  |  |






<a name="user-LoginWire"></a>

### LoginWire
A wire-safe version of a login record.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [UserRef](#user-UserRef) |  |  |
| ip | [LoginIP](#user-LoginIP) | optional |  |
| user_agent | [LoginUserAgent](#user-LoginUserAgent) | optional |  |
| login_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The time the user logged in. If you&#39;re adding a login, this will be set automatically by the backend if it&#39;s not set. |
| logout_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional | The time the user logged out. Could be unset if the user hasn&#39;t logged out yet. |






<a name="user-Preferences"></a>

### Preferences
How the preferences are stored in the backend. Don&#39;t expose
this through a service/API.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  |  |
| preferences | [string](#string) |  | should come across as JSON. |






<a name="user-SavedSearches"></a>

### SavedSearches
Maintained for backwards compatibility. Use the Wire and Storage versions
for new messages


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  |  |
| saved_searches | [string](#string) |  |  |






<a name="user-SavedSearchesStorage"></a>

### SavedSearchesStorage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| saved_searches | [string](#string) |  |  |






<a name="user-SavedSearchesWire"></a>

### SavedSearchesWire



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [UserRef](#user-UserRef) |  |  |
| saved_searches | [string](#string) |  | Saved searches are stored in a JSON-encoded string. |






<a name="user-User"></a>

### User
A user&#39;s information. Represents how the user is stored.
Don&#39;t use this directly in Request messages.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  |  |
| username | [string](#string) |  |  |






<a name="user-UserRef"></a>

### UserRef
How a user can be referred to. Typically only one of them is
set. Can be used in Request messages


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| username | [string](#string) | optional | The username of the user in the database. Must be unique. It&#39;s more likely for a service to have this, which is why it&#39;s listed first. Writing services to use the username can skip a lookup of the UUID. |
| uuid | [string](#string) | optional | The UUID of the user in the database. A service can have this, but it&#39;s more likely for it to have the username. |





 

 

 

 



<a name="user_requests-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## user_requests.proto



<a name="user_requests-AddLoginRequest"></a>

### AddLoginRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| login | [user.LoginWire](#user-LoginWire) |  |  |






<a name="user_requests-DeleteSavedSearchesRequest"></a>

### DeleteSavedSearchesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |






<a name="user_requests-DeleteUserPreferencesRequest"></a>

### DeleteUserPreferencesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |






<a name="user_requests-GetLoginsRequest"></a>

### GetLoginsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |
| continuation | [string](#string) | optional |  |
| page | [PageSettings](#user_requests-PageSettings) | optional |  |






<a name="user_requests-GetSavedSearchesRequest"></a>

### GetSavedSearchesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |






<a name="user_requests-GetUserPreferencesRequest"></a>

### GetUserPreferencesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |






<a name="user_requests-InternalPaginationContinuationToken"></a>

### InternalPaginationContinuationToken
Don&#39;t expose this over the wire.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| offset | [int32](#int32) | optional |  |
| number | [int32](#int32) | optional |  |
| size | [int32](#int32) | optional |  |






<a name="user_requests-LoginsResponse"></a>

### LoginsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| list | [user.LoginListWire](#user-LoginListWire) | repeated |  |
| error | [svcerror.Error](#svcerror-Error) | optional |  |






<a name="user_requests-PageSettings"></a>

### PageSettings



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| offset | [int32](#int32) |  |  |
| number | [int32](#int32) |  |  |
| size | [int32](#int32) |  |  |






<a name="user_requests-SavedSearchesResponse"></a>

### SavedSearchesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| searches | [user.SavedSearchesWire](#user-SavedSearchesWire) | optional |  |
| error | [svcerror.Error](#svcerror-Error) | optional |  |






<a name="user_requests-SetSavedSearchesRequest"></a>

### SetSavedSearchesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |
| searches | [user.SavedSearchesWire](#user-SavedSearchesWire) |  |  |






<a name="user_requests-SetUserPreferencesRequest"></a>

### SetUserPreferencesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |
| preferences | [string](#string) |  |  |






<a name="user_requests-UserLookupRequest"></a>

### UserLookupRequest
A request for user information. This is kept for backwards compatibility,
you should really use the other request types.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| username | [string](#string) |  |  |
| user_id | [string](#string) |  |  |
| analysis_id | [string](#string) |  |  |
| include_logins | [bool](#bool) |  | Whether to include user logins in the response. |
| include_preferences | [bool](#bool) |  | Whether to include user preferences in the response. |
| include_saved_searches | [bool](#bool) |  | Whether to include saved searches in the response. |
| login_limit | [uint32](#uint32) |  | Paging limit. |
| login_offset | [uint32](#uint32) |  | Paging offset |
| header | [header.Header](#header-Header) |  | Contains telemetry information |






<a name="user_requests-UserLookupResponse"></a>

### UserLookupResponse
A response to a request for information about a single user.

Please use the other, smaller requests and responses.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  |  |
| basic_info | [user.User](#user-User) |  |  |
| logins | [user.Login](#user-Login) | repeated |  |
| preferences | [user.Preferences](#user-Preferences) |  |  |
| saved_searches | [user.SavedSearches](#user-SavedSearches) | repeated |  |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  |  |






<a name="user_requests-UserPreferencesResponse"></a>

### UserPreferencesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) | optional |  |
| preferences | [string](#string) | optional |  |
| error | [svcerror.Error](#svcerror-Error) | optional |  |






<a name="user_requests-UserRequest"></a>

### UserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |






<a name="user_requests-UserResponse"></a>

### UserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [user.UserRef](#user-UserRef) |  |  |
| error | [svcerror.Error](#svcerror-Error) |  |  |





 

 

 

 



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

