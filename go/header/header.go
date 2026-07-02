// Message types for the DE header messages, formerly generated from header.proto.
// The JSON tags are the wire contract; goldens/ in this repository verifies them.

package header

type Header struct {
	Map map[string]*Header_Value `json:"map"`
}

func (x *Header) GetMap() map[string]*Header_Value {
	if x != nil {
		return x.Map
	}
	return nil
}

type Header_Value struct {
	Value []string `json:"value"`
}

func (x *Header_Value) GetValue() []string {
	if x != nil {
		return x.Value
	}
	return nil
}
