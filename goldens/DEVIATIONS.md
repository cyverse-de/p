# Sanctioned wire-format deviations

The hand-written types reproduce the protojson wire format captured in
`testdata/`, with the following intentional differences. All of them are
accepted by protojson-based peers, so mixed old/new deployments interoperate.

- **int64 fields marshal as JSON numbers, not strings.** protojson emitted
  64-bit integers as strings; `ptypes.Int64` emits numbers (which protojson
  parsers also accept) and parses both forms. Numbers keep the apps-service
  HTTP decode path (`discoenv-analyses`) behaving as before.
- **Empty repeated/map fields may marshal as `null` instead of `[]`/`{}`.**
  A nil slice or map marshals as `null` where protojson with EmitUnpopulated
  emitted an empty collection. protojson parsers accept `null` for both.
  Services that serve list-bearing responses over HTTP should initialize
  slices to non-nil values where UI consumers care.
- **Unpopulated fields are always emitted.** The services that registered the
  bare (non-EmitUnpopulated) codec — discoenv-users, discoenv-analyses — will
  emit explicit defaults after migrating. Known fields with default values are
  accepted by all peers.
- **Fractional seconds width.** `ptypes.Timestamp` uses RFC 3339 with
  nanosecond precision trimmed of trailing zeros; protojson always padded to
  3/6/9 digits. Same instants, occasionally different digit counts.

The `testdata/` files were generated from the protobuf-generated types before
their removal; see the repository history (goldens/gen at the commit that
added the fixtures) for the generator.
