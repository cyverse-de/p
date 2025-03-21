// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.1
// 	protoc        v5.29.3
// source: qms_overages.proto

package qms

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

// *
// Represents when a user's resource type usage exceeds their configured
// quota. Usually embedded in request and response message types.
type Overage struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The type of resource that is in overage. Usually data.size of cpu.hours.
	ResourceName string `protobuf:"bytes,1,opt,name=resource_name,proto3" json:"resource_name,omitempty"`
	// The configured quota value for the resource type.
	Quota float64 `protobuf:"fixed64,2,opt,name=quota,proto3" json:"quota,omitempty"`
	// The actual usage value for the resource type.
	Usage float64 `protobuf:"fixed64,3,opt,name=usage,proto3" json:"usage,omitempty"`
}

func (x *Overage) Reset() {
	*x = Overage{}
	mi := &file_qms_overages_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Overage) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Overage) ProtoMessage() {}

func (x *Overage) ProtoReflect() protoreflect.Message {
	mi := &file_qms_overages_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Overage.ProtoReflect.Descriptor instead.
func (*Overage) Descriptor() ([]byte, []int) {
	return file_qms_overages_proto_rawDescGZIP(), []int{0}
}

func (x *Overage) GetResourceName() string {
	if x != nil {
		return x.ResourceName
	}
	return ""
}

func (x *Overage) GetQuota() float64 {
	if x != nil {
		return x.Quota
	}
	return 0
}

func (x *Overage) GetUsage() float64 {
	if x != nil {
		return x.Usage
	}
	return 0
}

// *
// Returned by handlers in response to overage requests.
type OverageResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The header used for passing telemetry data.
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// Contains any errors generated by the handler emitting the response.
	Error *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	// The overage returned by the handler emitting the response.
	Overage *Overage `protobuf:"bytes,3,opt,name=overage,proto3" json:"overage,omitempty"`
}

func (x *OverageResponse) Reset() {
	*x = OverageResponse{}
	mi := &file_qms_overages_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *OverageResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*OverageResponse) ProtoMessage() {}

func (x *OverageResponse) ProtoReflect() protoreflect.Message {
	mi := &file_qms_overages_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use OverageResponse.ProtoReflect.Descriptor instead.
func (*OverageResponse) Descriptor() ([]byte, []int) {
	return file_qms_overages_proto_rawDescGZIP(), []int{1}
}

func (x *OverageResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *OverageResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *OverageResponse) GetOverage() *Overage {
	if x != nil {
		return x.Overage
	}
	return nil
}

// *
// A response message returned by handlers in response to overage related requests.
type OverageList struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The header used for passing telemetry data.
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// Contains any errors generated by the handler emitting the response.
	Error *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	// The list of overages returned by the handler emitting the response.
	Overages []*Overage `protobuf:"bytes,3,rep,name=overages,proto3" json:"overages,omitempty"`
}

func (x *OverageList) Reset() {
	*x = OverageList{}
	mi := &file_qms_overages_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *OverageList) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*OverageList) ProtoMessage() {}

func (x *OverageList) ProtoReflect() protoreflect.Message {
	mi := &file_qms_overages_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use OverageList.ProtoReflect.Descriptor instead.
func (*OverageList) Descriptor() ([]byte, []int) {
	return file_qms_overages_proto_rawDescGZIP(), []int{2}
}

func (x *OverageList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *OverageList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *OverageList) GetOverages() []*Overage {
	if x != nil {
		return x.Overages
	}
	return nil
}

// *
// A response message returned by handlers in response to overage related requests.
type IsOverage struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The header userd for passing telemetry data.
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// Contains any errors generated by the handler emitting the response.
	Error *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	// Whether or not there is an overage.
	IsOverage bool `protobuf:"varint,3,opt,name=is_overage,proto3" json:"is_overage,omitempty"`
}

func (x *IsOverage) Reset() {
	*x = IsOverage{}
	mi := &file_qms_overages_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *IsOverage) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IsOverage) ProtoMessage() {}

func (x *IsOverage) ProtoReflect() protoreflect.Message {
	mi := &file_qms_overages_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IsOverage.ProtoReflect.Descriptor instead.
func (*IsOverage) Descriptor() ([]byte, []int) {
	return file_qms_overages_proto_rawDescGZIP(), []int{3}
}

func (x *IsOverage) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *IsOverage) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *IsOverage) GetIsOverage() bool {
	if x != nil {
		return x.IsOverage
	}
	return false
}

var File_qms_overages_proto protoreflect.FileDescriptor

