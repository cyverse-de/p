package ptypes

import (
	"bytes"
	"strconv"
)

// Int64 is an int64 that tolerates both JSON encodings in use for 64-bit
// integers: protojson emitted them as strings while plain encoding/json (the
// apps HTTP responses, the old subscriptions HTTP handlers) emitted numbers.
// It always marshals as a number, which protojson-based peers also accept.
type Int64 int64

func (x Int64) MarshalJSON() ([]byte, error) {
	return strconv.AppendInt(nil, int64(x), 10), nil
}

func (x *Int64) UnmarshalJSON(b []byte) error {
	b = bytes.TrimSpace(b)
	if string(b) == "null" {
		*x = 0
		return nil
	}
	if len(b) >= 2 && b[0] == '"' && b[len(b)-1] == '"' {
		b = b[1 : len(b)-1]
	}
	v, err := strconv.ParseInt(string(b), 10, 64)
	if err != nil {
		return err
	}
	*x = Int64(v)
	return nil
}
