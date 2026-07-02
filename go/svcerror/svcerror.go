// Message types for the DE svcerror messages, formerly generated from svcerror.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package svcerror

import (
	"encoding/json"
	"fmt"
	"strconv"
)

// The types of errors that can be retuned by message handlers.
type ErrorCode int32

const (
	ErrorCode_UNSET             ErrorCode = 0
	ErrorCode_UNSPECIFIED       ErrorCode = 1
	ErrorCode_INTERNAL          ErrorCode = 2
	ErrorCode_NOT_FOUND         ErrorCode = 3
	ErrorCode_BAD_REQUEST       ErrorCode = 4
	ErrorCode_MARSHAL_FAILURE   ErrorCode = 5
	ErrorCode_UNMARSHAL_FAILURE ErrorCode = 6
	ErrorCode_PARAMETER_MISSING ErrorCode = 7
	ErrorCode_PARAMETER_INVALID ErrorCode = 8
	ErrorCode_UNAUTHENTICATED   ErrorCode = 9
	ErrorCode_FORBIDDEN         ErrorCode = 10
	ErrorCode_TIMEOUT           ErrorCode = 11
	ErrorCode_UNSUPPORTED       ErrorCode = 12
	ErrorCode_UNIMPLEMENTED     ErrorCode = 13
)

var ErrorCode_name = map[int32]string{
	0:  "UNSET",
	1:  "UNSPECIFIED",
	2:  "INTERNAL",
	3:  "NOT_FOUND",
	4:  "BAD_REQUEST",
	5:  "MARSHAL_FAILURE",
	6:  "UNMARSHAL_FAILURE",
	7:  "PARAMETER_MISSING",
	8:  "PARAMETER_INVALID",
	9:  "UNAUTHENTICATED",
	10: "FORBIDDEN",
	11: "TIMEOUT",
	12: "UNSUPPORTED",
	13: "UNIMPLEMENTED",
}

var ErrorCode_value = map[string]int32{
	"UNSET":             0,
	"UNSPECIFIED":       1,
	"INTERNAL":          2,
	"NOT_FOUND":         3,
	"BAD_REQUEST":       4,
	"MARSHAL_FAILURE":   5,
	"UNMARSHAL_FAILURE": 6,
	"PARAMETER_MISSING": 7,
	"PARAMETER_INVALID": 8,
	"UNAUTHENTICATED":   9,
	"FORBIDDEN":         10,
	"TIMEOUT":           11,
	"UNSUPPORTED":       12,
	"UNIMPLEMENTED":     13,
}

func (x ErrorCode) String() string {
	if s, ok := ErrorCode_name[int32(x)]; ok {
		return s
	}
	return strconv.Itoa(int(x))
}

// MarshalJSON emits the value's name, as protojson did for enums.
func (x ErrorCode) MarshalJSON() ([]byte, error) {
	if s, ok := ErrorCode_name[int32(x)]; ok {
		return json.Marshal(s)
	}
	return json.Marshal(int32(x))
}

// UnmarshalJSON accepts either a value name or a number.
func (x *ErrorCode) UnmarshalJSON(b []byte) error {
	if len(b) > 0 && b[0] == '"' {
		var s string
		if err := json.Unmarshal(b, &s); err != nil {
			return err
		}
		v, ok := ErrorCode_value[s]
		if !ok {
			return fmt.Errorf("unknown ErrorCode %q", s)
		}
		*x = ErrorCode(v)
		return nil
	}
	var v int32
	if err := json.Unmarshal(b, &v); err != nil {
		return err
	}
	*x = ErrorCode(v)
	return nil
}

// An error returned by a request handler.
type ServiceError struct {
	// The numeric error code from the error code enum.
	ErrorCode ErrorCode `json:"error_code"`
	// The status code for the error.
	StatusCode int32 `json:"status_code"`
	// The error's message.
	Message string `json:"message"`
}

func (x *ServiceError) GetErrorCode() ErrorCode {
	if x != nil {
		return x.ErrorCode
	}
	return 0
}

func (x *ServiceError) GetStatusCode() int32 {
	if x != nil {
		return x.StatusCode
	}
	return 0
}

func (x *ServiceError) GetMessage() string {
	if x != nil {
		return x.Message
	}
	return ""
}
