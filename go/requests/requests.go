// Message types for the DE requests messages, formerly generated from requests.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package requests

import (
	"github.com/cyverse-de/p/go/header"
)

// Request a resource by the username of a user.
type ByUsername struct {
	// Contains telemetry information
	Header   *header.Header `json:"header"`
	Username string         `json:"username"`
}

func (x *ByUsername) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUsername) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

// Request a resource by the user ID of a user.
type ByUserID struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	UserId string         `json:"user_id"`
}

func (x *ByUserID) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUserID) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

// Send a message that does not request any parameters. Common for triggering
// side-effects or for retrieving lists of resources as an administrator.
type NoParams struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
}

func (x *NoParams) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

// Request a resource by its UUID.
type ByUUID struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// The UUID of the resource being requested.
	Uuid string `json:"uuid"`
}

func (x *ByUUID) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUUID) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

// Request a resource by its UUID and a username. Useful in situations where a
// user's ability to access a resource needs to be checked as part of the
// request handler logic.
type ByUUIDAndUsername struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// The UUID of the resource being requested
	Uuid string `json:"uuid"`
	// The username associated with the request.
	Username string `json:"username"`
}

func (x *ByUUIDAndUsername) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUUIDAndUsername) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *ByUUIDAndUsername) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

// Request a resource by its UUID and a user's UUID. Useful when the user's
// access to the resource must be verified.
type ByUUIDAndUserID struct {
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// The UUID of the resource being requested
	Uuid string `json:"uuid"`
	// The user ID of the user associated with the request.
	UserId string `json:"user_id"`
}

func (x *ByUUIDAndUserID) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUUIDAndUserID) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *ByUUIDAndUserID) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

// Request that two resources be associated.
type AssociateByUUIDs struct {
	// Contains telemetry information.
	Header *header.Header `json:"header"`
	// The UUID of the parent/owner/primary resource.
	ParentUuid string `json:"parent_uuid"`
	// The UUID of the child/object/secondary resource.
	ChildUuid string `json:"child_uuid"`
}

func (x *AssociateByUUIDs) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AssociateByUUIDs) GetParentUuid() string {
	if x != nil {
		return x.ParentUuid
	}
	return ""
}

func (x *AssociateByUUIDs) GetChildUuid() string {
	if x != nil {
		return x.ChildUuid
	}
	return ""
}