var file_qms_overages_proto_rawDesc = []byte{
	0x0a, 0x12, 0x71, 0x6d, 0x73, 0x5f, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x12, 0x03, 0x71, 0x6d, 0x73, 0x1a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f,
	0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a, 0x07, 0x4f, 0x76, 0x65, 0x72, 0x61,
	0x67, 0x65, 0x12, 0x24, 0x0a, 0x0d, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6e,
	0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x72, 0x65, 0x73, 0x6f, 0x75,
	0x72, 0x63, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x71, 0x75, 0x6f, 0x74,
	0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x12, 0x14,
	0x0a, 0x05, 0x75, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x75,
	0x73, 0x61, 0x67, 0x65, 0x22, 0x8f, 0x01, 0x0a, 0x0f, 0x4f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x12, 0x2c, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x16, 0x2e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69,
	0x63, 0x65, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x26,
	0x0a, 0x07, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x0c, 0x2e, 0x71, 0x6d, 0x73, 0x2e, 0x4f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x52, 0x07, 0x6f,
	0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x22, 0x8d, 0x01, 0x0a, 0x0b, 0x4f, 0x76, 0x65, 0x72, 0x61,
	0x67, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e,
	0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c,
	0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e,
	0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
	0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x28, 0x0a, 0x08,
	0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c,
	0x2e, 0x71, 0x6d, 0x73, 0x2e, 0x4f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x52, 0x08, 0x6f, 0x76,
	0x65, 0x72, 0x61, 0x67, 0x65, 0x73, 0x22, 0x81, 0x01, 0x0a, 0x09, 0x49, 0x73, 0x4f, 0x76, 0x65,
	0x72, 0x61, 0x67, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65,
	0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x05,
	0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x76,
	0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72,
	0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x1e, 0x0a, 0x0a, 0x69, 0x73,
	0x5f, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a,
	0x69, 0x73, 0x5f, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x42, 0x52, 0x0a, 0x18, 0x6f, 0x72,
	0x67, 0x2e, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x42, 0x14, 0x51, 0x4d, 0x53, 0x4f, 0x76, 0x65, 0x72, 0x61,
	0x67, 0x65, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x50, 0x01, 0x5a, 0x1e,
	0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72,
	0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x71, 0x6d, 0x73, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_qms_overages_proto_rawDescOnce sync.Once
	file_qms_overages_proto_rawDescData = file_qms_overages_proto_rawDesc
)

func file_qms_overages_proto_rawDescGZIP() []byte {
	file_qms_overages_proto_rawDescOnce.Do(func() {
		file_qms_overages_proto_rawDescData = protoimpl.X.CompressGZIP(file_qms_overages_proto_rawDescData)
	})
	return file_qms_overages_proto_rawDescData
}

var file_qms_overages_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_qms_overages_proto_goTypes = []any{
	(*Overage)(nil),               // 0: qms.Overage
	(*OverageResponse)(nil),       // 1: qms.OverageResponse
	(*OverageList)(nil),           // 2: qms.OverageList
	(*IsOverage)(nil),             // 3: qms.IsOverage
	(*header.Header)(nil),         // 4: header.Header
	(*svcerror.ServiceError)(nil), // 5: svcerror.ServiceError
}
var file_qms_overages_proto_depIdxs = []int32{
	4, // 0: qms.OverageResponse.header:type_name -> header.Header
	5, // 1: qms.OverageResponse.error:type_name -> svcerror.ServiceError
	0, // 2: qms.OverageResponse.overage:type_name -> qms.Overage
	4, // 3: qms.OverageList.header:type_name -> header.Header
	5, // 4: qms.OverageList.error:type_name -> svcerror.ServiceError
	0, // 5: qms.OverageList.overages:type_name -> qms.Overage
	4, // 6: qms.IsOverage.header:type_name -> header.Header
	5, // 7: qms.IsOverage.error:type_name -> svcerror.ServiceError
	8, // [8:8] is the sub-list for method output_type
	8, // [8:8] is the sub-list for method input_type
	8, // [8:8] is the sub-list for extension type_name
	8, // [8:8] is the sub-list for extension extendee
	0, // [0:8] is the sub-list for field type_name
}

func init() { file_qms_overages_proto_init() }
func file_qms_overages_proto_init() {
	if File_qms_overages_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_qms_overages_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_qms_overages_proto_goTypes,
		DependencyIndexes: file_qms_overages_proto_depIdxs,
		MessageInfos:      file_qms_overages_proto_msgTypes,
	}.Build()
	File_qms_overages_proto = out.File
	file_qms_overages_proto_rawDesc = nil
	file_qms_overages_proto_goTypes = nil
	file_qms_overages_proto_depIdxs = nil
}
