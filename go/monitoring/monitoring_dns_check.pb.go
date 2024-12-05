// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.1
// 	protoc        v5.29.0
// source: monitoring_dns_check.proto

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

type LookupType int32

const (
	LookupType_UNSET_LOOKUP    LookupType = 0
	LookupType_INTERNAL_LOOKUP LookupType = 1
	LookupType_EXTERNAL_LOOKUP LookupType = 2
)

// Enum value maps for LookupType.
var (
	LookupType_name = map[int32]string{
		0: "UNSET_LOOKUP",
		1: "INTERNAL_LOOKUP",
		2: "EXTERNAL_LOOKUP",
	}
	LookupType_value = map[string]int32{
		"UNSET_LOOKUP":    0,
		"INTERNAL_LOOKUP": 1,
		"EXTERNAL_LOOKUP": 2,
	}
)

func (x LookupType) Enum() *LookupType {
	p := new(LookupType)
	*p = x
	return p
}

func (x LookupType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (LookupType) Descriptor() protoreflect.EnumDescriptor {
	return file_monitoring_dns_check_proto_enumTypes[0].Descriptor()
}

func (LookupType) Type() protoreflect.EnumType {
	return &file_monitoring_dns_check_proto_enumTypes[0]
}

func (x LookupType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use LookupType.Descriptor instead.
func (LookupType) EnumDescriptor() ([]byte, []int) {
	return file_monitoring_dns_check_proto_rawDescGZIP(), []int{0}
}

type DNSLookup struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Host      string   `protobuf:"bytes,1,opt,name=host,proto3" json:"host,omitempty"`
	Addresses []string `protobuf:"bytes,2,rep,name=addresses,proto3" json:"addresses,omitempty"`
	Type      string   `protobuf:"bytes,3,opt,name=type,proto3" json:"type,omitempty"`
	Error     string   `protobuf:"bytes,4,opt,name=error,proto3" json:"error,omitempty"`
}

func (x *DNSLookup) Reset() {
	*x = DNSLookup{}
	mi := &file_monitoring_dns_check_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DNSLookup) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DNSLookup) ProtoMessage() {}

