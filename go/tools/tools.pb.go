// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.30.0
// 	protoc        v3.21.12
// source: tools.proto

package tools

import (
	apps "github.com/cyverse-de/p/go/apps"
	containers "github.com/cyverse-de/p/go/containers"
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
// A tool integrated into the system.
//
// A tool is part of an app and can run in an execution environment.
// Mostly correllates to the 'tools' table in the 'de' database.
type Tool struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The UUID for the tool.
	Uuid string `protobuf:"bytes,1,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// The name of the tool.
	Name string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	// The version of the tool.
	Version string `protobuf:"bytes,3,opt,name=version,proto3" json:"version,omitempty"`
	// Attribution information for the tool.
	Attribution string `protobuf:"bytes,4,opt,name=attribution,proto3" json:"attribution,omitempty"`
	// The description of the tool.
	Description string `protobuf:"bytes,5,opt,name=description,proto3" json:"description,omitempty"`
	// The time limit the tool is allowed to run for by default. Unit is seconds.
	TimeLimitSeconds int32 `protobuf:"varint,6,opt,name=time_limit_seconds,proto3" json:"time_limit_seconds,omitempty"`
	// Whether the tool is restricted.
	Restricted bool `protobuf:"varint,7,opt,name=restricted,proto3" json:"restricted,omitempty"`
	// Whether the tool is interactive (i.e. whether it is intended for use in VICE).
	Interactive bool `protobuf:"varint,8,opt,name=interactive,proto3" json:"interactive,omitempty"`
	// Whether the tool requires a GPU.
	GpuEnabled bool `protobuf:"varint,9,opt,name=gpu_enabled,proto3" json:"gpu_enabled,omitempty"`
	// Integration data associated with the tool.
	IntegrationData *apps.IntegrationData `protobuf:"bytes,10,opt,name=integration_data,proto3" json:"integration_data,omitempty"`
	// The container image to use when running the tool.
	ContainerImage *containers.Image `protobuf:"bytes,11,opt,name=container_image,proto3" json:"container_image,omitempty"`
}

func (x *Tool) Reset() {
	*x = Tool{}
	if protoimpl.UnsafeEnabled {
		mi := &file_tools_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Tool) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Tool) ProtoMessage() {}

func (x *Tool) ProtoReflect() protoreflect.Message {
	mi := &file_tools_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Tool.ProtoReflect.Descriptor instead.
func (*Tool) Descriptor() ([]byte, []int) {
	return file_tools_proto_rawDescGZIP(), []int{0}
}

func (x *Tool) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *Tool) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Tool) GetVersion() string {
	if x != nil {
		return x.Version
	}
	return ""
}

func (x *Tool) GetAttribution() string {
	if x != nil {
		return x.Attribution
	}
	return ""
}

func (x *Tool) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Tool) GetTimeLimitSeconds() int32 {
	if x != nil {
		return x.TimeLimitSeconds
	}
	return 0
}

func (x *Tool) GetRestricted() bool {
	if x != nil {
		return x.Restricted
	}
	return false
}

func (x *Tool) GetInteractive() bool {
	if x != nil {
		return x.Interactive
	}
	return false
}

func (x *Tool) GetGpuEnabled() bool {
	if x != nil {
		return x.GpuEnabled
	}
	return false
}

func (x *Tool) GetIntegrationData() *apps.IntegrationData {
	if x != nil {
		return x.IntegrationData
	}
	return nil
}

func (x *Tool) GetContainerImage() *containers.Image {
	if x != nil {
		return x.ContainerImage
	}
	return nil
}

var File_tools_proto protoreflect.FileDescriptor

var file_tools_proto_rawDesc = []byte{
	0x0a, 0x0b, 0x74, 0x6f, 0x6f, 0x6c, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x74,
	0x6f, 0x6f, 0x6c, 0x73, 0x1a, 0x0a, 0x61, 0x70, 0x70, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x1a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0xa0, 0x03, 0x0a, 0x04, 0x54, 0x6f, 0x6f, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x75,
	0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12,
	0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
	0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x20, 0x0a,
	0x0b, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x0b, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x12,
	0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
	0x6e, 0x12, 0x2e, 0x0a, 0x12, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f,
	0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x12, 0x74,
	0x69, 0x6d, 0x65, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
	0x73, 0x12, 0x1e, 0x0a, 0x0a, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x65, 0x64, 0x18,
	0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x65,
	0x64, 0x12, 0x20, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65,
	0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74,
	0x69, 0x76, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x67, 0x70, 0x75, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c,
	0x65, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x67, 0x70, 0x75, 0x5f, 0x65, 0x6e,
	0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x41, 0x0a, 0x10, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x72, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x15, 0x2e, 0x61, 0x70, 0x70, 0x73, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x72, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x10, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x72, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x12, 0x3b, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x74,
	0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x11, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x73, 0x2e, 0x49,
	0x6d, 0x61, 0x67, 0x65, 0x52, 0x0f, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f,
	0x69, 0x6d, 0x61, 0x67, 0x65, 0x42, 0x4e, 0x0a, 0x18, 0x6f, 0x72, 0x67, 0x2e, 0x63, 0x79, 0x76,
	0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
	0x73, 0x42, 0x0e, 0x54, 0x6f, 0x6f, 0x6c, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
	0x73, 0x50, 0x01, 0x5a, 0x20, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f,
	0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f,
	0x74, 0x6f, 0x6f, 0x6c, 0x73, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_tools_proto_rawDescOnce sync.Once
	file_tools_proto_rawDescData = file_tools_proto_rawDesc
)

func file_tools_proto_rawDescGZIP() []byte {
	file_tools_proto_rawDescOnce.Do(func() {
		file_tools_proto_rawDescData = protoimpl.X.CompressGZIP(file_tools_proto_rawDescData)
	})
	return file_tools_proto_rawDescData
}

var file_tools_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_tools_proto_goTypes = []interface{}{
	(*Tool)(nil),                 // 0: tools.Tool
	(*apps.IntegrationData)(nil), // 1: apps.IntegrationData
	(*containers.Image)(nil),     // 2: containers.Image
}
var file_tools_proto_depIdxs = []int32{
	1, // 0: tools.Tool.integration_data:type_name -> apps.IntegrationData
	2, // 1: tools.Tool.container_image:type_name -> containers.Image
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_tools_proto_init() }
func file_tools_proto_init() {
	if File_tools_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_tools_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Tool); i {
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
			RawDescriptor: file_tools_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_tools_proto_goTypes,
		DependencyIndexes: file_tools_proto_depIdxs,
		MessageInfos:      file_tools_proto_msgTypes,
	}.Build()
	File_tools_proto = out.File
	file_tools_proto_rawDesc = nil
	file_tools_proto_goTypes = nil
	file_tools_proto_depIdxs = nil
}
