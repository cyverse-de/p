package ptypes

import (
	"encoding/json"
	"testing"
	"time"
)

func TestTimestampUnmarshal(t *testing.T) {
	want := time.Date(2024, 5, 15, 12, 34, 56, 789000000, time.UTC)
	tests := []struct {
		name string
		in   string
		want time.Time
	}{
		{"rfc3339 with millis", `"2024-05-15T12:34:56.789Z"`, want},
		{"rfc3339 seconds only", `"2024-05-15T12:34:56Z"`, want.Truncate(time.Second)},
		{"rfc3339 with offset", `"2024-05-15T05:34:56.789-07:00"`, want},
		{"seconds and nanos object", `{"seconds":1715776496,"nanos":789000000}`, want},
		{"seconds only object", `{"seconds":1715776496}`, want.Truncate(time.Second)},
		{"empty object", `{}`, time.Unix(0, 0).UTC()},
		{"null", `null`, time.Time{}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var ts Timestamp
			if err := json.Unmarshal([]byte(tt.in), &ts); err != nil {
				t.Fatal(err)
			}
			if !ts.Time.Equal(tt.want) {
				t.Errorf("got %v, want %v", ts.Time, tt.want)
			}
		})
	}
}

func TestTimestampMarshal(t *testing.T) {
	ts := New(time.Date(2024, 5, 15, 12, 34, 56, 789000000, time.UTC))
	b, err := json.Marshal(ts)
	if err != nil {
		t.Fatal(err)
	}
	if string(b) != `"2024-05-15T12:34:56.789Z"` {
		t.Errorf("got %s", b)
	}
}

func TestTimestampNilAsTime(t *testing.T) {
	var ts *Timestamp
	if got := ts.AsTime(); !got.Equal(time.Unix(0, 0).UTC()) {
		t.Errorf("nil AsTime = %v, want epoch", got)
	}
}

func TestInt64(t *testing.T) {
	tests := []struct {
		name string
		in   string
		want Int64
	}{
		{"number", `42`, 42},
		{"string", `"42"`, 42},
		{"large number", `9007199254740993`, 9007199254740993},
		{"large string", `"9007199254740993"`, 9007199254740993},
		{"negative string", `"-7"`, -7},
		{"null", `null`, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var v Int64
			if err := json.Unmarshal([]byte(tt.in), &v); err != nil {
				t.Fatal(err)
			}
			if v != tt.want {
				t.Errorf("got %d, want %d", v, tt.want)
			}
		})
	}

	b, err := json.Marshal(Int64(9007199254740993))
	if err != nil {
		t.Fatal(err)
	}
	if string(b) != "9007199254740993" {
		t.Errorf("marshal got %s, want unquoted number", b)
	}
}
