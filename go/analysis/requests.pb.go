// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.0
// 	protoc        v3.19.4
// source: analysis/requests.proto

package analysis

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

type AnalysisLookupRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to LookupIds:
	//	*AnalysisLookupRequest_AnalysisId
	//	*AnalysisLookupRequest_ExternalId
	//	*AnalysisLookupRequest_UserId
	//	*AnalysisLookupRequest_Username
	LookupIds      isAnalysisLookupRequest_LookupIds `protobuf_oneof:"lookup_ids"`
	Header         *header.Header                    `protobuf:"bytes,5,opt,name=header,proto3" json:"header,omitempty"`
	RequestingUser string                            `protobuf:"bytes,6,opt,name=requesting_user,json=requestingUser,proto3" json:"requesting_user,omitempty"`
}

func (x *AnalysisLookupRequest) Reset() {
	*x = AnalysisLookupRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_analysis_requests_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AnalysisLookupRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AnalysisLookupRequest) ProtoMessage() {}

func (x *AnalysisLookupRequest) ProtoReflect() protoreflect.Message {
	mi := &file_analysis_requests_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AnalysisLookupRequest.ProtoReflect.Descriptor instead.
func (*AnalysisLookupRequest) Descriptor() ([]byte, []int) {
	return file_analysis_requests_proto_rawDescGZIP(), []int{0}
}

func (m *AnalysisLookupRequest) GetLookupIds() isAnalysisLookupRequest_LookupIds {
	if m != nil {
		return m.LookupIds
	}
	return nil
}

func (x *AnalysisLookupRequest) GetAnalysisId() string {
	if x, ok := x.GetLookupIds().(*AnalysisLookupRequest_AnalysisId); ok {
		return x.AnalysisId
	}
	return ""
}

func (x *AnalysisLookupRequest) GetExternalId() string {
	if x, ok := x.GetLookupIds().(*AnalysisLookupRequest_ExternalId); ok {
		return x.ExternalId
	}
	return ""
}

func (x *AnalysisLookupRequest) GetUserId() string {
	if x, ok := x.GetLookupIds().(*AnalysisLookupRequest_UserId); ok {
		return x.UserId
	}
	return ""
}

func (x *AnalysisLookupRequest) GetUsername() string {
	if x, ok := x.GetLookupIds().(*AnalysisLookupRequest_Username); ok {
		return x.Username
	}
	return ""
}

func (x *AnalysisLookupRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AnalysisLookupRequest) GetRequestingUser() string {
	if x != nil {
		return x.RequestingUser
	}
	return ""
}

type isAnalysisLookupRequest_LookupIds interface {
	isAnalysisLookupRequest_LookupIds()
}

type AnalysisLookupRequest_AnalysisId struct {
	AnalysisId string `protobuf:"bytes,1,opt,name=analysis_id,json=analysisId,proto3,oneof"`
}

type AnalysisLookupRequest_ExternalId struct {
	ExternalId string `protobuf:"bytes,2,opt,name=external_id,json=externalId,proto3,oneof"`
}

type AnalysisLookupRequest_UserId struct {
	UserId string `protobuf:"bytes,3,opt,name=user_id,json=userId,proto3,oneof"`
}

type AnalysisLookupRequest_Username struct {
	Username string `protobuf:"bytes,4,opt,name=username,proto3,oneof"`
}

func (*AnalysisLookupRequest_AnalysisId) isAnalysisLookupRequest_LookupIds() {}

func (*AnalysisLookupRequest_ExternalId) isAnalysisLookupRequest_LookupIds() {}

func (*AnalysisLookupRequest_UserId) isAnalysisLookupRequest_LookupIds() {}

func (*AnalysisLookupRequest_Username) isAnalysisLookupRequest_LookupIds() {}

var File_analysis_requests_proto protoreflect.FileDescriptor

var file_analysis_requests_proto_rawDesc = []byte{
	0x0a, 0x17, 0x61, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69, 0x73, 0x2f, 0x72, 0x65, 0x71, 0x75, 0x65,
	0x73, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x2f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xee,
	0x01, 0x0a, 0x15, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69, 0x73, 0x4c, 0x6f, 0x6f, 0x6b, 0x75,
	0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x0b, 0x61, 0x6e, 0x61, 0x6c,
	0x79, 0x73, 0x69, 0x73, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52,
	0x0a, 0x61, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69, 0x73, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0b, 0x65,
	0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x48, 0x00, 0x52, 0x0a, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x12, 0x19,
	0x0a, 0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48,
	0x00, 0x52, 0x06, 0x75, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x08, 0x75, 0x73, 0x65,
	0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08, 0x75,
	0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x27, 0x0a, 0x0f, 0x72, 0x65, 0x71, 0x75,
	0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x0e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x55, 0x73, 0x65,
	0x72, 0x42, 0x0c, 0x0a, 0x0a, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x73, 0x42,
	0x25, 0x5a, 0x23, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79,
	0x76, 0x65, 0x72, 0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x61, 0x6e,
	0x61, 0x6c, 0x79, 0x73, 0x69, 0x73, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_analysis_requests_proto_rawDescOnce sync.Once
	file_analysis_requests_proto_rawDescData = file_analysis_requests_proto_rawDesc
)

func file_analysis_requests_proto_rawDescGZIP() []byte {
	file_analysis_requests_proto_rawDescOnce.Do(func() {
		file_analysis_requests_proto_rawDescData = protoimpl.X.CompressGZIP(file_analysis_requests_proto_rawDescData)
	})
	return file_analysis_requests_proto_rawDescData
}

var file_analysis_requests_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_analysis_requests_proto_goTypes = []interface{}{
	(*AnalysisLookupRequest)(nil), // 0: AnalysisLookupRequest
	(*header.Header)(nil),         // 1: Header
}
var file_analysis_requests_proto_depIdxs = []int32{
	1, // 0: AnalysisLookupRequest.header:type_name -> Header
	1, // [1:1] is the sub-list for method output_type
	1, // [1:1] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_analysis_requests_proto_init() }
func file_analysis_requests_proto_init() {
	if File_analysis_requests_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_analysis_requests_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AnalysisLookupRequest); i {
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
	file_analysis_requests_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*AnalysisLookupRequest_AnalysisId)(nil),
		(*AnalysisLookupRequest_ExternalId)(nil),
		(*AnalysisLookupRequest_UserId)(nil),
		(*AnalysisLookupRequest_Username)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_analysis_requests_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_analysis_requests_proto_goTypes,
		DependencyIndexes: file_analysis_requests_proto_depIdxs,
		MessageInfos:      file_analysis_requests_proto_msgTypes,
	}.Build()
	File_analysis_requests_proto = out.File
	file_analysis_requests_proto_rawDesc = nil
	file_analysis_requests_proto_goTypes = nil
	file_analysis_requests_proto_depIdxs = nil
}
