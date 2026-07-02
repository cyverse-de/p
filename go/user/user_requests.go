// Message types for the DE user messages, formerly generated from user_requests.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package user

import (
	"github.com/cyverse-de/p/go/header"
)

// A request for user information.
type UserLookupRequest struct {
	Username   *string `json:"username,omitempty"`
	UserId     *string `json:"userId,omitempty"`
	AnalysisId *string `json:"analysisId,omitempty"`
	// Whether to include user logins in the response.
	IncludeLogins bool `json:"includeLogins"`
	// Whether to include user preferences in the response.
	IncludePreferences bool `json:"includePreferences"`
	// Whether to include saved searches in the response.
	IncludeSavedSearches bool `json:"includeSavedSearches"`
	// Paging limit.
	LoginLimit uint32 `json:"loginLimit"`
	// Paging offset
	LoginOffset uint32 `json:"loginOffset"`
	// Contains telemetry information
	Header *header.Header `json:"header"`
}

func (x *UserLookupRequest) GetUsername() string {
	if x != nil && x.Username != nil {
		return *x.Username
	}
	return ""
}

func (x *UserLookupRequest) GetUserId() string {
	if x != nil && x.UserId != nil {
		return *x.UserId
	}
	return ""
}

func (x *UserLookupRequest) GetAnalysisId() string {
	if x != nil && x.AnalysisId != nil {
		return *x.AnalysisId
	}
	return ""
}

func (x *UserLookupRequest) GetIncludeLogins() bool {
	if x != nil {
		return x.IncludeLogins
	}
	return false
}

func (x *UserLookupRequest) GetIncludePreferences() bool {
	if x != nil {
		return x.IncludePreferences
	}
	return false
}

func (x *UserLookupRequest) GetIncludeSavedSearches() bool {
	if x != nil {
		return x.IncludeSavedSearches
	}
	return false
}

func (x *UserLookupRequest) GetLoginLimit() uint32 {
	if x != nil {
		return x.LoginLimit
	}
	return 0
}

func (x *UserLookupRequest) GetLoginOffset() uint32 {
	if x != nil {
		return x.LoginOffset
	}
	return 0
}

func (x *UserLookupRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}
