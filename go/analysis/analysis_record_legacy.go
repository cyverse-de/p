// The analysis record exchanged by discoenv-analyses: the shape of the apps
// service's /analyses HTTP responses, relayed over NATS in
// AnalysisRecordResponse and AnalysisRecordList. This message was replaced by
// Analysis in analysis_record.proto after go/analysis/v0.0.12, but the
// deployed services still exchange this shape; the goldens for it were
// captured from that version.

package analysis

import (
	"github.com/cyverse-de/p/go/header"
)

// A single analysis record, as served by the apps service.
type AnalysisRecord struct {
	// Contains telemetry information
	Header      *header.Header `json:"header"`
	Id          string         `json:"id"`
	Description string         `json:"description"`
	Name        string         `json:"name"`
	CanShare    bool           `json:"can_share"`
	Username    string         `json:"username"`
	AppId       string         `json:"app_id"`
	// Batch-analysis progress counts.
	BatchStatus     *BatchStatus `json:"batch_status"`
	SystemId        string       `json:"system_id"`
	AppDisabled     bool         `json:"app_disabled"`
	Batch           bool         `json:"batch"`
	Enddate         string       `json:"enddate"`
	Startdate       string       `json:"startdate"`
	AppDescription  string       `json:"app_description"`
	InteractiveUrls []string     `json:"interactive_urls"`
	WikiUrl         string       `json:"wiki_url"`
	Notify          bool         `json:"notify"`
	ResultFolderId  string       `json:"resultfolderid"`
	AppName         string       `json:"app_name"`
}

func (x *AnalysisRecord) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AnalysisRecord) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *AnalysisRecord) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *AnalysisRecord) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *AnalysisRecord) GetCanShare() bool {
	if x != nil {
		return x.CanShare
	}
	return false
}

func (x *AnalysisRecord) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *AnalysisRecord) GetAppId() string {
	if x != nil {
		return x.AppId
	}
	return ""
}

func (x *AnalysisRecord) GetBatchStatus() *BatchStatus {
	if x != nil {
		return x.BatchStatus
	}
	return nil
}

func (x *AnalysisRecord) GetSystemId() string {
	if x != nil {
		return x.SystemId
	}
	return ""
}

func (x *AnalysisRecord) GetAppDisabled() bool {
	if x != nil {
		return x.AppDisabled
	}
	return false
}

func (x *AnalysisRecord) GetBatch() bool {
	if x != nil {
		return x.Batch
	}
	return false
}

func (x *AnalysisRecord) GetEnddate() string {
	if x != nil {
		return x.Enddate
	}
	return ""
}

func (x *AnalysisRecord) GetStartdate() string {
	if x != nil {
		return x.Startdate
	}
	return ""
}

func (x *AnalysisRecord) GetAppDescription() string {
	if x != nil {
		return x.AppDescription
	}
	return ""
}

func (x *AnalysisRecord) GetInteractiveUrls() []string {
	if x != nil {
		return x.InteractiveUrls
	}
	return nil
}

func (x *AnalysisRecord) GetWikiUrl() string {
	if x != nil {
		return x.WikiUrl
	}
	return ""
}

func (x *AnalysisRecord) GetNotify() bool {
	if x != nil {
		return x.Notify
	}
	return false
}

func (x *AnalysisRecord) GetResultFolderId() string {
	if x != nil {
		return x.ResultFolderId
	}
	return ""
}

func (x *AnalysisRecord) GetAppName() string {
	if x != nil {
		return x.AppName
	}
	return ""
}
