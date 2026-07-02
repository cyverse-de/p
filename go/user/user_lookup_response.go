// The response message for the discoenv user-lookup requests. This is the
// wire shape of the original full user.User message (removed from user.proto
// after go/user/v0.0.10 but still exchanged by discoenv-users and its
// clients); the goldens for it were captured from that version.

package user

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/svcerror"
)

// A user's information, as returned by discoenv-users.
type LookupResponse struct {
	Uuid     string `json:"uuid"`
	Username string `json:"username"`
	// The user's preferences; requested with include_preferences.
	Preferences *Preferences `json:"preferences"`
	// The user's login history; requested with include_logins.
	Logins []*Login `json:"logins"`
	// The total number of logins recorded for the user.
	LoginCount uint32 `json:"loginCount"`
	// The user's saved searches; requested with include_saved_searches.
	SavedSearches *SavedSearches `json:"savedSearches"`
	// Contains telemetry information
	Header *header.Header `json:"header"`
	// Error information returned by the request handler.
	Error *svcerror.ServiceError `json:"error"`
}

func (x *LookupResponse) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *LookupResponse) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *LookupResponse) GetPreferences() *Preferences {
	if x != nil {
		return x.Preferences
	}
	return nil
}

func (x *LookupResponse) GetLogins() []*Login {
	if x != nil {
		return x.Logins
	}
	return nil
}

func (x *LookupResponse) GetLoginCount() uint32 {
	if x != nil {
		return x.LoginCount
	}
	return 0
}

func (x *LookupResponse) GetSavedSearches() *SavedSearches {
	if x != nil {
		return x.SavedSearches
	}
	return nil
}

func (x *LookupResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *LookupResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}
