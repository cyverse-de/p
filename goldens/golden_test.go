// Wire-compatibility tests: the hand-written message types must reproduce the
// JSON that the protobuf-generated types put on the wire. The files under
// testdata/ were captured from the generated types before their removal (see
// the repository history for the generator). Sanctioned differences are
// documented in DEVIATIONS.md.
package goldens

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/big"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
	"time"

	"github.com/cyverse-de/p/go/ptypes"
)

type goldenFile struct {
	key     string // "<pkg>/<Message>"
	variant string // "full.emit", "zero.bare", "full.oneof_uuid.emit", ...
	path    string
}

func listGoldens(t *testing.T) []goldenFile {
	t.Helper()
	var out []goldenFile
	dirs, err := os.ReadDir("testdata")
	if err != nil {
		t.Fatal(err)
	}
	for _, d := range dirs {
		files, err := os.ReadDir(filepath.Join("testdata", d.Name()))
		if err != nil {
			t.Fatal(err)
		}
		for _, f := range files {
			name := strings.TrimSuffix(f.Name(), ".json")
			msg, variant, _ := strings.Cut(name, ".")
			key := d.Name() + "/" + msg
			if _, ok := messages[key]; !ok {
				t.Fatalf("golden %s has no registered message type", f.Name())
			}
			out = append(out, goldenFile{key, variant, filepath.Join("testdata", d.Name(), f.Name())})
		}
	}
	return out
}

func decodeJSON(t *testing.T, b []byte) any {
	t.Helper()
	dec := json.NewDecoder(bytes.NewReader(b))
	dec.UseNumber()
	var v any
	if err := dec.Decode(&v); err != nil {
		t.Fatal(err)
	}
	return v
}

func unmarshalGolden(t *testing.T, g goldenFile) any {
	t.Helper()
	raw, err := os.ReadFile(g.path)
	if err != nil {
		t.Fatal(err)
	}
	v := messages[g.key]()
	if err := json.Unmarshal(raw, v); err != nil {
		t.Fatalf("%s: unmarshal into %T: %v", g.path, v, err)
	}
	return v
}

// TestRoundTrip re-marshals every protojson golden through the new types and
// requires semantically identical JSON, so anything the new types emit is
// exactly what old protojson-based peers already accept and produce.
func TestRoundTrip(t *testing.T) {
	for _, g := range listGoldens(t) {
		if !strings.HasSuffix(g.variant, ".emit") && g.variant != "full.bare" && g.variant != "zero.bare" {
			continue
		}
		t.Run(g.key+"/"+g.variant, func(t *testing.T) {
			raw, err := os.ReadFile(g.path)
			if err != nil {
				t.Fatal(err)
			}
			v := unmarshalGolden(t, g)
			out, err := json.Marshal(v)
			if err != nil {
				t.Fatal(err)
			}
			want := decodeJSON(t, raw)
			got := decodeJSON(t, out)
			if strings.HasSuffix(g.variant, ".bare") {
				// bare goldens omit unpopulated fields; the new types always
				// emit them, so require the golden to be a semantic subset.
				if err := semanticSubset(want, got); err != nil {
					t.Errorf("bare golden not contained in output: %v\ngolden: %s\noutput: %s", err, raw, out)
				}
				return
			}
			if err := semanticEqual(want, got); err != nil {
				t.Errorf("round-trip mismatch: %v\ngolden: %s\noutput: %s", err, raw, out)
			}
		})
	}
}

// TestBareEquivalence checks that the bare-codec and EmitUnpopulated encodings
// of the same message decode to identical values.
func TestBareEquivalence(t *testing.T) {
	for _, g := range listGoldens(t) {
		if !strings.HasSuffix(g.variant, ".bare") {
			continue
		}
		t.Run(g.key+"/"+g.variant, func(t *testing.T) {
			emit := g
			emit.variant = strings.Replace(g.variant, ".bare", ".emit", 1)
			emit.path = strings.Replace(g.path, ".bare.json", ".emit.json", 1)
			a := unmarshalGolden(t, g)
			b := unmarshalGolden(t, emit)
			if err := structEqual(reflect.ValueOf(a), reflect.ValueOf(b)); err != nil {
				t.Errorf("bare and emit decode differently: %v", err)
			}
		})
	}
}

