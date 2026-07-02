package ptypes

import (
	"encoding/json"
	"fmt"
)

// MarshalEnumName encodes an enum value as its name string, as protojson did;
// values without a name fall back to their number.
func MarshalEnumName(names map[int32]string, v int32) ([]byte, error) {
	if s, ok := names[v]; ok {
		return json.Marshal(s)
	}
	return json.Marshal(v)
}

// UnmarshalEnumName decodes an enum from either a value name or a number.
func UnmarshalEnumName(values map[string]int32, enumType string, b []byte) (int32, error) {
	if len(b) > 0 && b[0] == '"' {
		var s string
		if err := json.Unmarshal(b, &s); err != nil {
			return 0, err
		}
		v, ok := values[s]
		if !ok {
			return 0, fmt.Errorf("unknown %s %q", enumType, s)
		}
		return v, nil
	}
	var v int32
	if err := json.Unmarshal(b, &v); err != nil {
		return 0, err
	}
	return v, nil
}
