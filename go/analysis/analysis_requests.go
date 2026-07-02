// Message types for the DE analysis messages, formerly generated from analysis_requests.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package analysis

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/ptypes"
	"github.com/cyverse-de/p/go/svcerror"
)

type AnalysisRecordLookupRequest struct {
	AnalysisId     *string        `json:"analysisId,omitempty"`
	ExternalId     *string        `json:"externalId,omitempty"`
	UserId         *string        `json:"userId,omitempty"`
	Username       *string        `json:"username,omitempty"`
	Header         *header.Header `json:"header"`
	RequestingUser string         `json:"requestingUser"`
}

func (x *AnalysisRecordLookupRequest) GetAnalysisId() string {
	if x != nil && x.AnalysisId != nil {
		return *x.AnalysisId
	}
	return ""
}

func (x *AnalysisRecordLookupRequest) GetExternalId() string {
	if x != nil && x.ExternalId != nil {
		return *x.ExternalId
	}
	return ""
}

func (x *AnalysisRecordLookupRequest) GetUserId() string {
	if x != nil && x.UserId != nil {
		return *x.UserId
	}
	return ""
}

func (x *AnalysisRecordLookupRequest) GetUsername() string {
	if x != nil && x.Username != nil {
		return *x.Username
	}
	return ""
}

func (x *AnalysisRecordLookupRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AnalysisRecordLookupRequest) GetRequestingUser() string {
	if x != nil {
		return x.RequestingUser
	}
	return ""
}

type AnalysisRecordResponse struct {
	Header      *header.Header                              `json:"header"`
	Analyses    []*AnalysisRecord                           `json:"analyses"`
	Timestamp   string                                      `json:"timestamp"`
	Total       ptypes.Int64                                `json:"total"`
	StatusCount []*AnalysisRecordResponse_StatusCountRecord `json:"status-count"`
	Error       *svcerror.ServiceError                      `json:"error"`
}

func (x *AnalysisRecordResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AnalysisRecordResponse) GetAnalyses() []*AnalysisRecord {
	if x != nil {
		return x.Analyses
	}
	return nil
}

func (x *AnalysisRecordResponse) GetTimestamp() string {
	if x != nil {
		return x.Timestamp
	}
	return ""
}

func (x *AnalysisRecordResponse) GetTotal() ptypes.Int64 {
	if x != nil {
		return x.Total
	}
	return 0
}

func (x *AnalysisRecordResponse) GetStatusCount() []*AnalysisRecordResponse_StatusCountRecord {
	if x != nil {
		return x.StatusCount
	}
	return nil
}

func (x *AnalysisRecordResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

type AnalysisRecordResponse_StatusCountRecord struct {
	Count  ptypes.Int64 `json:"count"`
	Status string       `json:"status"`
}

func (x *AnalysisRecordResponse_StatusCountRecord) GetCount() ptypes.Int64 {
	if x != nil {
		return x.Count
	}
	return 0
}

func (x *AnalysisRecordResponse_StatusCountRecord) GetStatus() string {
	if x != nil {
		return x.Status
	}
	return ""
}

type AnalysisRecordList struct {
	Header   *header.Header         `json:"header"`
	Analyses []*AnalysisRecord      `json:"analyses"`
	Error    *svcerror.ServiceError `json:"error"`
}

func (x *AnalysisRecordList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AnalysisRecordList) GetAnalyses() []*AnalysisRecord {
	if x != nil {
		return x.Analyses
	}
	return nil
}

func (x *AnalysisRecordList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}
