// Package ptypes provides the scalar helper types shared by the hand-written
// DE message types, preserving the wire formats previously produced by
// protojson and by encoding/json over the protobuf-generated structs.
package ptypes

import (
	"bytes"
	"encoding/json"
	"fmt"
	"time"
)

// Timestamp marshals as an RFC 3339 UTC string, exactly as protojson encoded
// google.protobuf.Timestamp. It additionally accepts the
// {"seconds":...,"nanos":...} object form that encoding/json produced for the
// generated timestamppb type, so responses from not-yet-migrated services
// still decode.
type Timestamp struct {
	time.Time
}

// New returns a Timestamp for the given time, normalized to UTC.
func New(t time.Time) *Timestamp {
	return &Timestamp{t.UTC()}
}

// Now returns the current time as a Timestamp.
func Now() *Timestamp {
	return New(time.Now())
}

// AsTime returns the timestamp as a time.Time. Like timestamppb, a nil
// receiver yields the Unix epoch rather than panicking.
func (x *Timestamp) AsTime() time.Time {
	if x == nil {
		return time.Unix(0, 0).UTC()
	}
	return x.Time
}

func (x Timestamp) MarshalJSON() ([]byte, error) {
	return json.Marshal(x.Time.UTC().Format(time.RFC3339Nano))
}

func (x *Timestamp) UnmarshalJSON(b []byte) error {
	b = bytes.TrimSpace(b)
	if len(b) == 0 || string(b) == "null" {
		x.Time = time.Time{}
		return nil
	}
	if b[0] == '"' {
		var s string
		if err := json.Unmarshal(b, &s); err != nil {
			return err
		}
		t, err := time.Parse(time.RFC3339Nano, s)
		if err != nil {
			return fmt.Errorf("ptypes.Timestamp: %w", err)
		}
		x.Time = t.UTC()
		return nil
	}
	var obj struct {
		Seconds int64 `json:"seconds"`
		Nanos   int32 `json:"nanos"`
	}
	if err := json.Unmarshal(b, &obj); err != nil {
		return fmt.Errorf("ptypes.Timestamp: expected RFC 3339 string or {seconds,nanos} object: %w", err)
	}
	x.Time = time.Unix(obj.Seconds, int64(obj.Nanos)).UTC()
	return nil
}
