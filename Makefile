.PHONY: all go go-tidy clean

all: go 

go:
	protoc -I /usr/local/include/ -I ./protos --go_opt=paths=source_relative --go_out=./go protos/**/*.proto

go-tidy:
	cd ./go/analysis && go mod tidy
	cd ./go/user && go mod tidy
	cd ./go/svcerror && go mod tidy

clean:
	rm -rf ./go/analysis
	rm -rf ./go/user
	rm -rf ./go/svcerror
