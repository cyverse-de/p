_protoc := "protoc -I ./protos -I /usr/local/include"
_go_repo := "github.com/cyverse-de/p"
_go_base := "go"
_go_mods := "containers analysis user svcerror header qms monitoring requests tools"

# Compiles the protocol buffers into Go.
compile-go:
	{{_protoc}} --go_opt=module=github.com/cyverse-de/p/go --go_out=./go protos/*.proto

# Compiles the protocol buffers into Java.
compile-java:
	{{_protoc}} --java_out=./java protos/*.proto

# Compiles a single protobuf project for Rust.
_cr name:
	{{_protoc}} \
		--prost_opt=default_package_filename=gen.rs \
		--prost_opt=compile_well_known_types \
		--prost_opt=extern_path=.google.protobuf=::pbjson_types \
		--prost_out=debuff/src/ \
		--prost-serde_out=debuff/src \
		`ls protos/{{name}}.proto`

# Compiles the protocol buffers for Rust.
compile-rust: (_cr "analysis_*") (_cr "qms_*") (_cr "user*") (_cr "groups") (_cr "apps") (_cr "svcerror") (_cr "header") (_cr "containers")

# Compiles the protocol buffers for Go, Java, and Rust.
compile: compile-go compile-java compile-rust

# Calls 'go mod init' for new Go modules.
init-go: compile-go
	#!/usr/bin/env bash
	for mod in {{_go_mods}}; do
		if [ ! -f "./{{_go_base}}/$mod"/go.mod ]; then
			cd {{justfile_directory()}}/{{_go_base}}/$mod && go mod init {{_go_repo}}/go/$mod
		fi
	done 

# Calls 'go mod tidy' for Go modules.
tidy-go: init-go
	#!/usr/bin/env bash
	for mod in {{_go_mods}}; do
		cd {{justfile_directory()}}/{{_go_base}}/$mod && go mod tidy
	done

_doc filename proto-glob:
	@echo "generating docs/{{filename}}..."
	@{{_protoc}} --doc_out=./docs --doc_opt=markdown,{{filename}} `ls {{proto-glob}}`

_doc_analysis: (_doc "analysis.md" "protos/analysis_*.proto")
_doc_monitoring: (_doc "monitoring.md" "protos/monitoring_*.proto")
_doc_qms: (_doc "qms.md" "protos/qms_*.proto")
_doc_common: (_doc "common.md" "protos/header.proto protos/svcerror.proto")
_doc_users: (_doc "users.md" "protos/user.proto protos/user_requests.proto")
_doc_requests: (_doc "requests.md" "protos/requests.proto")
_doc_tools: (_doc "tools.md" "protos/tools.proto")

# Generates documentation.
docs: _doc_analysis _doc_monitoring _doc_qms _doc_common _doc_users _doc_requests _doc_tools

# Cleans out the generated Go code.
clean-go:
	rm -rf ./go/*

# Cleans out the generated Java code.
clean-java:
	rm -rf ./java/*

# Cleans out the generated documentation.
clean-docs:
	rm -rf ./docs/*

# Cleans up the clojure builds.
clean-clojure: clean-java
	lein clean

# Cleans up Go, Clojure, Java, and docs.
clean: clean-go clean-clojure clean-docs

# Cleans, compiles, tidies, and generates docs.
all: clean compile tidy-go docs
