// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.0
// 	protoc        v3.19.4
// source: svcerror.proto

package svcerror

import (
	header "github.com/cyverse-de/p/go/header"
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

type ErrorCode int32

const (
	ErrorCode_UNSET             ErrorCode = 0 // Default value for the error code. Don't set the error code to this. Use Unspecified if tempted.
	ErrorCode_UNSPECIFIED       ErrorCode = 1 // An error occurred, but the kind wasn't specified or included in the list.
	ErrorCode_INTERNAL          ErrorCode = 2
	ErrorCode_NOT_FOUND         ErrorCode = 3
	ErrorCode_BAD_REQUEST       ErrorCode = 4
	ErrorCode_MARSHAL_FAILURE   ErrorCode = 5
	ErrorCode_UNMARSHAL_FAILURE ErrorCode = 6
	ErrorCode_PARAMETER_MISSING ErrorCode = 7
	ErrorCode_PARAMETER_INVALID ErrorCode = 8
)

// Enum value maps for ErrorCode.
var (
	ErrorCode_name = map[int32]string{
		0: "UNSET",
		1: "UNSPECIFIED",
		2: "INTERNAL",
		3: "NOT_FOUND",
		4: "BAD_REQUEST",
		5: "MARSHAL_FAILURE",
		6: "UNMARSHAL_FAILURE",
		7: "PARAMETER_MISSING",
		8: "PARAMETER_INVALID",
	}
	ErrorCode_value = map[string]int32{
		"UNSET":             0,
		"UNSPECIFIED":       1,
		"INTERNAL":          2,
		"NOT_FOUND":         3,
		"BAD_REQUEST":       4,
		"MARSHAL_FAILURE":   5,
		"UNMARSHAL_FAILURE": 6,
		"PARAMETER_MISSING": 7,
		"PARAMETER_INVALID": 8,
	}
)

func (x ErrorCode) Enum() *ErrorCode {
	p := new(ErrorCode)
	*p = x
	return p
}

func (x ErrorCode) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (ErrorCode) Descriptor() protoreflect.EnumDescriptor {
	return file_svcerror_proto_enumTypes[0].Descriptor()
}

func (ErrorCode) Type() protoreflect.EnumType {
	return &file_svcerror_proto_enumTypes[0]
}

func (x ErrorCode) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use ErrorCode.Descriptor instead.
func (ErrorCode) EnumDescriptor() ([]byte, []int) {
	return file_svcerror_proto_rawDescGZIP(), []int{0}
}

type ServiceError struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Header     *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	ErrorCode  ErrorCode      `protobuf:"varint,2,opt,name=error_code,proto3,enum=ErrorCode" json:"error_code,omitempty"`
	StatusCode int32          `protobuf:"varint,3,opt,name=status_code,proto3" json:"status_code,omitempty"`
	Message    string         `protobuf:"bytes,4,opt,name=message,proto3" json:"message,omitempty"`
}

func (x *ServiceError) Reset() {
	*x = ServiceError{}
	if protoimpl.UnsafeEnabled {
		mi := &file_svcerror_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ServiceError) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ServiceError) ProtoMessage() {}

func (x *ServiceError) ProtoReflect() protoreflect.Message {
	mi := &file_svcerror_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ServiceError.ProtoReflect.Descriptor instead.
func (*ServiceError) Descriptor() ([]byte, []int) {
	return file_svcerror_proto_rawDescGZIP(), []int{0}
}

func (x *ServiceError) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ServiceError) GetErrorCode() ErrorCode {
	if x != nil {
		return x.ErrorCode
	}
	return ErrorCode_UNSET
}

func (x *ServiceError) GetStatusCode() int32 {
	if x != nil {
		return x.StatusCode
	}
	return 0
}

func (x *ServiceError) GetMessage() string {
	if x != nil {
		return x.Message
	}
	return ""
}

var File_svcerror_proto protoreflect.FileDescriptor

var file_svcerror_proto_rawDesc = []byte{
	0x0a, 0x0e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x1a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x97,
	0x01, 0x0a, 0x0c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12,
	0x1f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x07, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x12, 0x2a, 0x0a, 0x0a, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x43, 0x6f, 0x64, 0x65,
	0x52, 0x0a, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x20, 0x0a, 0x0b,
	0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
	0x05, 0x52, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x18,
	0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2a, 0xaf, 0x01, 0x0a, 0x09, 0x45, 0x72, 0x72,
	0x6f, 0x72, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10,
	0x00, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44,
	0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x4e, 0x41, 0x4c, 0x10, 0x02,
	0x12, 0x0d, 0x0a, 0x09, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x03, 0x12,
	0x0f, 0x0a, 0x0b, 0x42, 0x41, 0x44, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x04,
	0x12, 0x13, 0x0a, 0x0f, 0x4d, 0x41, 0x52, 0x53, 0x48, 0x41, 0x4c, 0x5f, 0x46, 0x41, 0x49, 0x4c,
	0x55, 0x52, 0x45, 0x10, 0x05, 0x12, 0x15, 0x0a, 0x11, 0x55, 0x4e, 0x4d, 0x41, 0x52, 0x53, 0x48,
	0x41, 0x4c, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x10, 0x06, 0x12, 0x15, 0x0a, 0x11,
	0x50, 0x41, 0x52, 0x41, 0x4d, 0x45, 0x54, 0x45, 0x52, 0x5f, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4e,
	0x47, 0x10, 0x07, 0x12, 0x15, 0x0a, 0x11, 0x50, 0x41, 0x52, 0x41, 0x4d, 0x45, 0x54, 0x45, 0x52,
	0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x08, 0x42, 0x58, 0x0a, 0x18, 0x6f, 0x72,
	0x67, 0x2e, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x42, 0x15, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45,
	0x72, 0x72, 0x6f, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x50, 0x01, 0x5a,
	0x23, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65,
	0x72, 0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x73, 0x76, 0x63, 0x65,
	0x72, 0x72, 0x6f, 0x72, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_svcerror_proto_rawDescOnce sync.Once
	file_svcerror_proto_rawDescData = file_svcerror_proto_rawDesc
)

func file_svcerror_proto_rawDescGZIP() []byte {
	file_svcerror_proto_rawDescOnce.Do(func() {
		file_svcerror_proto_rawDescData = protoimpl.X.CompressGZIP(file_svcerror_proto_rawDescData)
	})
	return file_svcerror_proto_rawDescData
}

var file_svcerror_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_svcerror_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_svcerror_proto_goTypes = []interface{}{
	(ErrorCode)(0),        // 0: ErrorCode
	(*ServiceError)(nil),  // 1: ServiceError
	(*header.Header)(nil), // 2: Header
}
var file_svcerror_proto_depIdxs = []int32{
	2, // 0: ServiceError.header:type_name -> Header
	0, // 1: ServiceError.error_code:type_name -> ErrorCode
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_svcerror_proto_init() }
func file_svcerror_proto_init() {
	if File_svcerror_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_svcerror_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ServiceError); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_svcerror_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_svcerror_proto_goTypes,
		DependencyIndexes: file_svcerror_proto_depIdxs,
		EnumInfos:         file_svcerror_proto_enumTypes,
		MessageInfos:      file_svcerror_proto_msgTypes,
	}.Build()
	File_svcerror_proto = out.File
	file_svcerror_proto_rawDesc = nil
	file_svcerror_proto_goTypes = nil
	file_svcerror_proto_depIdxs = nil
}