func (x *DNSLookup) ProtoReflect() protoreflect.Message {
	mi := &file_monitoring_dns_check_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DNSLookup.ProtoReflect.Descriptor instead.
func (*DNSLookup) Descriptor() ([]byte, []int) {
	return file_monitoring_dns_check_proto_rawDescGZIP(), []int{0}
}

func (x *DNSLookup) GetHost() string {
	if x != nil {
		return x.Host
	}
	return ""
}

func (x *DNSLookup) GetAddresses() []string {
	if x != nil {
		return x.Addresses
	}
	return nil
}

func (x *DNSLookup) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (x *DNSLookup) GetError() string {
	if x != nil {
		return x.Error
	}
	return ""
}

type DNSCheckResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Header   *header.Header         `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	Error    *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	Lookups  []*DNSLookup           `protobuf:"bytes,3,rep,name=lookups,proto3" json:"lookups,omitempty"`
	Node     string                 `protobuf:"bytes,4,opt,name=node,proto3" json:"node,omitempty"`
	DateSent string                 `protobuf:"bytes,5,opt,name=date_sent,proto3" json:"date_sent,omitempty"`
}

func (x *DNSCheckResult) Reset() {
	*x = DNSCheckResult{}
	mi := &file_monitoring_dns_check_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DNSCheckResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DNSCheckResult) ProtoMessage() {}

func (x *DNSCheckResult) ProtoReflect() protoreflect.Message {
	mi := &file_monitoring_dns_check_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DNSCheckResult.ProtoReflect.Descriptor instead.
func (*DNSCheckResult) Descriptor() ([]byte, []int) {
	return file_monitoring_dns_check_proto_rawDescGZIP(), []int{1}
}

func (x *DNSCheckResult) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *DNSCheckResult) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *DNSCheckResult) GetLookups() []*DNSLookup {
	if x != nil {
		return x.Lookups
	}
	return nil
}

func (x *DNSCheckResult) GetNode() string {
	if x != nil {
		return x.Node
	}
	return ""
}

func (x *DNSCheckResult) GetDateSent() string {
	if x != nil {
		return x.DateSent
	}
	return ""
}

var File_monitoring_dns_check_proto protoreflect.FileDescriptor

var file_monitoring_dns_check_proto_rawDesc = []byte{
	0x0a, 0x1a, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x6e, 0x73,
	0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x6d, 0x6f,
	0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x1a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x67, 0x0a, 0x09, 0x44, 0x4e, 0x53, 0x4c, 0x6f, 0x6f,
	0x6b, 0x75, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x04, 0x68, 0x6f, 0x73, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x61, 0x64, 0x64, 0x72, 0x65,
	0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09, 0x61, 0x64, 0x64, 0x72,
	0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72,
	0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22,
	0xc9, 0x01, 0x0a, 0x0e, 0x44, 0x4e, 0x53, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x75,
	0x6c, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x05, 0x65, 0x72,
	0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x76, 0x63, 0x65,
	0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72, 0x72, 0x6f,
	0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x2f, 0x0a, 0x07, 0x6c, 0x6f, 0x6f, 0x6b,
	0x75, 0x70, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6d, 0x6f, 0x6e, 0x69,
	0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x2e, 0x44, 0x4e, 0x53, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70,
	0x52, 0x07, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x6f, 0x64,
	0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x12, 0x1c, 0x0a,
	0x09, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x09, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x2a, 0x48, 0x0a, 0x0a, 0x4c,
	0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x54, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x55, 0x4e, 0x53,
	0x45, 0x54, 0x5f, 0x4c, 0x4f, 0x4f, 0x4b, 0x55, 0x50, 0x10, 0x00, 0x12, 0x13, 0x0a, 0x0f, 0x49,
	0x4e, 0x54, 0x45, 0x52, 0x4e, 0x41, 0x4c, 0x5f, 0x4c, 0x4f, 0x4f, 0x4b, 0x55, 0x50, 0x10, 0x01,
	0x12, 0x13, 0x0a, 0x0f, 0x45, 0x58, 0x54, 0x45, 0x52, 0x4e, 0x41, 0x4c, 0x5f, 0x4c, 0x4f, 0x4f,
	0x4b, 0x55, 0x50, 0x10, 0x02, 0x42, 0x57, 0x0a, 0x18, 0x6f, 0x72, 0x67, 0x2e, 0x63, 0x79, 0x76,
	0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
	0x73, 0x42, 0x12, 0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x44, 0x4e, 0x53,
	0x43, 0x68, 0x65, 0x63, 0x6b, 0x50, 0x01, 0x5a, 0x25, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
	0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70,
	0x2f, 0x67, 0x6f, 0x2f, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_monitoring_dns_check_proto_rawDescOnce sync.Once
	file_monitoring_dns_check_proto_rawDescData = file_monitoring_dns_check_proto_rawDesc
)

func file_monitoring_dns_check_proto_rawDescGZIP() []byte {
	file_monitoring_dns_check_proto_rawDescOnce.Do(func() {
		file_monitoring_dns_check_proto_rawDescData = protoimpl.X.CompressGZIP(file_monitoring_dns_check_proto_rawDescData)
	})
	return file_monitoring_dns_check_proto_rawDescData
}

var file_monitoring_dns_check_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_monitoring_dns_check_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_monitoring_dns_check_proto_goTypes = []any{
	(LookupType)(0),               // 0: monitoring.LookupType
	(*DNSLookup)(nil),             // 1: monitoring.DNSLookup
	(*DNSCheckResult)(nil),        // 2: monitoring.DNSCheckResult
	(*header.Header)(nil),         // 3: header.Header
	(*svcerror.ServiceError)(nil), // 4: svcerror.ServiceError
}
var file_monitoring_dns_check_proto_depIdxs = []int32{
	3, // 0: monitoring.DNSCheckResult.header:type_name -> header.Header
	4, // 1: monitoring.DNSCheckResult.error:type_name -> svcerror.ServiceError
	1, // 2: monitoring.DNSCheckResult.lookups:type_name -> monitoring.DNSLookup
	3, // [3:3] is the sub-list for method output_type
	3, // [3:3] is the sub-list for method input_type
	3, // [3:3] is the sub-list for extension type_name
	3, // [3:3] is the sub-list for extension extendee
	0, // [0:3] is the sub-list for field type_name
}

func init() { file_monitoring_dns_check_proto_init() }
func file_monitoring_dns_check_proto_init() {
	if File_monitoring_dns_check_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_monitoring_dns_check_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_monitoring_dns_check_proto_goTypes,
		DependencyIndexes: file_monitoring_dns_check_proto_depIdxs,
		EnumInfos:         file_monitoring_dns_check_proto_enumTypes,
		MessageInfos:      file_monitoring_dns_check_proto_msgTypes,
	}.Build()
	File_monitoring_dns_check_proto = out.File
	file_monitoring_dns_check_proto_rawDesc = nil
	file_monitoring_dns_check_proto_goTypes = nil
	file_monitoring_dns_check_proto_depIdxs = nil
}
