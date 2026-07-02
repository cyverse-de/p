// Phase A golden generator: captures the current wire formats produced by the
// protobuf-generated types so the hand-written replacements can be verified
// against them after the protobuf code is deleted.
//
// For every message in the DE proto packages it writes, under testdata/<pkg>/:
//
//	<Message>.full.emit.json     protojson, EmitUnpopulated (NATS dialect, most services)
//	<Message>.full.bare.json     protojson, default options (NATS dialect, discoenv-*)
//	<Message>.full.httpjson.json encoding/json over the generated struct (subscriptions HTTP dialect)
//	<Message>.zero.emit.json     zero value, EmitUnpopulated
//	<Message>.zero.bare.json     zero value, default options
//	<Message>.full.oneof_<f>.emit.json   one variant per oneof member
//
// protojson deliberately randomizes whitespace, so all output is normalized
// (parsed with json.Number preserved, re-marshaled with sorted keys/indent)
// to keep the committed files stable.
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/reflect/protoregistry"

	_ "github.com/cyverse-de/p/go/analysis"
	_ "github.com/cyverse-de/p/go/apps"
	_ "github.com/cyverse-de/p/go/containers"
	_ "github.com/cyverse-de/p/go/header"
	_ "github.com/cyverse-de/p/go/monitoring"
	_ "github.com/cyverse-de/p/go/qms"
	_ "github.com/cyverse-de/p/go/requests"
	_ "github.com/cyverse-de/p/go/svcerror"
	_ "github.com/cyverse-de/p/go/user"
)

// groups and tools are omitted: nothing in the DE imports them and their
// modules are being dropped rather than rewritten.
var wantedPackages = map[string]bool{
	"analysis":      true,
	"apps":          true,
	"containers":    true,
	"header":        true,
	"monitoring":    true,
	"qms":           true,
	"requests":      true,
	"svcerror":      true,
	"user":          true,
	"user_requests": true,
}

const timestampFullName = "google.protobuf.Timestamp"

func main() {
	outRoot := "testdata"
	if len(os.Args) > 1 {
		outRoot = os.Args[1]
	}

	var msgTypes []protoreflect.MessageType
	protoregistry.GlobalTypes.RangeMessages(func(mt protoreflect.MessageType) bool {
		pkg := string(mt.Descriptor().ParentFile().Package())
		if wantedPackages[pkg] {
			msgTypes = append(msgTypes, mt)
		}
		return true
	})

	count := 0
	for _, mt := range msgTypes {
		desc := mt.Descriptor()
		pkg := string(desc.ParentFile().Package())
		dir := filepath.Join(outRoot, pkg)
		if err := os.MkdirAll(dir, 0o755); err != nil {
			fatal(err)
		}
		name := string(desc.Name())

		full := mt.New()
		populate(full, nil)
		writeVariants(dir, name, "full", full.Interface())
		writeHTTPJSON(dir, name, full.Interface(), desc)

		zero := mt.New()
		writeVariants(dir, name, "zero", zero.Interface())

		// One extra fixture per oneof member beyond the default choice, so the
		// wire name of every member is captured.
		oneofs := desc.Oneofs()
		for i := 0; i < oneofs.Len(); i++ {
			fields := oneofs.Get(i).Fields()
			for j := 0; j < fields.Len(); j++ {
				variant := mt.New()
				populate(variant, nil)
				setField(variant, fields.Get(j), nil)
				emitJSON(filepath.Join(dir, fmt.Sprintf("%s.full.oneof_%s.emit.json", name, fields.Get(j).Name())),
					protojson.MarshalOptions{EmitUnpopulated: true}, variant.Interface())
			}
		}
		count++
	}
	fmt.Printf("wrote goldens for %d messages under %s\n", count, outRoot)
}

func writeVariants(dir, name, kind string, msg proto.Message) {
	emitJSON(filepath.Join(dir, fmt.Sprintf("%s.%s.emit.json", name, kind)),
		protojson.MarshalOptions{EmitUnpopulated: true}, msg)
	emitJSON(filepath.Join(dir, fmt.Sprintf("%s.%s.bare.json", name, kind)),
		protojson.MarshalOptions{}, msg)
}

// writeHTTPJSON captures the second dialect: encoding/json over the generated
// struct tags, as emitted by subscriptions' Echo HTTP handlers. Messages with
// oneofs are skipped — the wrapper interface marshals uselessly and no oneof
// message crosses the HTTP boundary.
func writeHTTPJSON(dir, name string, msg proto.Message, desc protoreflect.MessageDescriptor) {
	if hasOneof(desc, map[protoreflect.FullName]bool{}) {
		return
	}
	b, err := json.Marshal(msg)
	if err != nil {
		fatal(fmt.Errorf("httpjson %s: %w", name, err))
	}
	writeNormalized(filepath.Join(dir, fmt.Sprintf("%s.full.httpjson.json", name)), b)
}

func hasOneof(desc protoreflect.MessageDescriptor, seen map[protoreflect.FullName]bool) bool {
	if seen[desc.FullName()] {
		return false
	}
	seen[desc.FullName()] = true
	if desc.Oneofs().Len() > 0 {
		return true
	}
	fields := desc.Fields()
	for i := 0; i < fields.Len(); i++ {
		fd := fields.Get(i)
		if fd.Kind() == protoreflect.MessageKind && fd.Message().FullName() != timestampFullName {
			if hasOneof(fd.Message(), seen) {
				return true
			}
		}
	}
	return false
}