// TestHTTPDialect checks that the encoding/json output of the old generated
// structs (as produced by the not-yet-migrated subscriptions HTTP handlers)
// decodes into the new types with no information loss, for every message whose
// two dialects use the same field names.
func TestHTTPDialect(t *testing.T) {
	tested := 0
	for _, g := range listGoldens(t) {
		if g.variant != "full.httpjson" || !httpSafe[g.key] {
			continue
		}
		tested++
		t.Run(g.key, func(t *testing.T) {
			emit := g
			emit.variant = "full.emit"
			emit.path = strings.Replace(g.path, ".httpjson.json", ".emit.json", 1)
			a := unmarshalGolden(t, g)
			b := unmarshalGolden(t, emit)
			if err := structEqual(reflect.ValueOf(a), reflect.ValueOf(b)); err != nil {
				t.Errorf("http dialect decodes differently from protojson: %v", err)
			}
		})
	}
	if tested == 0 {
		t.Fatal("no httpjson goldens were tested")
	}
}

// TestKeyInventory walks every protojson golden in parallel with the Go types
// and requires (a) every golden key to correspond to a JSON tag — a typo here
// would make old protojson peers reject the message as containing unknown
// fields — and (b) every field of every engaged type to be exercised by at
// least one golden, catching fields whose tag no golden ever produced.
func TestKeyInventory(t *testing.T) {
	cov := map[reflect.Type]map[string]bool{}
	for _, g := range listGoldens(t) {
		if !strings.HasSuffix(g.variant, ".emit") {
			continue
		}
		raw, err := os.ReadFile(g.path)
		if err != nil {
			t.Fatal(err)
		}
		typ := reflect.TypeOf(messages[g.key]()).Elem()
		walkKeys(t, typ, decodeJSON(t, raw), cov, g.key)
	}
	for typ, seen := range cov {
		for _, tag := range jsonTags(typ) {
			if !seen[tag] {
				t.Errorf("%s: field %q never exercised by any golden", typ, tag)
			}
		}
	}
}

var timestampType = reflect.TypeFor[ptypes.Timestamp]()

func walkKeys(t *testing.T, typ reflect.Type, node any, cov map[reflect.Type]map[string]bool, ctx string) {
	t.Helper()
	if node == nil {
		return
	}
	for typ.Kind() == reflect.Pointer {
		typ = typ.Elem()
	}
	if typ == timestampType {
		return
	}
	switch typ.Kind() {
	case reflect.Struct:
		m, ok := node.(map[string]any)
		if !ok {
			t.Errorf("%s: expected object for %s, got %T", ctx, typ, node)
			return
		}
		fieldByTag := map[string]reflect.StructField{}
		for _, tag := range jsonTags(typ) {
			f, _ := fieldByJSONTag(typ, tag)
			fieldByTag[tag] = f
		}
		if cov[typ] == nil {
			cov[typ] = map[string]bool{}
		}
		for k, v := range m {
			f, ok := fieldByTag[k]
			if !ok {
				t.Errorf("%s: golden key %q has no JSON tag on %s", ctx, k, typ)
				continue
			}
			cov[typ][k] = true
			walkKeys(t, f.Type, v, cov, ctx+"."+k)
		}
	case reflect.Map:
		m, ok := node.(map[string]any)
		if !ok {
			t.Errorf("%s: expected object for %s, got %T", ctx, typ, node)
			return
		}
		for k, v := range m {
			walkKeys(t, typ.Elem(), v, cov, ctx+"["+k+"]")
		}
	case reflect.Slice:
		l, ok := node.([]any)
		if !ok {
			t.Errorf("%s: expected array for %s, got %T", ctx, typ, node)
			return
		}
		for _, v := range l {
			walkKeys(t, typ.Elem(), v, cov, ctx+"[]")
		}
	}
}

func jsonTags(typ reflect.Type) []string {
	var tags []string
	for i := 0; i < typ.NumField(); i++ {
		tag := typ.Field(i).Tag.Get("json")
		name, _, _ := strings.Cut(tag, ",")
		if name != "" && name != "-" {
			tags = append(tags, name)
		}
	}
	return tags
}

func fieldByJSONTag(typ reflect.Type, tag string) (reflect.StructField, bool) {
	for i := 0; i < typ.NumField(); i++ {
		name, _, _ := strings.Cut(typ.Field(i).Tag.Get("json"), ",")
		if name == tag {
			return typ.Field(i), true
		}
	}
	return reflect.StructField{}, false
}

// semanticEqual compares two decoded JSON trees, tolerating only the
// sanctioned representation differences: int64 as string vs number, and
// RFC 3339 timestamps that denote the same instant.
func semanticEqual(want, got any) error {
	return compareJSON(want, got, "$", false)
}

// semanticSubset requires every key present in want to match in got; got may
// have additional keys (used for bare goldens, which omit unpopulated fields).
func semanticSubset(want, got any) error {
	return compareJSON(want, got, "$", true)
}

