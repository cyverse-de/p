// Message types for the DE analysis messages, formerly generated from analysis_status.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package analysis

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/svcerror"
)

type AnalysisStatus struct {
	Header  *header.Header         `json:"header"`
	Error   *svcerror.ServiceError `json:"error"`
	Job     *AnalysisSubmission    `json:"job"`
	Version int32                  `json:"version"`
	State   string                 `json:"state"`
	Message string                 `json:"message"`
	SentOn  string                 `json:"sent_on"`
	Sender  string                 `json:"sender"`
}

func (x *AnalysisStatus) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AnalysisStatus) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *AnalysisStatus) GetJob() *AnalysisSubmission {
	if x != nil {
		return x.Job
	}
	return nil
}

func (x *AnalysisStatus) GetVersion() int32 {
	if x != nil {
		return x.Version
	}
	return 0
}

func (x *AnalysisStatus) GetState() string {
	if x != nil {
		return x.State
	}
	return ""
}

func (x *AnalysisStatus) GetMessage() string {
	if x != nil {
		return x.Message
	}
	return ""
}

func (x *AnalysisStatus) GetSentOn() string {
	if x != nil {
		return x.SentOn
	}
	return ""
}

func (x *AnalysisStatus) GetSender() string {
	if x != nil {
		return x.Sender
	}
	return ""
}