func emitJSON(path string, opts protojson.MarshalOptions, msg proto.Message) {
	b, err := opts.Marshal(msg)
	if err != nil {
		fatal(fmt.Errorf("%s: %w", path, err))
	}
	writeNormalized(path, b)
}

func writeNormalized(path string, raw []byte) {
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.UseNumber()
	var v any
	if err := dec.Decode(&v); err != nil {
		fatal(fmt.Errorf("normalize %s: %w", path, err))
	}
	out, err := json.MarshalIndent(v, "", "  ")
	if err != nil {
		fatal(err)
	}
	if err := os.WriteFile(path, append(out, '\n'), 0o644); err != nil {
		fatal(err)
	}
}

// populate fills every field of msg with deterministic values. For each oneof
// the first member is chosen. Recursive message cycles are broken by leaving
// the repeated-ancestor field unset.
func populate(msg protoreflect.Message, ancestors []protoreflect.FullName) {
	desc := msg.Descriptor()
	for _, a := range ancestors {
		if a == desc.FullName() {
			return
		}
	}
	ancestors = append(ancestors, desc.FullName())

	fields := desc.Fields()
	for i := 0; i < fields.Len(); i++ {
		fd := fields.Get(i)
		if od := fd.ContainingOneof(); od != nil && fd != od.Fields().Get(0) {
			continue
		}
		setField(msg, fd, ancestors)
	}
}

func setField(msg protoreflect.Message, fd protoreflect.FieldDescriptor, ancestors []protoreflect.FullName) {
	switch {
	case fd.IsMap():
		mp := msg.Mutable(fd).Map()
		for _, k := range []string{"alpha", "beta"} {
			key := protoreflect.ValueOfString(k).MapKey()
			if fd.MapValue().Kind() == protoreflect.MessageKind {
				v := mp.NewValue()
				populate(v.Message(), ancestors)
				mp.Set(key, v)
			} else {
				mp.Set(key, scalarValue(fd.MapValue(), k))
			}
		}
	case fd.IsList():
		list := msg.Mutable(fd).List()
		for n := 0; n < 2; n++ {
			if fd.Kind() == protoreflect.MessageKind {
				v := list.NewElement()
				populate(v.Message(), ancestors)
				list.Append(v)
			} else {
				list.Append(scalarValue(fd, fmt.Sprintf("%d", n)))
			}
		}
	case fd.Kind() == protoreflect.MessageKind:
		if fd.Message().FullName() == timestampFullName {
			ts := msg.Mutable(fd).Message()
			ts.Set(ts.Descriptor().Fields().ByName("seconds"), protoreflect.ValueOfInt64(1715776496))
			var nanos int32
			if fd.Number()%2 == 1 {
				nanos = 789000000
			}
			ts.Set(ts.Descriptor().Fields().ByName("nanos"), protoreflect.ValueOfInt32(nanos))
			return
		}
		populate(msg.Mutable(fd).Message(), ancestors)
	default:
		msg.Set(fd, scalarValue(fd, ""))
	}
}

func scalarValue(fd protoreflect.FieldDescriptor, suffix string) protoreflect.Value {
	n := int64(fd.Number())
	switch fd.Kind() {
	case protoreflect.StringKind:
		s := string(fd.Name()) + "_v"
		if suffix != "" {
			s += "_" + suffix
		}
		return protoreflect.ValueOfString(s)
	case protoreflect.BoolKind:
		return protoreflect.ValueOfBool(true)
	case protoreflect.Int32Kind, protoreflect.Sint32Kind, protoreflect.Sfixed32Kind:
		return protoreflect.ValueOfInt32(int32(100 + n))
	case protoreflect.Int64Kind, protoreflect.Sint64Kind, protoreflect.Sfixed64Kind:
		// Beyond 2^53 so float64-based JSON handling would corrupt it.
		return protoreflect.ValueOfInt64(9007199254740993 + n)
	case protoreflect.Uint32Kind, protoreflect.Fixed32Kind:
		return protoreflect.ValueOfUint32(uint32(200 + n))
	case protoreflect.Uint64Kind, protoreflect.Fixed64Kind:
		return protoreflect.ValueOfUint64(uint64(9007199254740995 + n))
	case protoreflect.FloatKind:
		return protoreflect.ValueOfFloat32(1.5)
	case protoreflect.DoubleKind:
		return protoreflect.ValueOfFloat64(float64(n) + 0.25)
	case protoreflect.BytesKind:
		return protoreflect.ValueOfBytes([]byte{1, 2, 3})
	case protoreflect.EnumKind:
		values := fd.Enum().Values()
		idx := 0
		if values.Len() > 1 {
			idx = 1
		}
		return protoreflect.ValueOfEnum(values.Get(idx).Number())
	default:
		fatal(fmt.Errorf("unhandled kind %v for field %s", fd.Kind(), fd.FullName()))
		return protoreflect.Value{}
	}
}

func fatal(err error) {
	fmt.Fprintln(os.Stderr, "error:", err)
	os.Exit(1)
}
