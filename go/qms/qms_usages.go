// Message types for the DE qms messages, formerly generated from qms_usages.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package qms

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/ptypes"
	"github.com/cyverse-de/p/go/svcerror"
)

// A representation of how much a user has used a resource type.
type Usage struct {
	// The unique identifier
	Uuid string `json:"uuid"`
	// How much the resource has been used.
	Usage float64 `json:"usage"`
	// The unique identifier for the subscription the usage is associated with.
	SubscriptionId string `json:"subscription_id"`
	// The resource type the usage applies to.
	ResourceType *ResourceType `json:"resource_type"`
	// Who created the usage record. Probably not the name of a user.
	CreatedBy string `json:"created_by"`
	// When the usage record was created.
	CreatedAt *ptypes.Timestamp `json:"created_at"`
	// Who last modified the usage record. Probably not the name of a user.
	LastModifiedBy string `json:"last_modified_by"`
	// When the usage record was last modified.
	LastModifiedAt *ptypes.Timestamp `json:"last_modified_at"`
}

func (x *Usage) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *Usage) GetUsage() float64 {
	if x != nil {
		return x.Usage
	}
	return 0
}

func (x *Usage) GetSubscriptionId() string {
	if x != nil {
		return x.SubscriptionId
	}
	return ""
}

func (x *Usage) GetResourceType() *ResourceType {
	if x != nil {
		return x.ResourceType
	}
	return nil
}

func (x *Usage) GetCreatedBy() string {
	if x != nil {
		return x.CreatedBy
	}
	return ""
}

func (x *Usage) GetCreatedAt() *ptypes.Timestamp {
	if x != nil {
		return x.CreatedAt
	}
	return nil
}

func (x *Usage) GetLastModifiedBy() string {
	if x != nil {
		return x.LastModifiedBy
	}
	return ""
}

func (x *Usage) GetLastModifiedAt() *ptypes.Timestamp {
	if x != nil {
		return x.LastModifiedAt
	}
	return nil
}

// A response to a request for a usage record.
type UsageResponse struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// Error information returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// Contains the usage info returned by the request handler.
	Usage *Usage `json:"usage"`
}

func (x *UsageResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *UsageResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *UsageResponse) GetUsage() *Usage {
	if x != nil {
		return x.Usage
	}
	return nil
}

// A response to a request for usage info containing multiple usage records.
type UsageList struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// Error information returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// A list of usages returned by the request handler.
	Usages []*Usage `json:"usages"`
}

func (x *UsageList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *UsageList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *UsageList) GetUsages() []*Usage {
	if x != nil {
		return x.Usages
	}
	return nil
}
