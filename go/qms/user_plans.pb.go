// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.0
// 	protoc        v3.19.4
// source: qms/user_plans.proto

package qms

import (
	header "github.com/cyverse-de/p/go/header"
	svcerror "github.com/cyverse-de/p/go/svcerror"
	qms "github.com/cyverse-de/p/gp/qms"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type UserPlan struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Uuid               string                 `protobuf:"bytes,1,opt,name=uuid,proto3" json:"uuid,omitempty"`
	EffectiveStartDate *timestamppb.Timestamp `protobuf:"bytes,2,opt,name=effective_start_date,proto3" json:"effective_start_date,omitempty"`
	EffectiveEndDate   *timestamppb.Timestamp `protobuf:"bytes,3,opt,name=effective_end_date,proto3" json:"effective_end_date,omitempty"`
	User               *QMSUser               `protobuf:"bytes,4,opt,name=user,proto3" json:"user,omitempty"`
	Plan               *Plan                  `protobuf:"bytes,5,opt,name=plan,proto3" json:"plan,omitempty"`
	Quotas             []*qms.Quota           `protobuf:"bytes,6,rep,name=quotas,proto3" json:"quotas,omitempty"`
	Usages             []*Usage               `protobuf:"bytes,7,rep,name=usages,proto3" json:"usages,omitempty"`
}

func (x *UserPlan) Reset() {
	*x = UserPlan{}
	if protoimpl.UnsafeEnabled {
		mi := &file_qms_user_plans_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UserPlan) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UserPlan) ProtoMessage() {}

func (x *UserPlan) ProtoReflect() protoreflect.Message {
	mi := &file_qms_user_plans_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UserPlan.ProtoReflect.Descriptor instead.
func (*UserPlan) Descriptor() ([]byte, []int) {
	return file_qms_user_plans_proto_rawDescGZIP(), []int{0}
}

func (x *UserPlan) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *UserPlan) GetEffectiveStartDate() *timestamppb.Timestamp {
	if x != nil {
		return x.EffectiveStartDate
	}
	return nil
}

func (x *UserPlan) GetEffectiveEndDate() *timestamppb.Timestamp {
	if x != nil {
		return x.EffectiveEndDate
	}
	return nil
}

func (x *UserPlan) GetUser() *QMSUser {
	if x != nil {
		return x.User
	}
	return nil
}

func (x *UserPlan) GetPlan() *Plan {
	if x != nil {
		return x.Plan
	}
	return nil
}

func (x *UserPlan) GetQuotas() []*qms.Quota {
	if x != nil {
		return x.Quotas
	}
	return nil
}

func (x *UserPlan) GetUsages() []*Usage {
	if x != nil {
		return x.Usages
	}
	return nil
}

type UserPlanResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Header   *header.Header         `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	Error    *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	UserPlan *UserPlan              `protobuf:"bytes,3,opt,name=user_plan,proto3" json:"user_plan,omitempty"`
}

func (x *UserPlanResponse) Reset() {
	*x = UserPlanResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_qms_user_plans_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UserPlanResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UserPlanResponse) ProtoMessage() {}

func (x *UserPlanResponse) ProtoReflect() protoreflect.Message {
	mi := &file_qms_user_plans_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UserPlanResponse.ProtoReflect.Descriptor instead.
func (*UserPlanResponse) Descriptor() ([]byte, []int) {
	return file_qms_user_plans_proto_rawDescGZIP(), []int{1}
}

func (x *UserPlanResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *UserPlanResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *UserPlanResponse) GetUserPlan() *UserPlan {
	if x != nil {
		return x.UserPlan
	}
	return nil
}

type UserPlanList struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Header    *header.Header         `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	Error     *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	UserPlans []*UserPlan            `protobuf:"bytes,3,rep,name=user_plans,proto3" json:"user_plans,omitempty"`
}

