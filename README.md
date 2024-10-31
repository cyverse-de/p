# p

This repo contains the protocol buffer v3 definitions and generated modules for
data types that cross service boundaries in the Discovery Environment.

## Requirements

### TL;DR

You will need the following:

- `protoc`
- `protoc-gen-go`
- `protoc-gen-go-grpc`
- `protoc-gen-doc`
- `protoc-gen-prost`

### Installation

On MacOS, you can install `protoc` with `homebrew`.

```
brew install protobuf
```

On Fedora, you can install `protoc` and header files with the following command:

```
sudo dnf install protobuf protobuf-devel
```

You can install `protoc-gen-go` with `go install`:

```
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
```

You can install `protoc-gen-go-grpc` with `go-install`:

```
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

You can install `protoc-gen-doc` with the `go install` command:

```
go install github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc@latest
```

## Repo Layout

`protos/` contains the protocol buffers definitions.

`go/` contains the generated Go modules. Do not edit these files directly.

`java/` contains the generated Java packages. Do not edit these files directly.

## Building Go packages

The Makefile has a target to build the Go modules, so `make` or `make go` will
both work.

`make clean` will delete the generated files, so it is not run by default.

## NATS Subject Mapping

| Subject                           | Accepts                                                                  | Response                                                        | Responding Service                                                   |
| --------------------------------- | ------------------------------------------------------------------------ | --------------------------------------------------------------- | -------------------------------------------------------------------- |
| `cyverse.qms.user.usages.add`     | [qms.AddUsages](./protos/qms_requests.proto)                             | [qms.UsageResponse](./protos/qms_usages.proto)                  | [subscriptions](https://github.com/cyverse-de/subscriptions)         |
| `cyverse.qms.user.usages.get`     | [qms.GetUsages](./protos/qms_requests.proto)                             | [qms.UsageList](./protos/qms_usages.proto)                      | [subscriptions](https://github.com/cyverse-de/subscriptions)         |
| `cyverse.qms.user.overages.get`   | [qms.AllOveragesRequest](./protos/qms_requests.proto)                    | [qms.OverageList](./protos/qms_overages.proto)                  | [subscriptions](https://github.com/cyverse-de/subscriptions)         |
| `cyverse.qms.user.overages.check` | [qms.IsOverageRequest](./protos/qms_requests.proto)                      | [qms.IsOverage](./protos/qms_overages.proto)                    | [subscriptions](https://github.com/cyverse-de/subscriptions)         |
| `cyverse.discoenv.analyses.>`     | [analysis.AnalysisRecordLookupRequest](./protos/analysis_requests.proto) | [analysis.AnalysisRecordList](./protos/analysis_requests.proto) | [discoenv-analyses](https://github.com/cyverse-de/discoenv-analyses) |
| `cyverse.discoenv.users.>`        | [users.UserLookupRequest](./protos/user_requests.proto)                  | [user.User](./protos/user.proto)                                | [discoenv-users](https://github.com/cyverse-de/discoenv-users)       |
| `cyverse.qms.user.updates.get`    | [qms.UpdateListRequest](./protos/qms_updates.proto)                      | [qms.UpdateListResponse](./protos/qms_updates.proto)            | [subscriptions](https://github.com/cyverse-de/subscriptions)         |
| `cyverse.qms.user.updates.add`    | [qms.AddUpdateRequest](./protos/qms_updates.proto)                       | [qms.AddUpdateResponse](./protos/qms_updates.proto)             | [subscriptions](https://github.com/cyverse-de/subscriptions)         |
| `cyverse.qms.user.summary.get`    | [qms.RequestByUsername](./protos/qms_requests.proto)                     | [qms.UserPlanResponse](./protos/qms_user_plans.proto)           | [subscriptions](https://github.com/cyverse-de/subscriptions)         |

`

## Hints/Tips/Notes

### Reload VScode Window after generating Go code.

Sometimes the VSCode Go support thinks that some of the modules needed by the
`go` package are missing. I've found that reloading the window from the command
palette fixes that problem.
````
