// Message types for the DE qms messages, formerly generated from qms_plans.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package qms

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/ptypes"
	"github.com/cyverse-de/p/go/svcerror"
)

// Represents a default quota value used in plans. Can be overridden on a
// per-user basis for a subscription to provide customized quotas. Also referred to
// as plan quota defaults.
type QuotaDefault struct {
	// The unique identifier/primary key for the quota default.
	Uuid string `json:"uuid"`
	// The value of the quota default.
	QuotaValue float64 `json:"quota_value"`
	// The resource type the quota applies to.
	ResourceType *ResourceType `json:"resource_type"`
	// The date that quota default becomes effective. There can be multiple quota defaults for the same resource type,
	// and the quota default that is currently effective is always the one with the most recent effective date that
	// occurs in the past.
	EffectiveDate *ptypes.Timestamp `json:"effective_date"`
}

func (x *QuotaDefault) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *QuotaDefault) GetQuotaValue() float64 {
	if x != nil {
		return x.QuotaValue
	}
	return 0
}

func (x *QuotaDefault) GetResourceType() *ResourceType {
	if x != nil {
		return x.ResourceType
	}
	return nil
}

func (x *QuotaDefault) GetEffectiveDate() *ptypes.Timestamp {
	if x != nil {
		return x.EffectiveDate
	}
	return nil
}

// A response type for quota default requests.
type QuotaDefaultResponse struct {
	// Can contain telemetry data.
	Header *header.Header `json:"header"`
	// Contains error info from the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The quota default object returned by the request handler.
	QuotaDefault *QuotaDefault `json:"quota_default"`
}

func (x *QuotaDefaultResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *QuotaDefaultResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *QuotaDefaultResponse) GetQuotaDefault() *QuotaDefault {
	if x != nil {
		return x.QuotaDefault
	}
	return nil
}

// A response type for quota default requests that contains a list of quota
// defaults.
type QuotaDefaultList struct {
	// Can contain telemetry data.
	Header *header.Header `json:"header"`
	// Contains error info from the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The list of quota default objects returned by the request handler.
	QuotaDefaults []*QuotaDefault `json:"quota_defaults"`
}

func (x *QuotaDefaultList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *QuotaDefaultList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *QuotaDefaultList) GetQuotaDefaults() []*QuotaDefault {
	if x != nil {
		return x.QuotaDefaults
	}
	return nil
}

// Represents a price for a subscription plan.
type PlanRate struct {
	// The unique identifier/primary key for the plan rate.
	Uuid string `json:"uuid"`
	// The subscription plan rate, that is the price for 1 year of service.
	Rate float64 `json:"rate"`
	// The date that the rate becomes effective. There can be multiple rates for a subscription, and the rate that is
	// currently effective is always the one with the most recent effective date that occurs in the past.
	EffectiveDate *ptypes.Timestamp `json:"effective_date"`
}

func (x *PlanRate) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *PlanRate) GetRate() float64 {
	if x != nil {
		return x.Rate
	}
	return 0
}

func (x *PlanRate) GetEffectiveDate() *ptypes.Timestamp {
	if x != nil {
		return x.EffectiveDate
	}
	return nil
}

// A response type for plan rate requests.
type PlanRateResponse struct {
	// Can contain telemetry data.
	Header *header.Header `json:"header"`
	// Contains error info from the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The plan rate object returned by the request handler.
	PlanRate *PlanRate `json:"plan_rate"`
}

func (x *PlanRateResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *PlanRateResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *PlanRateResponse) GetPlanRate() *PlanRate {
	if x != nil {
		return x.PlanRate
	}
	return nil
}

// A response type for plan rate requests that contains a list of plan rates.
type PlanRateList struct {
	// Can contain telemetry data.
	Header *header.Header `json:"header"`
	// Contains error info from the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The list of plan rate objects returned by the request handler.
	PlanRates []*PlanRate `json:"plan_rates"`
}

func (x *PlanRateList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *PlanRateList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *PlanRateList) GetPlanRates() []*PlanRate {
	if x != nil {
		return x.PlanRates
	}
	return nil
}

