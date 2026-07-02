// Message types for the DE apps messages, formerly generated from apps.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package apps

import (
	"github.com/cyverse-de/p/go/ptypes"
	"github.com/cyverse-de/p/go/user"
)

// An app in the system.
//
// An app is a collection of one or more tools that run in an environment to produce outputs.
type App struct {
	// The UUID for the app.
	Id string `json:"id"`
	// The name of the app.
	Name string `json:"name"`
	// The description of the app.
	Description string `json:"description"`
	// The wiki URL of the app.
	WikiUrl string `json:"wiki_url"`
}

func (x *App) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *App) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *App) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *App) GetWikiUrl() string {
	if x != nil {
		return x.WikiUrl
	}
	return ""
}

// Information about when something was integrated.
type IntegrationData struct {
	// The UUID of the integration data.
	Id string `json:"id"`
	// The name of the person that integrated stuff.
	IntegratorName string `json:"integrator_name"`
	// The email of the person that integrated stuff.
	IntegratorEmail string `json:"integrator_email"`
	// The user information of the integrator.
	User *user.User `json:"user"`
}

func (x *IntegrationData) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *IntegrationData) GetIntegratorName() string {
	if x != nil {
		return x.IntegratorName
	}
	return ""
}

func (x *IntegrationData) GetIntegratorEmail() string {
	if x != nil {
		return x.IntegratorEmail
	}
	return ""
}

func (x *IntegrationData) GetUser() *user.User {
	if x != nil {
		return x.User
	}
	return nil
}

// App version information.
type AppVersion struct {
	// The UUID for the app version information.
	Id string `json:"id"`
	// The UUID of the app the version information applies to.
	AppId string `json:"app_id"`
	// The string representation of the version.
	Version string `json:"version"`
	// The ordering of the version.
	VersionOrder ptypes.Int64 `json:"version_order"`
	// Whether the version is deleted.
	Deleted bool `json:"deleted"`
	// Whether the version is disabled.
	Disabled bool `json:"disabled"`
	// Integration information for this version.
	Integration *IntegrationData `json:"integration"`
	// The date the version was integrated.
	IntegrationDate *ptypes.Timestamp `json:"integration_date"`
	// The date the app version was last edited.
	EditedDate *ptypes.Timestamp `json:"edited_date"`
}

func (x *AppVersion) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *AppVersion) GetAppId() string {
	if x != nil {
		return x.AppId
	}
	return ""
}

func (x *AppVersion) GetVersion() string {
	if x != nil {
		return x.Version
	}
	return ""
}

func (x *AppVersion) GetVersionOrder() ptypes.Int64 {
	if x != nil {
		return x.VersionOrder
	}
	return 0
}

func (x *AppVersion) GetDeleted() bool {
	if x != nil {
		return x.Deleted
	}
	return false
}

func (x *AppVersion) GetDisabled() bool {
	if x != nil {
		return x.Disabled
	}
	return false
}

func (x *AppVersion) GetIntegration() *IntegrationData {
	if x != nil {
		return x.Integration
	}
	return nil
}

func (x *AppVersion) GetIntegrationDate() *ptypes.Timestamp {
	if x != nil {
		return x.IntegrationDate
	}
	return nil
}

func (x *AppVersion) GetEditedDate() *ptypes.Timestamp {
	if x != nil {
		return x.EditedDate
	}
	return nil
}
