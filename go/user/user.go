// Message types for the DE user messages, formerly generated from user.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package user

import (
	"github.com/cyverse-de/p/go/ptypes"
)

type Preferences struct {
	Uuid        string `json:"uuid"`
	Preferences string `json:"preferences"`
}

func (x *Preferences) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *Preferences) GetPreferences() string {
	if x != nil {
		return x.Preferences
	}
	return ""
}

type Login struct {
	Uuid       string            `json:"uuid"`
	IpAddress  string            `json:"ipAddress"`
	UserAgent  string            `json:"userAgent"`
	LoginTime  *ptypes.Timestamp `json:"loginTime"`
	LogoutTime *ptypes.Timestamp `json:"logoutTime"`
}

func (x *Login) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *Login) GetIpAddress() string {
	if x != nil {
		return x.IpAddress
	}
	return ""
}

func (x *Login) GetUserAgent() string {
	if x != nil {
		return x.UserAgent
	}
	return ""
}

func (x *Login) GetLoginTime() *ptypes.Timestamp {
	if x != nil {
		return x.LoginTime
	}
	return nil
}

func (x *Login) GetLogoutTime() *ptypes.Timestamp {
	if x != nil {
		return x.LogoutTime
	}
	return nil
}

type SavedSearches struct {
	Uuid          string `json:"uuid"`
	SavedSearches string `json:"savedSearches"`
}

func (x *SavedSearches) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *SavedSearches) GetSavedSearches() string {
	if x != nil {
		return x.SavedSearches
	}
	return ""
}

// A user's information.
type User struct {
	Uuid     string `json:"uuid"`
	Username string `json:"username"`
}

func (x *User) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *User) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}
