# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [qms_addons.proto](#qms_addons-proto)
    - [AddAddonRequest](#qms-AddAddonRequest)
    - [Addon](#qms-Addon)
    - [AddonListResponse](#qms-AddonListResponse)
    - [AddonLookupRequest](#qms-AddonLookupRequest)
    - [AddonRate](#qms-AddonRate)
    - [AddonRateList](#qms-AddonRateList)
    - [AddonRateResponse](#qms-AddonRateResponse)
    - [AddonResponse](#qms-AddonResponse)
    - [SubscriptionAddon](#qms-SubscriptionAddon)
    - [SubscriptionAddonListResponse](#qms-SubscriptionAddonListResponse)
    - [SubscriptionAddonResponse](#qms-SubscriptionAddonResponse)
    - [UpdateAddonRequest](#qms-UpdateAddonRequest)
    - [UpdateSubscriptionAddonRequest](#qms-UpdateSubscriptionAddonRequest)
  
- [qms_overages.proto](#qms_overages-proto)
    - [IsOverage](#qms-IsOverage)
    - [Overage](#qms-Overage)
    - [OverageList](#qms-OverageList)
    - [OverageResponse](#qms-OverageResponse)
  
- [qms_plans.proto](#qms_plans-proto)
    - [AddPlanQuotaDefaultRequest](#qms-AddPlanQuotaDefaultRequest)
    - [AddPlanRateRequest](#qms-AddPlanRateRequest)
    - [AddPlanRequest](#qms-AddPlanRequest)
    - [Plan](#qms-Plan)
    - [PlanList](#qms-PlanList)
    - [PlanRate](#qms-PlanRate)
    - [PlanRateList](#qms-PlanRateList)
    - [PlanRateResponse](#qms-PlanRateResponse)
    - [PlanRequest](#qms-PlanRequest)
    - [PlanResponse](#qms-PlanResponse)
    - [QuotaDefault](#qms-QuotaDefault)
    - [QuotaDefaultList](#qms-QuotaDefaultList)
    - [QuotaDefaultResponse](#qms-QuotaDefaultResponse)
  
- [qms_quotas.proto](#qms_quotas-proto)
    - [AddQuotaRequest](#qms-AddQuotaRequest)
    - [Quota](#qms-Quota)
    - [QuotaList](#qms-QuotaList)
    - [QuotaResponse](#qms-QuotaResponse)
  
- [qms_requests.proto](#qms_requests-proto)
    - [AddUsage](#qms-AddUsage)
    - [AllUserOveragesRequest](#qms-AllUserOveragesRequest)
    - [GetUsages](#qms-GetUsages)
    - [IsOverageRequest](#qms-IsOverageRequest)
    - [NoParamsRequest](#qms-NoParamsRequest)
    - [RequestByUserID](#qms-RequestByUserID)
    - [RequestByUsername](#qms-RequestByUsername)
    - [UserResourceOveragesRequest](#qms-UserResourceOveragesRequest)
  
- [qms_resource_types.proto](#qms_resource_types-proto)
    - [ResourceType](#qms-ResourceType)
    - [ResourceTypeList](#qms-ResourceTypeList)
    - [ResourceTypeResponse](#qms-ResourceTypeResponse)
  
- [qms_subscriptions.proto](#qms_subscriptions-proto)
    - [ChangeSubscriptionRequest](#qms-ChangeSubscriptionRequest)
    - [Subscription](#qms-Subscription)
    - [SubscriptionList](#qms-SubscriptionList)
    - [SubscriptionResponse](#qms-SubscriptionResponse)
  
- [qms_updates.proto](#qms_updates-proto)
    - [AddUpdateRequest](#qms-AddUpdateRequest)
    - [AddUpdateResponse](#qms-AddUpdateResponse)
    - [Update](#qms-Update)
    - [UpdateListRequest](#qms-UpdateListRequest)
    - [UpdateListResponse](#qms-UpdateListResponse)
    - [UpdateOperation](#qms-UpdateOperation)
  
- [qms_usages.proto](#qms_usages-proto)
    - [Usage](#qms-Usage)
    - [UsageList](#qms-UsageList)
    - [UsageResponse](#qms-UsageResponse)
  
- [qms_users.proto](#qms_users-proto)
    - [AddUserRequest](#qms-AddUserRequest)
    - [AddUserResponse](#qms-AddUserResponse)
    - [QMSUser](#qms-QMSUser)
    - [QMSUserList](#qms-QMSUserList)
    - [QMSUserResponse](#qms-QMSUserResponse)
  
- [Scalar Value Types](#scalar-value-types)



<a name="qms_addons-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_addons.proto



<a name="qms-AddAddonRequest"></a>

### AddAddonRequest
A request to add an add-on to the system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information. |
| addon | [Addon](#qms-Addon) |  | The add-on to be added. |






<a name="qms-Addon"></a>

### Addon
Represents an add-on that can be applied to a subscription to provide a user
with a change in a resource quota.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier. |
| name | [string](#string) |  | The name of the add-on. |
| description | [string](#string) |  | The description of the add-on. |
| resource_type | [ResourceType](#qms-ResourceType) |  | The resource type of the add-on. |
| default_amount | [double](#double) |  | How much of the resource type is added to the quota by the add-on. |
| default_paid | [bool](#bool) |  | Whether a user must pay for the add-on. Not whether the user has paid. |
| addon_rates | [AddonRate](#qms-AddonRate) | repeated | The list of addon rates. An addon may have multiple rates; the one that is effective at any given time is always the rate with the most recent effective date that occurs in the past. |






<a name="qms-AddonListResponse"></a>

### AddonListResponse
A response to an add-on request that contains a list of add-ons.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| addons | [Addon](#qms-Addon) | repeated | The list of add-ons returned by the request handler. |






<a name="qms-AddonLookupRequest"></a>

### AddonLookupRequest
A request to get information about an add-on.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information. |
| uuid | [string](#string) |  |  |
| name | [string](#string) |  |  |






<a name="qms-AddonRate"></a>

### AddonRate
Represents the rate charged for an addon as the prcie for one year of service.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier. |
| rate | [double](#double) |  | The rate. |
| effective_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at any given time is always the rate with the most recent effective date that occurs in the past. |






<a name="qms-AddonRateList"></a>

### AddonRateList
A response type for addon rate requests that contains a list of addon rates.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Can contain telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error info from the request handler. |
| addon_rates | [AddonRate](#qms-AddonRate) | repeated | The list of addon rate objects returned by the request handler. |






<a name="qms-AddonRateResponse"></a>

### AddonRateResponse
A response type for addon rate requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Can contain telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error info from the request handler. |
| addon_rate | [AddonRate](#qms-AddonRate) |  | The addon rate object returned by the request handler. |






<a name="qms-AddonResponse"></a>

### AddonResponse
A response to an add-on request. Contains a single add-on.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| addon | [Addon](#qms-Addon) |  | The add-on returned by the request handler. |






<a name="qms-SubscriptionAddon"></a>

### SubscriptionAddon
SubscriptionAddon is an add-on that has been applied to a subscription. It
contains fields that can override the the default_amount and default_paid
fields in the subscription.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier for the add-on |
| addon | [Addon](#qms-Addon) |  | The add-on used with the subscription. May only contain the add-on&#39;s UUID in some circumstances. |
| subscription | [Subscription](#qms-Subscription) |  | The subscription the add-on was applied to. May only contain the add-on&#39;s UUID in some circumstances. |
| amount | [double](#double) |  | The amount of the resource applied by the add-on. This should default to the amount contained in the add-on definition, but can be overridden, which is why it&#39;s a separate field here. |
| paid | [bool](#bool) |  | Whether the subscription add-on costs money. This should default to the same paid value contained in the add-on definition, but can be overridden, which is why it&#39;s a separate field here. |
| addon_rate | [AddonRate](#qms-AddonRate) |  | The amount per year that we expect to have been charged if the user paid for the add-on. |






<a name="qms-SubscriptionAddonListResponse"></a>

### SubscriptionAddonListResponse
Contains a list of subscription add-ons returned by the request handler.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| subscription_addons | [SubscriptionAddon](#qms-SubscriptionAddon) | repeated | The list of subscription add-ons returned by the request handler. |






<a name="qms-SubscriptionAddonResponse"></a>

### SubscriptionAddonResponse
Contains the subscription add-on returned by the request handler.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| subscription_addon | [SubscriptionAddon](#qms-SubscriptionAddon) |  | The subscription add-on returned by the request handler. |






<a name="qms-UpdateAddonRequest"></a>

### UpdateAddonRequest
A request to update an add-on. The boolean fields are needed because Go (and
probably other languages) needs a way to tell when to set a field to an empty
string, since that&#39;s the zero value for a string.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information. |
| addon | [Addon](#qms-Addon) |  | The values to set in the update. |
| update_name | [bool](#bool) |  | Whether to update the name of the addon. |
| update_description | [bool](#bool) |  | Whether to update the description of the addon. |
| update_resource_type | [bool](#bool) |  | Whether to update the resource type of the addon. |
| update_default_amount | [bool](#bool) |  | Whether to update the default amount of the addon. |
| update_default_paid | [bool](#bool) |  | Whether to update the default paid field of the addon. |
| update_addon_rates | [bool](#bool) |  | Whether to update the addon rates. |






<a name="qms-UpdateSubscriptionAddonRequest"></a>

### UpdateSubscriptionAddonRequest
Contains the information needed to update a subscription add-on.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information. |
| subscription_addon | [SubscriptionAddon](#qms-SubscriptionAddon) |  | The subscription add-on information being updated. Does not necessarily have all fields set and the UUID field should not be set. |
| update_addon_id | [bool](#bool) |  | Whether to update the addon_id with the value contained in the subscription addon. The DE backend currently does not support this. Do a delete-&gt;add instead. |
| update_subscription_id | [bool](#bool) |  | Whether to update the subscription_id field with the value contained in the subscription addon. The DE backend currently does not support this. Do a delete-&gt;add instead. |
| update_amount | [bool](#bool) |  | Whether to update the amount field with the value contained in the subscription addon. |
| update_paid | [bool](#bool) |  | Whether to update the paid fields with the value contained in the subscription addon. |
| addon_rate | [AddonRate](#qms-AddonRate) |  | The amount per year that we expect to have been charged if the user paid for the add-on. |





 

 

 

 



<a name="qms_overages-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_overages.proto



<a name="qms-IsOverage"></a>

### IsOverage
A response message returned by handlers in response to overage related requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | The header userd for passing telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains any errors generated by the handler emitting the response. |
| is_overage | [bool](#bool) |  | Whether or not there is an overage. |






<a name="qms-Overage"></a>

### Overage
Represents when a user&#39;s resource type usage exceeds their configured
quota. Usually embedded in request and response message types.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| resource_name | [string](#string) |  | The type of resource that is in overage. Usually data.size of cpu.hours. |
| quota | [double](#double) |  | The configured quota value for the resource type. |
| usage | [double](#double) |  | The actual usage value for the resource type. |






<a name="qms-OverageList"></a>

### OverageList
A response message returned by handlers in response to overage related requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | The header used for passing telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains any errors generated by the handler emitting the response. |
| overages | [Overage](#qms-Overage) | repeated | The list of overages returned by the handler emitting the response. |






<a name="qms-OverageResponse"></a>

### OverageResponse
Returned by handlers in response to overage requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | The header used for passing telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains any errors generated by the handler emitting the response. |
| overage | [Overage](#qms-Overage) |  | The overage returned by the handler emitting the response. |





 

 

 

 



<a name="qms_plans-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_plans.proto



<a name="qms-AddPlanQuotaDefaultRequest"></a>

### AddPlanQuotaDefaultRequest
A request to add a quota default to an existing plan.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| plan_name | [string](#string) |  | The name of the plan to add the quota default to. |
| quota_default | [QuotaDefault](#qms-QuotaDefault) |  | The quota default to add to the plan specified by the plan_name field. |






<a name="qms-AddPlanRateRequest"></a>

### AddPlanRateRequest
A request to add a rate to an existing plan.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Can contain telemetry data. |
| plan_name | [string](#string) |  | The name of the plan to add the rate to. |
| plan_rate | [PlanRate](#qms-PlanRate) |  | The rate to add to the plan specified by the plan_name field. |






<a name="qms-AddPlanRequest"></a>

### AddPlanRequest
A request to add a new plan to the system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| plan | [Plan](#qms-Plan) |  | The plan to add to the system. |






<a name="qms-Plan"></a>

### Plan
Represents a subscription plan available to users.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | Unique identifier for the plan. |
| name | [string](#string) |  | The name of the plan. |
| description | [string](#string) |  | A description of the plan. |
| plan_quota_defaults | [QuotaDefault](#qms-QuotaDefault) | repeated | A list of quota defaults associated with the plan. Each resource type can have more than one plan quota default. The effective quota default for each resource type is the one with the most recent effective date that occurs in the past. |
| plan_rates | [PlanRate](#qms-PlanRate) | repeated | The list of plan rates. There may be multiple plan rates associated with the plan. The effective rate is always the rate with the most recent effective date that occurs in the past. |






<a name="qms-PlanList"></a>

### PlanList
A response to a plan request. Contains a list of plans.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error data returned by the request handler. |
| plans | [Plan](#qms-Plan) | repeated | A list of plans returned by the request handler. |






<a name="qms-PlanRate"></a>

### PlanRate
Represents a price for a subscription plan.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier/primary key for the plan rate. |
| rate | [double](#double) |  | The subscription plan rate, that is the price for 1 year of service. |
| effective_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date that the rate becomes effective. There can be multiple rates for a subscription, and the rate that is currently effective is always the one with the most recent effective date that occurs in the past. |






<a name="qms-PlanRateList"></a>

### PlanRateList
A response type for plan rate requests that contains a list of plan rates.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Can contain telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error info from the request handler. |
| plan_rates | [PlanRate](#qms-PlanRate) | repeated | The list of plan rate objects returned by the request handler. |






<a name="qms-PlanRateResponse"></a>

### PlanRateResponse
A response type for plan rate requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Can contain telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error info from the request handler. |
| plan_rate | [PlanRate](#qms-PlanRate) |  | The plan rate object returned by the request handler. |






<a name="qms-PlanRequest"></a>

### PlanRequest
A request for plan information specified by the plan&#39;s unique identifier.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| plan_id | [string](#string) |  | The unique identifier of the plan being requested. |






<a name="qms-PlanResponse"></a>

### PlanResponse
A response to a plan request. Contains a single plan.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemtry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error data returned by the request handler. |
| plan | [Plan](#qms-Plan) |  | The plan returned by the request handler. |






<a name="qms-QuotaDefault"></a>

### QuotaDefault
Represents a default quota value used in plans. Can be overridden on a
per-user basis for a subscription to provide customized quotas. Also referred to
as plan quota defaults.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier/primary key for the quota default. |
| quota_value | [double](#double) |  | The value of the quota default. |
| resource_type | [ResourceType](#qms-ResourceType) |  | The resource type the quota applies to. |
| effective_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date that quota default becomes effective. There can be multiple quota defaults for the same resource type, and the quota default that is currently effective is always the one with the most recent effective date that occurs in the past. |






<a name="qms-QuotaDefaultList"></a>

### QuotaDefaultList
A response type for quota default requests that contains a list of quota
defaults.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Can contain telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error info from the request handler. |
| quota_defaults | [QuotaDefault](#qms-QuotaDefault) | repeated | The list of quota default objects returned by the request handler. |






<a name="qms-QuotaDefaultResponse"></a>

### QuotaDefaultResponse
A response type for quota default requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Can contain telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Contains error info from the request handler. |
| quota_default | [QuotaDefault](#qms-QuotaDefault) |  | The quota default object returned by the request handler. |





 

 

 

 



<a name="qms_quotas-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_quotas.proto



<a name="qms-AddQuotaRequest"></a>

### AddQuotaRequest
A request to add a quota to a subscription.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| quota | [Quota](#qms-Quota) |  | The quota to be added to the system. |






<a name="qms-Quota"></a>

### Quota
Represents a quota in the system, which is the currently configured limit on
a resource type a user has associated with their plan. Overrides the quota
default associated with the plan the user has.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier. |
| quota | [double](#double) |  | The quota value (aka limit). |
| resource_type | [ResourceType](#qms-ResourceType) |  | The resource type the quota value applies to. |
| CreatedBy | [string](#string) |  | A freeform text field containing info about who created the quota. |
| CreatedAt | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the quota was created. |
| LastModifiedBy | [string](#string) |  | A freeform text field containing info about who last modified the quota. |
| LastModifiedAt | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the quota was last modified. |
| subscription_id | [string](#string) |  | The unique identifier of the subscription that the quota is associated with. |






<a name="qms-QuotaList"></a>

### QuotaList
A response to a quota request containing a list of quotas.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| quotas | [Quota](#qms-Quota) | repeated | A list of quotas returned by the request handler. |






<a name="qms-QuotaResponse"></a>

### QuotaResponse
A response to a quota request. Contains a single quota object.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| quota | [Quota](#qms-Quota) |  | The quota returned by the request handler. |





 

 

 

 



<a name="qms_requests-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_requests.proto



<a name="qms-AddUsage"></a>

### AddUsage
A request to add a usage to the system for a resource type consumed by the
specified user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| username | [string](#string) |  |  |
| resource_name | [string](#string) |  |  |
| update_type | [string](#string) |  | Possible values are defined in the database, so we can&#39;t use an enum here |
| usage_value | [double](#double) |  |  |
| resource_unit | [string](#string) |  |  |






<a name="qms-AllUserOveragesRequest"></a>

### AllUserOveragesRequest
A request for all of a user&#39;s current resource type overages.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| username | [string](#string) |  | The user&#39;s username in the QMS system. |






<a name="qms-GetUsages"></a>

### GetUsages



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| username | [string](#string) |  |  |






<a name="qms-IsOverageRequest"></a>

### IsOverageRequest
A request to check if a user is in overage for a particular resource.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| username | [string](#string) |  | A username. |
| resource_name | [string](#string) |  | The name of the resource type to check for usage overages by the user. |






<a name="qms-NoParamsRequest"></a>

### NoParamsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |






<a name="qms-RequestByUserID"></a>

### RequestByUserID



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| user_id | [string](#string) |  |  |






<a name="qms-RequestByUsername"></a>

### RequestByUsername



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| username | [string](#string) |  |  |






<a name="qms-UserResourceOveragesRequest"></a>

### UserResourceOveragesRequest
A request for a user&#39;s overages specific to a particular resource type.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry data. |
| username | [string](#string) |  | A user&#39;s username. |
| resource_name | [string](#string) |  | The name of the resource type to look up overages for. |





 

 

 

 



<a name="qms_resource_types-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_resource_types.proto



<a name="qms-ResourceType"></a>

### ResourceType
Representation of a resource type.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier. |
| name | [string](#string) |  | The name of the resource. Will usually be &#34;data.size&#34; and &#34;cpu.hours&#34;. |
| unit | [string](#string) |  | The units used for the resource. Usually &#34;bytes&#34; or &#34;cpu hours&#34;. |
| consumable | [bool](#bool) |  | Whether or not using the resource consumes a portion of the allocation permanently. |






<a name="qms-ResourceTypeList"></a>

### ResourceTypeList
A response type for the resource type requests that contains a list of 
resource type definitions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| resource_types | [ResourceType](#qms-ResourceType) | repeated | A list of resource types returned by the request handler. |






<a name="qms-ResourceTypeResponse"></a>

### ResourceTypeResponse
A response type for resource type requests.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| resource_type | [ResourceType](#qms-ResourceType) |  | The resource type returned by the request handler. |





 

 

 

 



<a name="qms_subscriptions-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_subscriptions.proto



<a name="qms-ChangeSubscriptionRequest"></a>

### ChangeSubscriptionRequest
A request to change a subscription.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| username | [string](#string) |  | A username for the user whose subscription is being changed. |
| uuid | [string](#string) |  |  |
| name | [string](#string) |  |  |
| periods | [int32](#int32) |  | The number of subscription periods that the subscription will be good for. The subscription period is one year, so purchasing a subscription for 3 periods will create a subscription for 3 years. Consumable resources are also allocated based on the number of periods, so if a subscription plan comes with 2000 CPU Hours, for example, then a user who purchases 3 subscription periods will get 6000 CPU hours to use over the course of three years. |
| end_date | [string](#string) |  | The end-date of the subscription. Accepted formats are `YYYY-MM-DD`, `YYYY-MM-DDThh:mm:ss`, `YYYY-MM-DDThh:mm:ssZ` and `YYYY-MM-DDThh:mm:ss&#43;hh:mm`. Date and tiestamps without time zones are assumed to be in the time zone used by the CyVerse Discovery Environment itself. |






<a name="qms-Subscription"></a>

### Subscription
Representation of a subscription.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier |
| effective_start_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date the subscription activates. |
| effective_end_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date the subscription deactivates/expires. |
| user | [QMSUser](#qms-QMSUser) |  | The user in the QMS system that the subscription is for. |
| plan | [Plan](#qms-Plan) |  | The plan the user is subscribed to. |
| quotas | [Quota](#qms-Quota) | repeated | The list of quotas applied to the subscription. Initially populated by quota defaults, but can be overridden. |
| usages | [Usage](#qms-Usage) | repeated | The list of resource usages that the user has generated while this plan was active. |
| paid | [bool](#bool) |  | A flag indicating whether or not the user paid for the subscription. |
| plan_rate | [PlanRate](#qms-PlanRate) |  | Information about the rate that was active when the subscription was purchased. Note that this rate is recorded whether or not the user paid for the subscription directly. |






<a name="qms-SubscriptionList"></a>

### SubscriptionList
A response to a request for a list of subscriptions.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| subscriptions | [Subscription](#qms-Subscription) | repeated | The subscription list returned by the request handler. |






<a name="qms-SubscriptionResponse"></a>

### SubscriptionResponse
A response to a request for a subscription.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| subscription | [Subscription](#qms-Subscription) |  | The subscription returned by the request handler. |





 

 

 

 



<a name="qms_updates-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_updates.proto



<a name="qms-AddUpdateRequest"></a>

### AddUpdateRequest
A request to add an update to the system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| update | [Update](#qms-Update) |  | The update being added to the system. |






<a name="qms-AddUpdateResponse"></a>

### AddUpdateResponse
A response to requests to add an update to the system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| update | [Update](#qms-Update) |  | The update added to the system. |






<a name="qms-Update"></a>

### Update
A representation of an update to a quota or usage value.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier |
| value_type | [string](#string) |  | Determines whether the update is for a &#34;quota&#34; or &#34;usage&#34;. |
| value | [double](#double) |  | The value being applied to the usage or quota. |
| effective_date | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The date the update takes effect. |
| operation | [UpdateOperation](#qms-UpdateOperation) |  | The type of operation being done. |
| resource_type | [ResourceType](#qms-ResourceType) |  | The resource type for the quota or usage being updated. |
| user | [QMSUser](#qms-QMSUser) |  | The user in the QMS system that the update is for. |






<a name="qms-UpdateListRequest"></a>

### UpdateListRequest
A request to get the list of updates generated by the specified user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| user | [QMSUser](#qms-QMSUser) |  | The user whose updates have been requested. |






<a name="qms-UpdateListResponse"></a>

### UpdateListResponse
A response containing the requested list of updates generated by a user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| updates | [Update](#qms-Update) | repeated | The list of updates returned by the request handler. |






<a name="qms-UpdateOperation"></a>

### UpdateOperation
A representation of update operations, which are ways calling code can change
quota and usage value.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier |
| name | [string](#string) |  | The name of the update operation |





 

 

 

 



<a name="qms_usages-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_usages.proto



<a name="qms-Usage"></a>

### Usage
A representation of how much a user has used a resource type.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | The unique identifier |
| usage | [double](#double) |  | How much the resource has been used. |
| subscription_id | [string](#string) |  | The unique identifier for the subscription the usage is associated with. |
| resource_type | [ResourceType](#qms-ResourceType) |  | The resource type the usage applies to. |
| CreatedBy | [string](#string) |  | Who created the usage record. Probably not the name of a user. |
| CreatedAt | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the usage record was created. |
| LastModifiedBy | [string](#string) |  | Who last modified the usage record. Probably not the name of a user. |
| LastModifiedAt | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | When the usage record was last modified. |






<a name="qms-UsageList"></a>

### UsageList
A response to a request for usage info containing multiple usage records.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| usages | [Usage](#qms-Usage) | repeated | A list of usages returned by the request handler. |






<a name="qms-UsageResponse"></a>

### UsageResponse
A response to a request for a usage record.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information returned by the request handler. |
| usage | [Usage](#qms-Usage) |  | Contains the usage info returned by the request handler. |





 

 

 

 



<a name="qms_users-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## qms_users.proto



<a name="qms-AddUserRequest"></a>

### AddUserRequest
A request to add a user to the QMS system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| username | [string](#string) |  | The username for the user being added to the system. |
| plan_name | [string](#string) |  | The name of the plan the user should be subscribed to. |
| paid | [bool](#bool) |  | True if the user paid for the subscription. |
| periods | [int32](#int32) |  | The number of subscription periods that the subscription will be good for. The subscription period is one year, so purchasing a subscription for 3 periods will create a subscription for 3 years. Consumable resources are also allocated based on the number of periods, so if a subscription plan comes with 2000 CPU Hours, for example, then a user who purchases 3 subscription periods will get 6000 CPU hours to use over the course of three years. |
| end_date | [string](#string) |  | The end-date of the subscription. Accepted formats are `YYYY-MM-DD`, `YYYY-MM-DDThh:mm:ss`, `YYYY-MM-DDThh:mm:ssZ` and `YYYY-MM-DDThh:mm:ss&#43;hh:mm`. Date and tiestamps without time zones are assumed to be in the time zone used by the CyVerse Discovery Environment itself. |
| force | [bool](#bool) |  | True if the user should get a new subscription even if they already have a subscription with the same plan. |






<a name="qms-AddUserResponse"></a>

### AddUserResponse
A response to a request to add a user to the QMS system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| uuid | [string](#string) |  | The unique identfier of the newly added user. |
| username | [string](#string) |  | The username of the newly added user. |
| plan_name | [string](#string) |  | The name of the plan the newly added user is subscribed to. |
| plan_uuid | [string](#string) |  | The unique identifier for the plan the newly added user is subscribed to. |






<a name="qms-QMSUser"></a>

### QMSUser
A representation of a user in the QMS system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| uuid | [string](#string) |  | A user&#39;s unique identifier in QMS. |
| username | [string](#string) |  | A user&#39;s username in QMS. |






<a name="qms-QMSUserList"></a>

### QMSUserList
A response to a request for info about a list of users.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| users | [QMSUser](#qms-QMSUser) | repeated | The user list returned by the request handler. |






<a name="qms-QMSUserResponse"></a>

### QMSUserResponse
A response to a request for info about a QMS user.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [header.Header](#header-Header) |  | Contains telemetry information |
| error | [svcerror.ServiceError](#svcerror-ServiceError) |  | Error information generated by the request handler. |
| user | [QMSUser](#qms-QMSUser) |  | The user returned by the request handler. |





 

 

 

 



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

