// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.1
// 	protoc        v3.19.6
// source: header.proto

package header

import (
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

type Header struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Map map[string]*Header_Value `protobuf:"bytes,1,rep,name=map,proto3" json:"map,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
}

func (x *Header) Reset() {
	*x = Header{}
	mi := &file_header_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Header) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Header) ProtoMessage() {}

func (x *Header) ProtoReflect() protoreflect.Message {
	mi := &file_header_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Header.ProtoReflect.Descriptor instead.
func (*Header) Descriptor() ([]byte, []int) {
	return file_header_proto_rawDescGZIP(), []int{0}
}

func (x *Header) GetMap() map[string]*Header_Value {
	if x != nil {
		return x.Map
	}
	return nil
}

type Header_Value struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Value []string `protobuf:"bytes,1,rep,name=value,proto3" json:"value,omitempty"`
}

func (x *Header_Value) Reset() {
	*x = Header_Value{}
	mi := &file_header_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Header_Value) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Header_Value) ProtoMessage() {}

func (x *Header_Value) ProtoReflect() protoreflect.Message {
	mi := &file_header_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Header_Value.ProtoReflect.Descriptor instead.
func (*Header_Value) Descriptor() ([]byte, []int) {
	return file_header_proto_rawDescGZIP(), []int{0, 0}
}

func (x *Header_Value) GetValue() []string {
	if x != nil {
		return x.Value
	}
	return nil
}

var File_header_proto protoreflect.FileDescriptor

var file_header_proto_rawDesc = []byte{
	0x0a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06,
	0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0xa0, 0x01, 0x0a, 0x06, 0x48, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x12, 0x29, 0x0a, 0x03, 0x6d, 0x61, 0x70, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17,
	0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x4d,
	0x61, 0x70, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x03, 0x6d, 0x61, 0x70, 0x1a, 0x1d, 0x0a, 0x05,
	0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01,
	0x20, 0x03, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x1a, 0x4c, 0x0a, 0x08, 0x4d,
	0x61, 0x70, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x2a, 0x0a, 0x05, 0x76, 0x61, 0x6c,
	0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x05,
	0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x42, 0x50, 0x0a, 0x18, 0x6f, 0x72, 0x67,
	0x2e, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x62, 0x75, 0x66, 0x73, 0x42, 0x0f, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f,
	0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x50, 0x01, 0x5a, 0x21, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f,
	0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x62, 0x06, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x33,
}

var (
	file_header_proto_rawDescOnce sync.Once
	file_header_proto_rawDescData = file_header_proto_rawDesc
)

func file_header_proto_rawDescGZIP() []byte {
	file_header_proto_rawDescOnce.Do(func() {
		file_header_proto_rawDescData = protoimpl.X.CompressGZIP(file_header_proto_rawDescData)
	})
	return file_header_proto_rawDescData
}

var file_header_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_header_proto_goTypes = []any{
	(*Header)(nil),       // 0: header.Header
	(*Header_Value)(nil), // 1: header.Header.Value
	nil,                  // 2: header.Header.MapEntry
}
var file_header_proto_depIdxs = []int32{
	2, // 0: header.Header.map:type_name -> header.Header.MapEntry
	1, // 1: header.Header.MapEntry.value:type_name -> header.Header.Value
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_header_proto_init() }
func file_header_proto_init() {
	if File_header_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_header_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_header_proto_goTypes,
		DependencyIndexes: file_header_proto_depIdxs,
		MessageInfos:      file_header_proto_msgTypes,
	}.Build()
	File_header_proto = out.File
	file_header_proto_rawDesc = nil
	file_header_proto_goTypes = nil
	file_header_proto_depIdxs = nil
}
