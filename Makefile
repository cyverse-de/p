.PHONY: all compile go-tidy go-init clean java-jar compile-go compile-java compile-rust documentation clean-go clean-java clean-rust clean-docs

all: clean compile go-init go-tidy java-jar documentation
comma:=

export GOTOOLCHAIN=auto

compile-go:
	protoc -I ./protos -I ./protos/vendor \
		--go_out=./go \
		--go_opt=module=github.com/cyverse-de/p/go \
		--go-grpc_out=./go \
		--go-grpc_opt=module=github.com/cyverse-de/p/go \
		protos/*.proto

compile-java:
	protoc -I ./protos -I ./protos/vendor --java_out=./java protos/*.proto

compile-rust:
	protoc -I ./protos -I ./protos/vendor \
		--prost_opt=default_package_filename=gen.rs \
		--prost_opt=compile_well_known_types \
		--prost_opt=extern_path=.google.protobuf=::pbjson_types \
		--prost_opt=type_attribute=.groups.ServiceInfo='#[derive(garde::Validate)]' \
		--prost_opt=field_attribute=.groups.ServiceInfo='#[garde(length(min=1))]' \
		--prost_out=debuff/src/ \
		--prost-serde_out=debuff/src \
		protos/*.proto

compile: compile-go compile-java compile-rust

documentation:
	protoc -I ./protos -I ./protos/vendor --doc_out=./docs --doc_opt=markdown,analysis.md protos/analysis_*.proto
	protoc -I ./protos -I ./protos/vendor --doc_out=./docs --doc_opt=markdown,monitoring.md protos/monitoring_*.proto
	protoc -I ./protos -I ./protos/vendor --doc_out=./docs --doc_opt=markdown,qms.md protos/qms_*.proto
	protoc -I ./protos -I ./protos/vendor --doc_out=./docs --doc_opt=markdown,common.md protos/header.proto protos/svcerror.proto
	protoc -I ./protos -I ./protos/vendor --doc_out=./docs --doc_opt=markdown,users.md protos/user.proto protos/user_requests.proto
	protoc -I ./protos -I ./protos/vendor --doc_out=./docs --doc_opt=markdown,requests.md protos/requests.proto
	protoc -I ./protos -I ./protos/vendor --doc_out=./docs --doc_opt=markdown,tools.md protos/tools.proto
	

java-jar:
	lein jar

go-init:
	cd ./go/containers && go mod init github.com/cyverse-de/p/go/containers
	cd ./go/analysis && go mod init github.com/cyverse-de/p/go/analysis
	cd ./go/user && go mod init github.com/cyverse-de/p/go/user
	cd ./go/svcerror && go mod init github.com/cyverse-de/p/go/svcerror
	cd ./go/header && go mod init github.com/cyverse-de/p/go/header
	cd ./go/qms && go mod init github.com/cyverse-de/p/go/qms
	cd ./go/monitoring && go mod init github.com/cyverse-de/p/go/monitoring
	cd ./go/requests && go mod init github.com/cyverse-de/p/go/requests
	cd ./go/tools && go mod init github.com/cyverse-de/p/go/tools
	cd ./go/userservice && go mod init github.com/cyverse-de/p/go/userservice

go-tidy:
	cd ./go/containers && go mod tidy
	cd ./go/analysis && go mod tidy
	cd ./go/user && go mod tidy
	cd ./go/svcerror && go mod tidy
	cd ./go/header && go mod tidy
	cd ./go/qms && go mod tidy
	cd ./go/monitoring && go mod tidy
	cd ./go/requests && go mod tidy
	cd ./go/tools && go mod tidy
	cd ./go/userservice && go mod tidy

clean-go:
	rm -rf ./go/*

clean-java:
	rm -rf ./java/*
	lein clean

clean-docs:
	rm -rf ./docs/*

clean-rust:
	rm -rf ./debuff/src/*

clean: clean-go clean-java clean-rust clean-docs

godirs: $(ls ./go/)
