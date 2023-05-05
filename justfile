_protoc := "protoc -I ./protos -I /usr/local/include"


compile: compile-go compile-java compile-rust

compile-go:
	{{_protoc}} --go_opt=module=github.com/cyverse-de/p/go --go_out=./go protos/*.proto

compile-java:
	{{_protoc}} -java_out=./java protos/*.proto

compile-rust: compile-rust-groups

compile-rust-groups:
	{{_protoc}} \
		--prost_opt=default_package_filename=gen.rs \
		--prost_opt=compile_well_known_types \
		--prost_opt=extern_path=.google.protobuf=::pbjson_types \
		--prost_opt=type_attribute=.groups.ServiceInfo='#[derive(garde::Validate)]' \
		--prost_opt=field_attribute=.groups.ServiceInfo='#[garde(length(min=1))]' \
		--prost_out=debuff/src/ \
		--prost-serde_out=debuff/src \
		protos/groups.proto

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

documentation: _doc_analysis _doc_monitoring _doc_qms _doc_common _doc_users _doc_requests _doc_tools