// Represents a subscription plan available to users.
type Plan struct {
	// Unique identifier for the plan.
	Uuid string `json:"uuid"`
	// The name of the plan.
	Name string `json:"name"`
	// A description of the plan.
	Description string `json:"description"`
	// A list of quota defaults associated with the plan. Each resource type can have more than one plan quota
	// default. The effective quota default for each resource type is the one with the most recent effective date that
	// occurs in the past.
	PlanQuotaDefaults []*QuotaDefault `json:"plan_quota_defaults"`
	// The list of plan rates. There may be multiple plan rates associated with the plan. The effective rate is always
	// the rate with the most recent effective date that occurs in the past.
	PlanRates []*PlanRate `json:"plan_rates"`
}

func (x *Plan) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *Plan) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Plan) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Plan) GetPlanQuotaDefaults() []*QuotaDefault {
	if x != nil {
		return x.PlanQuotaDefaults
	}
	return nil
}

func (x *Plan) GetPlanRates() []*PlanRate {
	if x != nil {
		return x.PlanRates
	}
	return nil
}

// A response to a plan request. Contains a single plan.
type PlanResponse struct {
	// Contains telemtry data.
	Header *header.Header `json:"header"`
	// Contains error data returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The plan returned by the request handler.
	Plan *Plan `json:"plan"`
}

func (x *PlanResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *PlanResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *PlanResponse) GetPlan() *Plan {
	if x != nil {
		return x.Plan
	}
	return nil
}

// A response to a plan request. Contains a list of plans.
type PlanList struct {
	// Contains telemetry data.
	Header *header.Header `json:"header"`
	// Contains error data returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// A list of plans returned by the request handler.
	Plans []*Plan `json:"plans"`
}

func (x *PlanList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *PlanList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *PlanList) GetPlans() []*Plan {
	if x != nil {
		return x.Plans
	}
	return nil
}

// A request for plan information specified by the plan's unique identifier.
type PlanRequest struct {
	// Contains telemetry data.
	Header *header.Header `json:"header"`
	// The unique identifier of the plan being requested.
	PlanId string `json:"plan_id"`
}

func (x *PlanRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *PlanRequest) GetPlanId() string {
	if x != nil {
		return x.PlanId
	}
	return ""
}

// A request to add a new plan to the system.
type AddPlanRequest struct {
	// Contains telemetry data.
	Header *header.Header `json:"header"`
	// The plan to add to the system.
	Plan *Plan `json:"plan"`
}

func (x *AddPlanRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AddPlanRequest) GetPlan() *Plan {
	if x != nil {
		return x.Plan
	}
	return nil
}

// A request to add a quota default to an existing plan.
type AddPlanQuotaDefaultRequest struct {
	// Contains telemetry data.
	Header *header.Header `json:"header"`
	// The name of the plan to add the quota default to.
	PlanName string `json:"plan_name"`
	// The quota default to add to the plan specified by the plan_name field.
	QuotaDefault *QuotaDefault `json:"quota_default"`
}

func (x *AddPlanQuotaDefaultRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AddPlanQuotaDefaultRequest) GetPlanName() string {
	if x != nil {
		return x.PlanName
	}
	return ""
}

func (x *AddPlanQuotaDefaultRequest) GetQuotaDefault() *QuotaDefault {
	if x != nil {
		return x.QuotaDefault
	}
	return nil
}

// A request to add a rate to an existing plan.
type AddPlanRateRequest struct {
	// Can contain telemetry data.
	Header *header.Header `json:"header"`
	// The name of the plan to add the rate to.
	PlanName string `json:"plan_name"`
	// The rate to add to the plan specified by the plan_name field.
	PlanRate *PlanRate `json:"plan_rate"`
}

func (x *AddPlanRateRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AddPlanRateRequest) GetPlanName() string {
	if x != nil {
		return x.PlanName
	}
	return ""
}

func (x *AddPlanRateRequest) GetPlanRate() *PlanRate {
	if x != nil {
		return x.PlanRate
	}
	return nil
}
