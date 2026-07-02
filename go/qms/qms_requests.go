// Message types for the DE qms messages, formerly generated from qms_requests.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package qms

import (
	"github.com/cyverse-de/p/go/header"
)

// A request for all of a user's current resource type overages.
type AllUserOveragesRequest struct {
	// Contains telemetry data.
	Header *header.Header `json:"header"`
	// The user's username in the QMS system.
	Username string `json:"username"`
}

func (x *AllUserOveragesRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AllUserOveragesRequest) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

// A request for a user's overages specific to a particular resource type.
type UserResourceOveragesRequest struct {
	// Contains telemetry data.
	Header *header.Header `json:"header"`
	// A user's username.
	Username string `json:"username"`
	// The name of the resource type to look up overages for.
	ResourceName string `json:"resource_name"`
}

func (x *UserResourceOveragesRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *UserResourceOveragesRequest) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *UserResourceOveragesRequest) GetResourceName() string {
	if x != nil {
		return x.ResourceName
	}
	return ""
}

// A request to check if a user is in overage for a particular resource.
type IsOverageRequest struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// A username.
	Username string `json:"username"`
	// The name of the resource type to check for usage overages by the user.
	ResourceName string `json:"resource_name"`
}

func (x *IsOverageRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *IsOverageRequest) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *IsOverageRequest) GetResourceName() string {
	if x != nil {
		return x.ResourceName
	}
	return ""
}

// A request to add a usage to the system for a resource type consumed by the
// specified user.
type AddUsage struct {
	// Contains telemetry information
	Header       *header.Header `json:"header"`
	Username     string         `json:"username"`
	ResourceName string         `json:"resource_name"`
	// Possible values are defined in the database, so we can't use an enum here
	UpdateType   string  `json:"update_type"`
	UsageValue   float64 `json:"usage_value"`
	ResourceUnit string  `json:"resource_unit"`
}

func (x *AddUsage) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AddUsage) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *AddUsage) GetResourceName() string {
	if x != nil {
		return x.ResourceName
	}
	return ""
}

func (x *AddUsage) GetUpdateType() string {
	if x != nil {
		return x.UpdateType
	}
	return ""
}

func (x *AddUsage) GetUsageValue() float64 {
	if x != nil {
		return x.UsageValue
	}
	return 0
}

func (x *AddUsage) GetResourceUnit() string {
	if x != nil {
		return x.ResourceUnit
	}
	return ""
}

type GetUsages struct {
	// Contains telemetry information
	Header   *header.Header `json:"header"`
	Username string         `json:"username"`
}

func (x *GetUsages) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *GetUsages) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

type RequestByUsername struct {
	// Contains telemetry information
	Header   *header.Header `json:"header"`
	Username string         `json:"username"`
}

func (x *RequestByUsername) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *RequestByUsername) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

type RequestByUserID struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	UserId string         `json:"user_id"`
}

func (x *RequestByUserID) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *RequestByUserID) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

type NoParamsRequest struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
}

func (x *NoParamsRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}
