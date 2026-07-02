// Message types for the DE qms messages, formerly generated from qms_resource_types.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package qms

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/svcerror"
)

// Representation of a resource type.
type ResourceType struct {
	// The unique identifier.
	Uuid string `json:"uuid"`
	// The name of the resource. Will usually be "data.size" and "cpu.hours".
	Name string `json:"name"`
	// The units used for the resource. Usually "bytes" or "cpu hours".
	Unit string `json:"unit"`
	// Whether or not using the resource consumes a portion of the allocation permanently.
	Consumable bool `json:"consumable"`
}

func (x *ResourceType) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *ResourceType) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *ResourceType) GetUnit() string {
	if x != nil {
		return x.Unit
	}
	return ""
}

func (x *ResourceType) GetConsumable() bool {
	if x != nil {
		return x.Consumable
	}
	return false
}

// A response type for resource type requests.
type ResourceTypeResponse struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// Error information returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// The resource type returned by the request handler.
	ResourceType *ResourceType `json:"resource_type"`
}

func (x *ResourceTypeResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ResourceTypeResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *ResourceTypeResponse) GetResourceType() *ResourceType {
	if x != nil {
		return x.ResourceType
	}
	return nil
}

// A response type for the resource type requests that contains a list of
// resource type definitions.
type ResourceTypeList struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// Error information returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
	// A list of resource types returned by the request handler.
	ResourceTypes []*ResourceType `json:"resource_types"`
}

func (x *ResourceTypeList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ResourceTypeList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *ResourceTypeList) GetResourceTypes() []*ResourceType {
	if x != nil {
		return x.ResourceTypes
	}
	return nil
}
