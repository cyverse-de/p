# p

This repo contains the Go message types for data that crosses service
boundaries in the Discovery Environment, one Go module per domain under
`go/`. The messages travel as JSON, over NATS request/reply and over some
HTTP endpoints.

These types were originally generated from protocol buffer definitions and
were serialized with protojson; the hand-written types in this repository
preserve that wire format exactly. The JSON struct tags are the wire
contract — do not rename them. The `goldens/` module holds fixtures captured
from the generated code and tests that verify the hand-written types
reproduce them (see `goldens/DEVIATIONS.md` for the few sanctioned
differences). The protobuf definitions, the generated Java artifact
(`org.cyverse/cyverse-de-protobufs`), and the protoc tooling were removed;
they remain available in the repository history.

## Modules

- `go/analysis` — analysis records, lookups, status updates, and submissions
- `go/apps` — app and app-version records
- `go/containers` — tool container definitions
- `go/header` — the message header, which carries telemetry (OpenTelemetry
  trace context) inside each message
- `go/monitoring` — heartbeat and DNS-check messages
- `go/ptypes` — shared scalar helpers (`Timestamp`, `Int64`) that preserve
  the historical JSON encodings
- `go/qms` — QMS subscriptions, plans, quotas, usages, updates, overages,
  add-ons, users, and resource types
- `go/requests` — generic lookup/association request messages
- `go/svcerror` — the error embedded in response messages
- `go/user` — user records and lookup requests
- `goldens` — wire-format regression tests

`docs/` is frozen reference documentation of the message fields, generated
back when the protobuf definitions existed; the field tables are still
accurate.

## Development

`go.work` ties the modules together for local work, so cross-module changes
build and test without tagging. Run everything with:

    make test

## Releasing

Each module is versioned independently with tags of the form
`go/<module>/vX.Y.Z`. `tag-go-modules.sh` bumps the patch version of every
module at once. The inter-module versions in the `go.mod` files must point at
tags that exist; after tagging and pushing, `make tidy` refreshes the module
metadata.
