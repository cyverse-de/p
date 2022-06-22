// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.0
// 	protoc        v3.19.4
// source: qms/quotas.proto

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

type Quota struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Uuid         string        `protobuf:"bytes,1,opt,name=uuid,proto3" json:"uuid,omitempty"`
	Quota        float32       `protobuf:"fixed32,2,opt,name=quota,proto3" json:"quota,omitempty"`
	ResourceType *ResourceType `protobuf:"bytes,3,opt,name=resource_type,proto3" json:"resource_type,omitempty"`
}

func (x *Quota) Reset() {
	*x = Quota{}
	if protoimpl.UnsafeEnabled {
		mi := &file_qms_quotas_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Quota) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Quota) ProtoMessage() {}

func (x *Quota) ProtoReflect() protoreflect.Message {
	mi := &file_qms_quotas_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Quota.ProtoReflect.Descriptor instead.
func (*Quota) Descriptor() ([]byte, []int) {
	return file_qms_quotas_proto_rawDescGZIP(), []int{0}
}

func (x *Quota) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *Quota) GetQuota() float32 {
	if x != nil {
		return x.Quota
	}
	return 0
}

func (x *Quota) GetResourceType() *ResourceType {
	if x != nil {
		return x.ResourceType
	}
	return nil
}

type QuotaResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Header *header.Header         `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	Error  *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	Quota  *Quota                 `protobuf:"bytes,3,opt,name=quota,proto3" json:"quota,omitempty"`
}

func (x *QuotaResponse) Reset() {
	*x = QuotaResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_qms_quotas_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *QuotaResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QuotaResponse) ProtoMessage() {}

func (x *QuotaResponse) ProtoReflect() protoreflect.Message {
	mi := &file_qms_quotas_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use QuotaResponse.ProtoReflect.Descriptor instead.
func (*QuotaResponse) Descriptor() ([]byte, []int) {
	return file_qms_quotas_proto_rawDescGZIP(), []int{1}
}

func (x *QuotaResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *QuotaResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *QuotaResponse) GetQuota() *Quota {
	if x != nil {
		return x.Quota
	}
	return nil
}

type QuotaList struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Header *header.Header         `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	Error  *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	Quotas []*Quota               `protobuf:"bytes,3,rep,name=quotas,proto3" json:"quotas,omitempty"`
}

func (x *QuotaList) Reset() {
	*x = QuotaList{}
	if protoimpl.UnsafeEnabled {
		mi := &file_qms_quotas_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *QuotaList) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QuotaList) ProtoMessage() {}

func (x *QuotaList) ProtoReflect() protoreflect.Message {
	mi := &file_qms_quotas_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use QuotaList.ProtoReflect.Descriptor instead.
func (*QuotaList) Descriptor() ([]byte, []int) {
	return file_qms_quotas_proto_rawDescGZIP(), []int{2}
}

func (x *QuotaList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *QuotaList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *QuotaList) GetQuotas() []*Quota {
	if x != nil {
		return x.Quotas
	}
	return nil
}

var File_qms_quotas_proto protoreflect.FileDescriptor

var file_qms_quotas_proto_rawDesc = []byte{
	0x0a, 0x10, 0x71, 0x6d, 0x73, 0x2f, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x73, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x1a, 0x13, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2f, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f,
	0x72, 0x2f, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x1a, 0x18, 0x71, 0x6d, 0x73, 0x2f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x74,
	0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x66, 0x0a, 0x05, 0x51, 0x75,
	0x6f, 0x74, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x71, 0x75, 0x6f, 0x74, 0x61,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x12, 0x33, 0x0a,
	0x0d, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54,
	0x79, 0x70, 0x65, 0x52, 0x0d, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x74, 0x79,
	0x70, 0x65, 0x22, 0x73, 0x0a, 0x0d, 0x51, 0x75, 0x6f, 0x74, 0x61, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x1f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65,
	0x61, 0x64, 0x65, 0x72, 0x12, 0x23, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72, 0x72,
	0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x1c, 0x0a, 0x05, 0x71, 0x75, 0x6f,
	0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x51, 0x75, 0x6f, 0x74, 0x61,
	0x52, 0x05, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x22, 0x71, 0x0a, 0x09, 0x51, 0x75, 0x6f, 0x74, 0x61,
	0x4c, 0x69, 0x73, 0x74, 0x12, 0x1f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68,
	0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x23, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72,
	0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x1e, 0x0a, 0x06, 0x71, 0x75,
	0x6f, 0x74, 0x61, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x51, 0x75, 0x6f,
	0x74, 0x61, 0x52, 0x06, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x73, 0x42, 0x20, 0x5a, 0x1e, 0x67, 0x69,
	0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65,
	0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x71, 0x6d, 0x73, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_qms_quotas_proto_rawDescOnce sync.Once
	file_qms_quotas_proto_rawDescData = file_qms_quotas_proto_rawDesc
)

func file_qms_quotas_proto_rawDescGZIP() []byte {
	file_qms_quotas_proto_rawDescOnce.Do(func() {
		file_qms_quotas_proto_rawDescData = protoimpl.X.CompressGZIP(file_qms_quotas_proto_rawDescData)
	})
	return file_qms_quotas_proto_rawDescData
}

var file_qms_quotas_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_qms_quotas_proto_goTypes = []interface{}{
	(*Quota)(nil),                 // 0: Quota
	(*QuotaResponse)(nil),         // 1: QuotaResponse
	(*QuotaList)(nil),             // 2: QuotaList
	(*ResourceType)(nil),          // 3: ResourceType
	(*header.Header)(nil),         // 4: Header
	(*svcerror.ServiceError)(nil), // 5: ServiceError
}
var file_qms_quotas_proto_depIdxs = []int32{
	3, // 0: Quota.resource_type:type_name -> ResourceType
	4, // 1: QuotaResponse.header:type_name -> Header
	5, // 2: QuotaResponse.error:type_name -> ServiceError
	0, // 3: QuotaResponse.quota:type_name -> Quota
	4, // 4: QuotaList.header:type_name -> Header
	5, // 5: QuotaList.error:type_name -> ServiceError
	0, // 6: QuotaList.quotas:type_name -> Quota
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_qms_quotas_proto_init() }
func file_qms_quotas_proto_init() {
	if File_qms_quotas_proto != nil {
		return
	}
	file_qms_resource_types_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_qms_quotas_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Quota); i {
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
		file_qms_quotas_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*QuotaResponse); i {
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
		file_qms_quotas_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*QuotaList); i {
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
			RawDescriptor: file_qms_quotas_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_qms_quotas_proto_goTypes,
		DependencyIndexes: file_qms_quotas_proto_depIdxs,
		MessageInfos:      file_qms_quotas_proto_msgTypes,
	}.Build()
	File_qms_quotas_proto = out.File
	file_qms_quotas_proto_rawDesc = nil
	file_qms_quotas_proto_goTypes = nil
	file_qms_quotas_proto_depIdxs = nil
}
