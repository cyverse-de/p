.PHONY: all go-compile go-tidy go-init clean

godirs: $(ls ./go/)

all: go-compile go-tidy

compile:
	protoc -I ./protos -I /usr/local/include --go_opt=module=github.com/cyverse-de/p/go --go_out=./go protos/*.proto
	protoc -I ./protos -I /usr/local/include --java_out=./java protos/*.proto 

go-init:
	cd ./go/analysis && go mod init github.com/cyverse-de/p/go/analysis
	cd ./go/user && go mod init github.com/cyverse-de/p/go/user
	cd ./go/svcerror && go mod init github.com/cyverse-de/p/go/svcerror
	cd ./go/header && go mod init github.com/cyverse-de/p/go/header
	cd ./go/qms && go mod init github.com/cyverse-de/p/go/qms

go-tidy:
	cd ./go/analysis && go mod tidy
	cd ./go/user && go mod tidy
	cd ./go/svcerror && go mod tidy
	cd ./go/header && go mod tidy
	cd ./go/qms && go mod tidy

clean:
	rm -rf ./go/*
	rm -rf ./java/*