func compareJSON(want, got any, path string, subset bool) error {
	switch w := want.(type) {
	case map[string]any:
		g, ok := got.(map[string]any)
		if !ok {
			return pathErr(path, "object", got)
		}
		if !subset && len(w) != len(g) {
			return pathErrf(path, "key sets differ: %d vs %d keys", len(w), len(g))
		}
		for k, wv := range w {
			gv, ok := g[k]
			if !ok {
				return pathErrf(path, "missing key %q", k)
			}
			if err := compareJSON(wv, gv, path+"."+k, subset); err != nil {
				return err
			}
		}
		return nil
	case []any:
		g, ok := got.([]any)
		if !ok {
			return pathErr(path, "array", got)
		}
		if len(w) != len(g) {
			return pathErrf(path, "array length %d vs %d", len(w), len(g))
		}
		for i := range w {
			if err := compareJSON(w[i], g[i], path+"[]", subset); err != nil {
				return err
			}
		}
		return nil
	case json.Number:
		return compareScalar(path, string(w), got)
	case string:
		return compareScalar(path, w, got)
	case bool:
		if g, ok := got.(bool); !ok || g != w {
			return pathErrf(path, "%v vs %v", w, got)
		}
		return nil
	case nil:
		if got != nil {
			return pathErrf(path, "null vs %v", got)
		}
		return nil
	default:
		return pathErrf(path, "unhandled golden type %T", want)
	}
}

func compareScalar(path, w string, got any) error {
	var g string
	switch v := got.(type) {
	case json.Number:
		g = string(v)
	case string:
		g = v
	default:
		return pathErrf(path, "%q vs %v (%T)", w, got, got)
	}
	if w == g {
		return nil
	}
	if wr, ok := new(big.Rat).SetString(w); ok {
		if gr, ok := new(big.Rat).SetString(g); ok && wr.Cmp(gr) == 0 {
			return nil
		}
	}
	if wt, err := time.Parse(time.RFC3339Nano, w); err == nil {
		if gt, err := time.Parse(time.RFC3339Nano, g); err == nil && wt.Equal(gt) {
			return nil
		}
	}
	return pathErrf(path, "%q vs %q", w, g)
}

// structEqual compares two decoded messages, treating nil and empty
// slices/maps as equal and comparing timestamps as instants.
func structEqual(a, b reflect.Value) error {
	return structEqualPath(a, b, "$")
}

func structEqualPath(a, b reflect.Value, path string) error {
	if a.Type() != b.Type() {
		return pathErrf(path, "type mismatch %s vs %s", a.Type(), b.Type())
	}
	switch a.Kind() {
	case reflect.Pointer:
		if a.IsNil() || b.IsNil() {
			if a.IsNil() != b.IsNil() {
				return pathErrf(path, "nil vs non-nil pointer")
			}
			return nil
		}
		return structEqualPath(a.Elem(), b.Elem(), path)
	case reflect.Struct:
		if a.Type() == timestampType {
			at := a.Interface().(ptypes.Timestamp)
			bt := b.Interface().(ptypes.Timestamp)
			if !at.Time.Equal(bt.Time) {
				return pathErrf(path, "timestamps %v vs %v", at.Time, bt.Time)
			}
			return nil
		}
		for i := 0; i < a.NumField(); i++ {
			if err := structEqualPath(a.Field(i), b.Field(i), path+"."+a.Type().Field(i).Name); err != nil {
				return err
			}
		}
		return nil
	case reflect.Slice:
		if a.Len() != b.Len() {
			return pathErrf(path, "slice length %d vs %d", a.Len(), b.Len())
		}
		for i := 0; i < a.Len(); i++ {
			if err := structEqualPath(a.Index(i), b.Index(i), path+"[]"); err != nil {
				return err
			}
		}
		return nil
	case reflect.Map:
		if a.Len() != b.Len() {
			return pathErrf(path, "map length %d vs %d", a.Len(), b.Len())
		}
		iter := a.MapRange()
		for iter.Next() {
			bv := b.MapIndex(iter.Key())
			if !bv.IsValid() {
				return pathErrf(path, "missing map key %v", iter.Key())
			}
			if err := structEqualPath(iter.Value(), bv, path+"["+iter.Key().String()+"]"); err != nil {
				return err
			}
		}
		return nil
	default:
		if !a.Equal(b) {
			return pathErrf(path, "%v vs %v", a, b)
		}
		return nil
	}
}

type compareError struct{ msg string }

func (e *compareError) Error() string { return e.msg }

func pathErr(path, expected string, got any) error {
	return &compareError{msg: fmt.Sprintf("%s: expected %s, got %T", path, expected, got)}
}

func pathErrf(path, format string, args ...any) error {
	return &compareError{msg: path + ": " + fmt.Sprintf(format, args...)}
}
