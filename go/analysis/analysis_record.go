// Message types for the DE analysis messages, formerly generated from analysis_record.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package analysis

import (
	"github.com/cyverse-de/p/go/apps"
	"github.com/cyverse-de/p/go/ptypes"
	"github.com/cyverse-de/p/go/user"
)

type BatchStatus struct {
	Total     ptypes.Int64 `json:"total"`
	Completed ptypes.Int64 `json:"completed"`
	Running   ptypes.Int64 `json:"running"`
	Submitted ptypes.Int64 `json:"submitted"`
}

func (x *BatchStatus) GetTotal() ptypes.Int64 {
	if x != nil {
		return x.Total
	}
	return 0
}

func (x *BatchStatus) GetCompleted() ptypes.Int64 {
	if x != nil {
		return x.Completed
	}
	return 0
}

func (x *BatchStatus) GetRunning() ptypes.Int64 {
	if x != nil {
		return x.Running
	}
	return 0
}

func (x *BatchStatus) GetSubmitted() ptypes.Int64 {
	if x != nil {
		return x.Submitted
	}
	return 0
}

// Corresponds to the job_types table in the database.
type AnalysisType struct {
	// The UUID for the analysis type.
	Id string `json:"id"`
	// The name of the analysis type.
	Name string `json:"name"`
	// The kind of system the analysis should run on.
	SystemId string `json:"system_id"`
}

func (x *AnalysisType) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *AnalysisType) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *AnalysisType) GetSystemId() string {
	if x != nil {
		return x.SystemId
	}
	return ""
}

// An analysis in the system.
//
// An analysis is an app that was run by a user.
type Analysis struct {
	// The UUID for the analysis
	Id string `json:"id"`
	// The description of the analysis provided by the user.
	Description string `json:"description"`
	// The name of the analysis provided by the user.
	Name string `json:"name"`
	// App information about the analysis.
	App *apps.App `json:"app"`
	// App version information for the analysis.
	AppVersion *apps.AppVersion `json:"app_version"`
	// The analysis type, which tells which environment to run the analysis in.
	Kind *AnalysisType `json:"type"`
	// The path to the folder containing analysis outputs.
	ResultFolderPath string `json:"result_folder_path"`
	// The date the analysis was submitted.
	StartDate *ptypes.Timestamp `json:"start_date"`
	// The date the analyses finished.
	EndDate *ptypes.Timestamp `json:"end_date"`
	// The date the analysis was scheduled to end.
	PlannedEndDate *ptypes.Timestamp `json:"planned_end_date"`
	// The status of the analysis
	Status string `json:"status"`
	// Whether the analysis was deleted.
	Deleted bool `json:"deleted"`
	// Whether notifications should be emitted on status changes.
	Notify bool `json:"notify"`
	// The user that submitted the analysis.
	User *user.User `json:"user"`
	// The subdomain assigned to the analysis for VICE.
	Subdomain string `json:"subdomain"`
	// The UUID of the analysis that spawned this analysis. Used for batch analyses.
	ParentId string `json:"parent_id"`
	// The number of millicores reserved for the analysis.
	MillicoresReserved float64 `json:"millicore_reserved"`
}

func (x *Analysis) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Analysis) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Analysis) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Analysis) GetApp() *apps.App {
	if x != nil {
		return x.App
	}
	return nil
}

func (x *Analysis) GetAppVersion() *apps.AppVersion {
	if x != nil {
		return x.AppVersion
	}
	return nil
}

func (x *Analysis) GetKind() *AnalysisType {
	if x != nil {
		return x.Kind
	}
	return nil
}

func (x *Analysis) GetResultFolderPath() string {
	if x != nil {
		return x.ResultFolderPath
	}
	return ""
}

func (x *Analysis) GetStartDate() *ptypes.Timestamp {
	if x != nil {
		return x.StartDate
	}
	return nil
}

func (x *Analysis) GetEndDate() *ptypes.Timestamp {
	if x != nil {
		return x.EndDate
	}
	return nil
}

func (x *Analysis) GetPlannedEndDate() *ptypes.Timestamp {
	if x != nil {
		return x.PlannedEndDate
	}
	return nil
}

func (x *Analysis) GetStatus() string {
	if x != nil {
		return x.Status
	}
	return ""
}

func (x *Analysis) GetDeleted() bool {
	if x != nil {
		return x.Deleted
	}
	return false
}

func (x *Analysis) GetNotify() bool {
	if x != nil {
		return x.Notify
	}
	return false
}

func (x *Analysis) GetUser() *user.User {
	if x != nil {
		return x.User
	}
	return nil
}

func (x *Analysis) GetSubdomain() string {
	if x != nil {
		return x.Subdomain
	}
	return ""
}

func (x *Analysis) GetParentId() string {
	if x != nil {
		return x.ParentId
	}
	return ""
}

func (x *Analysis) GetMillicoresReserved() float64 {
	if x != nil {
		return x.MillicoresReserved
	}
	return 0
}
