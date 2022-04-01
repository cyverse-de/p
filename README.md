# pkg

This repo contains the protocol buffer v3 definitions and generated modules for
data types that cross service boundaries in the Discovery Environment.

## Requirements

### TL;DR

You will need the following:

- `protoc`
- `protoc-gen-go`
- A symlink to the protobuf `include/` directory in `/usr/local/include`

### Installation

On MacOS, you can install `protoc` and `protoc-gen-go` with `homebrew`.

```
brew install protobuf protoc-gen-go
```

Link the `google/` directory from `protobuf` into `/usr/local/include`,
replacing the `3.19.4` in the directory path with whichever version got
installed above:

```
sudo ln -sf $HOMEBREW_CELLAR/protobuf/3.19.4/include/google /usr/local/include/google
```

## Repo Layout

`protos/` contains the protocol buffers definitions.

`go/` contains the generated Go modules. Do not edit these files directly.

## Building Go packages

The Makefile has a target to build the Go modules, so `make` or `make go` will
both work.

`make clean` will delete the generated files, so it is not run by default.

## Hints/Tips/Notes

### Reload VScode Window after generating Go code.

Sometimes the VSCode Go support thinks that some of the modules needed by the
`go` package are missing. I've found that reloading the window from the command
palette fixes that problem.
