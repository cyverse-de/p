# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repo contains the hand-written Go message types for data that crosses service boundaries in the CyVerse Discovery Environment, one Go module per domain under `go/`. Messages travel as plain JSON over NATS request/reply and over some HTTP endpoints.

**The JSON struct tags are the wire contract.** These types replaced protobuf-generated code that was serialized with protojson; the tags reproduce that wire format exactly, and old and new services interoperate only as long as they stay unchanged. Renaming a tag silently breaks peers: protojson-based decoders reject unknown fields, and `encoding/json` drops mismatched keys without error.

## Build and Test

```bash
make test     # runs go test in every module, including the goldens
make tidy     # go mod tidy everywhere — requires all inter-module versions to be tagged and pushed
```

`go.work` ties the modules together so cross-module changes build and test locally without tagging.

## Critical Rules

- **Never rename a JSON tag.** If a change to `go/**` breaks the tests in `goldens/`, the change is wrong (or it is a deliberate wire-format change, which requires coordinating every deployed consumer).
- `goldens/testdata/` holds fixtures captured from the old protobuf-generated types. They are the regression contract; do not regenerate or hand-edit them to make a failing test pass. Intentional deviations are documented in `goldens/DEVIATIONS.md`.
- New fields must use the protojson naming convention the deployed services expect: the explicit `json_name` from the historical `.proto` (usually snake_case) when one existed, otherwise lowerCamelCase of the field name. New fields on existing messages are wire-safe (old peers tolerate unknown-on-input only for `encoding/json` consumers — protojson-era peers reject unknown fields, so additive changes still need consumer coordination until all peers are migrated).
- Use the `ptypes` helpers for scalars with historical encoding quirks: `ptypes.Timestamp` (emits RFC 3339, also parses legacy `{seconds,nanos}` objects), `ptypes.Int64` (emits numbers, parses protojson's strings), and `ptypes.MarshalEnumName`/`UnmarshalEnumName` for enums (name strings on the wire, numbers accepted).
- Message fields are pointers (nil → `null`); flattened oneof members are pointers with `omitempty` — at most one may be set, and responders should validate that.
- Some types intentionally preserve shapes from **old module versions** that deployed services still exchange (`user.LookupResponse` from `go/user/v0.0.10`, `analysis.AnalysisRecord` from `go/analysis/v0.0.12`). The deployed wire contract is whatever version a service pins, not what this repo's main once said.

## Adding a Message or Field

1. Add the struct/field with the correct wire-name tag and nil-safe `Get*` getter, matching the style of the surrounding file.
2. Add or extend golden fixtures under `goldens/testdata/<pkg>/` (full + zero variants) and a `registry.go` entry for new messages.
3. `make test` must pass, including `TestKeyInventory` (every field exercised by at least one golden).

## Releasing

Each module is versioned independently with tags of the form `go/<module>/vX.Y.Z`; `tag-go-modules.sh` bumps the patch version of every module at once. Published tags are immutable once the module proxy caches them — never retag; bump. Inter-module versions in the `go.mod` files must point at tags that exist.
