// Message types for the DE monitoring messages, formerly generated from monitoring_heartbeat.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package monitoring

import (
	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/svcerror"
)

type Heartbeat struct {
	Header   *header.Header         `json:"header"`
	Error    *svcerror.ServiceError `json:"error"`
	Node     string                 `json:"node"`
	DateSent string                 `json:"date_sent"`
}

func (x *Heartbeat) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *Heartbeat) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *Heartbeat) GetNode() string {
	if x != nil {
		return x.Node
	}
	return ""
}

func (x *Heartbeat) GetDateSent() string {
	if x != nil {
		return x.DateSent
	}
	return ""
}