func (x *UserPlanList) Reset() {
	*x = UserPlanList{}
	if protoimpl.UnsafeEnabled {
		mi := &file_qms_user_plans_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UserPlanList) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UserPlanList) ProtoMessage() {}

func (x *UserPlanList) ProtoReflect() protoreflect.Message {
	mi := &file_qms_user_plans_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UserPlanList.ProtoReflect.Descriptor instead.
func (*UserPlanList) Descriptor() ([]byte, []int) {
	return file_qms_user_plans_proto_rawDescGZIP(), []int{2}
}

func (x *UserPlanList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *UserPlanList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *UserPlanList) GetUserPlans() []*UserPlan {
	if x != nil {
		return x.UserPlans
	}
	return nil
}

var File_qms_user_plans_proto protoreflect.FileDescriptor

var file_qms_user_plans_proto_rawDesc = []byte{
	0x0a, 0x14, 0x71, 0x6d, 0x73, 0x2f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x70, 0x6c, 0x61, 0x6e, 0x73,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
	0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2f,
	0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x73, 0x76,
	0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2f, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x71, 0x6d, 0x73, 0x2f, 0x75, 0x73, 0x65, 0x72, 0x73,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x71, 0x6d, 0x73, 0x2f, 0x70, 0x6c, 0x61, 0x6e,
	0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x10, 0x71, 0x6d, 0x73, 0x2f, 0x71, 0x75, 0x6f,
	0x74, 0x61, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x10, 0x71, 0x6d, 0x73, 0x2f, 0x75,
	0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb3, 0x02, 0x0a, 0x08,
	0x55, 0x73, 0x65, 0x72, 0x50, 0x6c, 0x61, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x4e, 0x0a, 0x14,
	0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f,
	0x64, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f,
	0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d,
	0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x14, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76,
	0x65, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x12, 0x4a, 0x0a, 0x12,
	0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x64, 0x61,
	0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
	0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73,
	0x74, 0x61, 0x6d, 0x70, 0x52, 0x12, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f,
	0x65, 0x6e, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x12, 0x1c, 0x0a, 0x04, 0x75, 0x73, 0x65, 0x72,
	0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x51, 0x4d, 0x53, 0x55, 0x73, 0x65, 0x72,
	0x52, 0x04, 0x75, 0x73, 0x65, 0x72, 0x12, 0x19, 0x0a, 0x04, 0x70, 0x6c, 0x61, 0x6e, 0x18, 0x05,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x50, 0x6c, 0x61, 0x6e, 0x52, 0x04, 0x70, 0x6c, 0x61,
	0x6e, 0x12, 0x1e, 0x0a, 0x06, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x06, 0x2e, 0x51, 0x75, 0x6f, 0x74, 0x61, 0x52, 0x06, 0x71, 0x75, 0x6f, 0x74, 0x61,
	0x73, 0x12, 0x1e, 0x0a, 0x06, 0x75, 0x73, 0x61, 0x67, 0x65, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x06, 0x2e, 0x55, 0x73, 0x61, 0x67, 0x65, 0x52, 0x06, 0x75, 0x73, 0x61, 0x67, 0x65,
	0x73, 0x22, 0x81, 0x01, 0x0a, 0x10, 0x55, 0x73, 0x65, 0x72, 0x50, 0x6c, 0x61, 0x6e, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52,
	0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x23, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
	0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x27, 0x0a, 0x09,
	0x75, 0x73, 0x65, 0x72, 0x5f, 0x70, 0x6c, 0x61, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x09, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x50, 0x6c, 0x61, 0x6e, 0x52, 0x09, 0x75, 0x73, 0x65, 0x72,
	0x5f, 0x70, 0x6c, 0x61, 0x6e, 0x22, 0x7f, 0x0a, 0x0c, 0x55, 0x73, 0x65, 0x72, 0x50, 0x6c, 0x61,
	0x6e, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x1f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06,
	0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x23, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45,
	0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x29, 0x0a, 0x0a, 0x75,
	0x73, 0x65, 0x72, 0x5f, 0x70, 0x6c, 0x61, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
	0x09, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x50, 0x6c, 0x61, 0x6e, 0x52, 0x0a, 0x75, 0x73, 0x65, 0x72,
	0x5f, 0x70, 0x6c, 0x61, 0x6e, 0x73, 0x42, 0x20, 0x5a, 0x1e, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f,
	0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x71, 0x6d, 0x73, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_qms_user_plans_proto_rawDescOnce sync.Once
	file_qms_user_plans_proto_rawDescData = file_qms_user_plans_proto_rawDesc
)

func file_qms_user_plans_proto_rawDescGZIP() []byte {
	file_qms_user_plans_proto_rawDescOnce.Do(func() {
		file_qms_user_plans_proto_rawDescData = protoimpl.X.CompressGZIP(file_qms_user_plans_proto_rawDescData)
	})
	return file_qms_user_plans_proto_rawDescData
}

var file_qms_user_plans_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_qms_user_plans_proto_goTypes = []interface{}{
	(*UserPlan)(nil),              // 0: UserPlan
	(*UserPlanResponse)(nil),      // 1: UserPlanResponse
	(*UserPlanList)(nil),          // 2: UserPlanList
	(*timestamppb.Timestamp)(nil), // 3: google.protobuf.Timestamp
	(*QMSUser)(nil),               // 4: QMSUser
	(*Plan)(nil),                  // 5: Plan
	(*qms.Quota)(nil),             // 6: Quota
	(*Usage)(nil),                 // 7: Usage
	(*header.Header)(nil),         // 8: Header
	(*svcerror.ServiceError)(nil), // 9: ServiceError
}
var file_qms_user_plans_proto_depIdxs = []int32{
	3,  // 0: UserPlan.effective_start_date:type_name -> google.protobuf.Timestamp
	3,  // 1: UserPlan.effective_end_date:type_name -> google.protobuf.Timestamp
	4,  // 2: UserPlan.user:type_name -> QMSUser
	5,  // 3: UserPlan.plan:type_name -> Plan
	6,  // 4: UserPlan.quotas:type_name -> Quota
	7,  // 5: UserPlan.usages:type_name -> Usage
	8,  // 6: UserPlanResponse.header:type_name -> Header
	9,  // 7: UserPlanResponse.error:type_name -> ServiceError
	0,  // 8: UserPlanResponse.user_plan:type_name -> UserPlan
	8,  // 9: UserPlanList.header:type_name -> Header
	9,  // 10: UserPlanList.error:type_name -> ServiceError
	0,  // 11: UserPlanList.user_plans:type_name -> UserPlan
	12, // [12:12] is the sub-list for method output_type
	12, // [12:12] is the sub-list for method input_type
	12, // [12:12] is the sub-list for extension type_name
	12, // [12:12] is the sub-list for extension extendee
	0,  // [0:12] is the sub-list for field type_name
}

func init() { file_qms_user_plans_proto_init() }
func file_qms_user_plans_proto_init() {
	if File_qms_user_plans_proto != nil {
		return
	}
	file_qms_users_proto_init()
	file_qms_plans_proto_init()
	file_qms_usages_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_qms_user_plans_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UserPlan); i {
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
		file_qms_user_plans_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UserPlanResponse); i {
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
		file_qms_user_plans_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UserPlanList); i {
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
			RawDescriptor: file_qms_user_plans_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_qms_user_plans_proto_goTypes,
		DependencyIndexes: file_qms_user_plans_proto_depIdxs,
		MessageInfos:      file_qms_user_plans_proto_msgTypes,
	}.Build()
	File_qms_user_plans_proto = out.File
	file_qms_user_plans_proto_rawDesc = nil
	file_qms_user_plans_proto_goTypes = nil
	file_qms_user_plans_proto_depIdxs = nil
}