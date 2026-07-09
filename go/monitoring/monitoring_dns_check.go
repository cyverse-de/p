// Message types for the DE monitoring messages, formerly generated from monitoring_dns_check.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package monitoring

import (
	"strconv"

	"github.com/cyverse-de/p/go/header"
	"github.com/cyverse-de/p/go/ptypes"
	"github.com/cyverse-de/p/go/svcerror"
)

type LookupType int32

const (
	LookupType_UNSET_LOOKUP    LookupType = 0
	LookupType_INTERNAL_LOOKUP LookupType = 1
	LookupType_EXTERNAL_LOOKUP LookupType = 2
)

var LookupType_name = map[int32]string{
	0: "UNSET_LOOKUP",
	1: "INTERNAL_LOOKUP",
	2: "EXTERNAL_LOOKUP",
}

var LookupType_value = map[string]int32{
	"UNSET_LOOKUP":    0,
	"INTERNAL_LOOKUP": 1,
	"EXTERNAL_LOOKUP": 2,
}

func (x LookupType) String() string {
	if s, ok := LookupType_name[int32(x)]; ok {
		return s
	}
	return strconv.Itoa(int(x))
}

// MarshalJSON emits the value's name, as protojson did for enums.
func (x LookupType) MarshalJSON() ([]byte, error) {
	return ptypes.MarshalEnumName(LookupType_name, int32(x))
}

// UnmarshalJSON accepts either a value name or a number.
func (x *LookupType) UnmarshalJSON(b []byte) error {
	v, err := ptypes.UnmarshalEnumName(LookupType_value, "LookupType", b)
	if err != nil {
		return err
	}
	*x = LookupType(v)
	return nil
}

type DNSLookup struct {
	Host      string   `json:"host"`
	Addresses []string `json:"addresses"`
	Type      string   `json:"type"`
	Error     string   `json:"error"`
}

func (x *DNSLookup) GetHost() string {
	if x != nil {
		return x.Host
	}
	return ""
}

func (x *DNSLookup) GetAddresses() []string {
	if x != nil {
		return x.Addresses
	}
	return nil
}

func (x *DNSLookup) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *DNSLookup) GetError() string {
	if x != nil {
		return x.Error
	}
	return ""
}

type DNSCheckResult struct {
	Header   *header.Header         `json:"header"`
	Error    *svcerror.ServiceError `json:"error"`
	Lookups  []*DNSLookup           `json:"lookups"`
	Node     string                 `json:"node"`
	DateSent string                 `json:"date_sent"`
}

func (x *DNSCheckResult) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *DNSCheckResult) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *DNSCheckResult) GetLookups() []*DNSLookup {
	if x != nil {
		return x.Lookups
	}
	return nil
}

func (x *DNSCheckResult) GetNode() string {
	if x != nil {
		return x.Node
	}
	return ""
}

func (x *DNSCheckResult) GetDateSent() string {
	if x != nil {
		return x.DateSent
	}
	return ""
}
