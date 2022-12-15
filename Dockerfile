FROM golang:1.19

ENV protoc_version 21.12
ENV arch x86_64

RUN apt-get update -y
RUN apt-get install -y curl unzip

RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v${protoc_version}/protoc-${protoc_version}-linux-${arch}.zip
RUN unzip protoc-21.12-linux-x86_64.zip -d /usr/local

RUN go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.28.0
RUN go install github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc@latest
# RUN ln -sf /usr/include/google/protobuf /usr/local/include/google


CMD ["bash"]

