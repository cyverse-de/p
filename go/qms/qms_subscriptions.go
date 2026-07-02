// Message types for the DE qms messages, formerly generated from qms_subscriptions.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package qms

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/ptypes"
	"github.com/cyverse-de/p/go/svcerror"
)

// Representation of a subscription.
type Subscription struct {
	// The unique identifier
	Uuid string `json:"uuid"`
	// The date the subscription activates.
	EffectiveStartDate *ptypes.Timestamp `json:"effective_start_date"`
	// The date the subscription deactivates/expires.
	EffectiveEndDate *ptypes.Timestamp `json:"effective_end_date"`
	// The user in the QMS system that the subscription is for.
	User *QMSUser `json:"user"`
	// The plan the user is subscribed to.
	Plan *Plan `json:"plan"`
	// The list of quotas applied to the subscription. Initially populated
	// by quota defaults, but can be overridden.
	Quotas []*Quota `json:"quotas"`
	// The list of resource usages that the user has generated while this plan was active.
	Usages []*Usage `json:"usages"`
	// A flag indicating whether or not the user paid for the subscription.
	Paid bool `json:"paid"`
	// Information about the rate that was active when the subscription was purchased. Note that this rate is recorded
	// whether or not the user paid for the subscription directly.
	PlanRate *PlanRate `json:"plan_rate"`
	// The list of add-ons applied to the subscription.
	Addons []*SubscriptionAddon `json:"addons"`
}

func (x *Subscription) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *Subscription) GetEffectiveStartDate() *ptypes.Timestamp {
	if x != nil {
		return x.EffectiveStartDate
	}
	return nil
}

func (x *Subscription) GetEffectiveEndDate() *ptypes.Timestamp {
	if x != nil {
		return x.EffectiveEndDate
	}
	return nil
}

func (x *Subscription) GetUser() *QMSUser {
	if x != nil {
		return x.User
	}
	return nil
}

func (x *Subscription) GetPlan() *Plan {
	if x != nil {
		return x.Plan
	}
	return nil
}

func (x *Subscription) GetQuotas() []*Quota {
	if x != nil {
		return x.Quotas
	}
	return nil
}

func (x *Subscription) GetUsages() []*Usage {
	if x != nil {
		return x.Usages
	}
	return nil
}

func (x *Subscription) GetPaid() bool {
	if x != nil {
		return x.Paid
	}
	return false
}

func (x *Subscription) GetPlanRate() *PlanRate {
	if x != nil {
		return x.PlanRate
	}
	return nil
}

func (x *Subscription) GetAddons() []*SubscriptionAddon {
	if x != nil {
		return x.Addons
	}
	return nil
}

// A response to a request for a subscription.
type SubscriptionResponse struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// Error information returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The subscription returned by the request handler.
	Subscription *Subscription `json:"subscription"`
}

func (x *SubscriptionResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *SubscriptionResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *SubscriptionResponse) GetSubscription() *Subscription {
	if x != nil {
		return x.Subscription
	}
	return nil
}

// A response to a request for a list of subscriptions.
type SubscriptionList struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// Error information returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The subscription list returned by the request handler.
	Subscriptions []*Subscription `json:"subscriptions"`
}

func (x *SubscriptionList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *SubscriptionList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *SubscriptionList) GetSubscriptions() []*Subscription {
	if x != nil {
		return x.Subscriptions
	}
	return nil
}

// A request to change a subscription.
type ChangeSubscriptionRequest struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// A username for the user whose subscription is being changed.
	Username string  `json:"username"`
	Uuid     *string `json:"uuid,omitempty"`
	Name     *string `json:"name,omitempty"`
	// The number of subscription periods that the subscription will be good for. The subscription period is one year,
	// so purchasing a subscription for 3 periods will create a subscription for 3 years. Consumable resources are also
	// allocated based on the number of periods, so if a subscription plan comes with 2000 CPU Hours, for example, then
	// a user who purchases 3 subscription periods will get 6000 CPU hours to use over the course of three years.
	Periods int32 `json:"periods"`
	// The end-date of the subscription. Accepted formats are `YYYY-MM-DD`, `YYYY-MM-DDThh:mm:ss`,
	// `YYYY-MM-DDThh:mm:ssZ` and `YYYY-MM-DDThh:mm:ss+hh:mm`. Date and tiestamps without time zones are assumed to
	// be in the time zone used by the CyVerse Discovery Environment itself.
	EndDate string `json:"endDate"`
}

func (x *ChangeSubscriptionRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ChangeSubscriptionRequest) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *ChangeSubscriptionRequest) GetUuid() string {
	if x != nil && x.Uuid != nil {
		return *x.Uuid
	}
	return ""
}

func (x *ChangeSubscriptionRequest) GetName() string {
	if x != nil && x.Name != nil {
		return *x.Name
	}
	return ""
}

func (x *ChangeSubscriptionRequest) GetPeriods() int32 {
	if x != nil {
		return x.Periods
	}
	return 0
}

func (x *ChangeSubscriptionRequest) GetEndDate() string {
	if x != nil {
		return x.EndDate
	}
	return ""
}
