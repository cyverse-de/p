.PHONY: all go clean

all: go 

go:
	protoc -I /usr/local/include/ -I ./protos --go_opt=paths=source_relative --go_out=./go protos/**/*.proto
	@cd ./go && go mod tidy

clean:
	rm -rf ./go/*