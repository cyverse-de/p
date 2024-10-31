// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.1
// 	protoc        v3.19.6
// source: monitoring_heartbeat.proto

package monitoring

import (
	header "github.com/cyverse-de/p/go/header"
	svcerror "github.com/cyverse-de/p/go/svcerror"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Heartbeat struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Header   *header.Header         `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	Error    *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	Node     string                 `protobuf:"bytes,3,opt,name=node,proto3" json:"node,omitempty"`
	DateSent string                 `protobuf:"bytes,4,opt,name=date_sent,proto3" json:"date_sent,omitempty"`
}

func (x *Heartbeat) Reset() {
	*x = Heartbeat{}
	mi := &file_monitoring_heartbeat_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Heartbeat) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Heartbeat) ProtoMessage() {}

func (x *Heartbeat) ProtoReflect() protoreflect.Message {
	mi := &file_monitoring_heartbeat_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Heartbeat.ProtoReflect.Descriptor instead.
func (*Heartbeat) Descriptor() ([]byte, []int) {
	return file_monitoring_heartbeat_proto_rawDescGZIP(), []int{0}
}

func (x *Heartbeat) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *Heartbeat) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *Heartbeat) GetNode() string {
	if x != nil {
		return x.Node
	}
	return ""
}

func (x *Heartbeat) GetDateSent() string {
	if x != nil {
		return x.DateSent
	}
	return ""
}

var File_monitoring_heartbeat_proto protoreflect.FileDescriptor

var file_monitoring_heartbeat_proto_rawDesc = []byte{
	0x0a, 0x1a, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x65, 0x61,
	0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x6d, 0x6f,
	0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x1a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x93, 0x01, 0x0a, 0x09, 0x48, 0x65, 0x61, 0x72, 0x74,
	0x62, 0x65, 0x61, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65,
	0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x05,
	0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x76,
	0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72,
	0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x6f,
	0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x12, 0x1c,
	0x0a, 0x09, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x09, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x42, 0x57, 0x0a, 0x18,
	0x6f, 0x72, 0x67, 0x2e, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x42, 0x12, 0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f,
	0x72, 0x69, 0x6e, 0x67, 0x48, 0x65, 0x61, 0x72, 0x62, 0x65, 0x61, 0x74, 0x50, 0x01, 0x5a, 0x25,
	0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72,
	0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x6d, 0x6f, 0x6e, 0x69, 0x74,
	0x6f, 0x72, 0x69, 0x6e, 0x67, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_monitoring_heartbeat_proto_rawDescOnce sync.Once
	file_monitoring_heartbeat_proto_rawDescData = file_monitoring_heartbeat_proto_rawDesc
)

func file_monitoring_heartbeat_proto_rawDescGZIP() []byte {
	file_monitoring_heartbeat_proto_rawDescOnce.Do(func() {
		file_monitoring_heartbeat_proto_rawDescData = protoimpl.X.CompressGZIP(file_monitoring_heartbeat_proto_rawDescData)
	})
	return file_monitoring_heartbeat_proto_rawDescData
}

var file_monitoring_heartbeat_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_monitoring_heartbeat_proto_goTypes = []any{
	(*Heartbeat)(nil),             // 0: monitoring.Heartbeat
	(*header.Header)(nil),         // 1: header.Header
	(*svcerror.ServiceError)(nil), // 2: svcerror.ServiceError
}
var file_monitoring_heartbeat_proto_depIdxs = []int32{
	1, // 0: monitoring.Heartbeat.header:type_name -> header.Header
	2, // 1: monitoring.Heartbeat.error:type_name -> svcerror.ServiceError
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_monitoring_heartbeat_proto_init() }
func file_monitoring_heartbeat_proto_init() {
	if File_monitoring_heartbeat_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_monitoring_heartbeat_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_monitoring_heartbeat_proto_goTypes,
		DependencyIndexes: file_monitoring_heartbeat_proto_depIdxs,
		MessageInfos:      file_monitoring_heartbeat_proto_msgTypes,
	}.Build()
	File_monitoring_heartbeat_proto = out.File
	file_monitoring_heartbeat_proto_rawDesc = nil
	file_monitoring_heartbeat_proto_goTypes = nil
	file_monitoring_heartbeat_proto_depIdxs = nil
}
