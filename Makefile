VENDOR_DIR = ./protos/vendor
PROTOS_DIR = ./protos
DOCS_DIR = ./docs
GO_OUTPUT_DIR = ./go
JAVA_OUTPUT_DIR = ./java

export GOTOOLCHAIN=auto

.PHONY: all
all: clean compile go-init go-tidy java-jar documentation
comma:=

.PHONY: compile-go
compile-go:
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) \
		--go_out=$(GO_OUTPUT_DIR) \
		--go_opt=module=github.com/cyverse-de/p/go \
		--go-grpc_out=$(GO_OUTPUT_DIR) \
		--go-grpc_opt=module=github.com/cyverse-de/p/go \
		$(PROTOS_DIR)/*.proto

.PHONY: compile-java
compile-java:
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --java_out=$(JAVA_OUTPUT_DIR)/ $(PROTOS_DIR)/*.proto

.PHONY: compile
compile: compile-go compile-java

.PHONY: documentation
documentation:
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --doc_out=$(DOCS_DIR) --doc_opt=markdown,analysis.md $(PROTOS_DIR)/analysis_*.proto
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --doc_out=$(DOCS_DIR) --doc_opt=markdown,monitoring.md $(PROTOS_DIR)/monitoring_*.proto
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --doc_out=$(DOCS_DIR) --doc_opt=markdown,qms.md $(PROTOS_DIR)/qms_*.proto
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --doc_out=$(DOCS_DIR) --doc_opt=markdown,common.md $(PROTOS_DIR)/header.proto $(PROTOS_DIR)/svcerror.proto
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --doc_out=$(DOCS_DIR) --doc_opt=markdown,users.md $(PROTOS_DIR)/user.proto $(PROTOS_DIR)/user_requests.proto
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --doc_out=$(DOCS_DIR) --doc_opt=markdown,requests.md $(PROTOS_DIR)/requests.proto
	protoc -I $(PROTOS_DIR) -I $(VENDOR_DIR) --doc_out=$(DOCS_DIR) --doc_opt=markdown,tools.md $(PROTOS_DIR)/tools.proto
	
.PHONY: java-jar
java-jar:
	lein jar

.PHONY: go-init
go-init:
	cd $(GO_OUTPUT_DIR)/containers && go mod init github.com/cyverse-de/p/go/containers
	cd $(GO_OUTPUT_DIR)/analysis && go mod init github.com/cyverse-de/p/go/analysis
	cd $(GO_OUTPUT_DIR)/user && go mod init github.com/cyverse-de/p/go/user
	cd $(GO_OUTPUT_DIR)/svcerror && go mod init github.com/cyverse-de/p/go/svcerror
	cd $(GO_OUTPUT_DIR)/header && go mod init github.com/cyverse-de/p/go/header
	cd $(GO_OUTPUT_DIR)/qms && go mod init github.com/cyverse-de/p/go/qms
	cd $(GO_OUTPUT_DIR)/monitoring && go mod init github.com/cyverse-de/p/go/monitoring
	cd $(GO_OUTPUT_DIR)/requests && go mod init github.com/cyverse-de/p/go/requests
	cd $(GO_OUTPUT_DIR)/tools && go mod init github.com/cyverse-de/p/go/tools
	cd $(GO_OUTPUT_DIR)/userservice && go mod init github.com/cyverse-de/p/go/userservice

.PHONY: go-tidy
go-tidy:
	cd $(GO_OUTPUT_DIR)/containers && go mod tidy
	cd $(GO_OUTPUT_DIR)/analysis && go mod tidy
	cd $(GO_OUTPUT_DIR)/user && go mod tidy
	cd $(GO_OUTPUT_DIR)/svcerror && go mod tidy
	cd $(GO_OUTPUT_DIR)/header && go mod tidy
	cd $(GO_OUTPUT_DIR)/qms && go mod tidy
	cd $(GO_OUTPUT_DIR)/monitoring && go mod tidy
	cd $(GO_OUTPUT_DIR)/requests && go mod tidy
	cd $(GO_OUTPUT_DIR)/tools && go mod tidy
	cd $(GO_OUTPUT_DIR)/userservice && go mod tidy

.PHONY: clean-go
clean-go:
	rm -rf $(GO_OUTPUT_DIR)/*

.PHONY: clean-java
clean-java:
	rm -rf $(JAVA_OUTPUT_DIR)//*
	lein clean

.PHONY: clean-docs
clean-docs:
	rm -rf $(DOCS_DIR)/*

.PHONY: clean
clean: clean-go clean-java clean-docs

.PHONY: godirs
godirs: $(ls $(GO_OUTPUT_DIR)/)
