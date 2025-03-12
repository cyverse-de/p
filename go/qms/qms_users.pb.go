// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.1
// 	protoc        v5.29.3
// source: qms_users.proto

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
// A representation of a user in the QMS system.
type QMSUser struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// A user's unique identifier in QMS.
	Uuid string `protobuf:"bytes,1,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// A user's username in QMS.
	Username string `protobuf:"bytes,2,opt,name=username,proto3" json:"username,omitempty"`
}

func (x *QMSUser) Reset() {
	*x = QMSUser{}
	mi := &file_qms_users_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *QMSUser) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QMSUser) ProtoMessage() {}

func (x *QMSUser) ProtoReflect() protoreflect.Message {
	mi := &file_qms_users_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use QMSUser.ProtoReflect.Descriptor instead.
func (*QMSUser) Descriptor() ([]byte, []int) {
	return file_qms_users_proto_rawDescGZIP(), []int{0}
}

func (x *QMSUser) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *QMSUser) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

// *
// A response to a request for info about a QMS user.
type QMSUserResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// Error information generated by the request handler.
	Error *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	// The user returned by the request handler.
	User *QMSUser `protobuf:"bytes,3,opt,name=user,proto3" json:"user,omitempty"`
}

func (x *QMSUserResponse) Reset() {
	*x = QMSUserResponse{}
	mi := &file_qms_users_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *QMSUserResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QMSUserResponse) ProtoMessage() {}

func (x *QMSUserResponse) ProtoReflect() protoreflect.Message {
	mi := &file_qms_users_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use QMSUserResponse.ProtoReflect.Descriptor instead.
func (*QMSUserResponse) Descriptor() ([]byte, []int) {
	return file_qms_users_proto_rawDescGZIP(), []int{1}
}

func (x *QMSUserResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *QMSUserResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *QMSUserResponse) GetUser() *QMSUser {
	if x != nil {
		return x.User
	}
	return nil
}

// *
//
//	A response to a request for info about a list of users.
type QMSUserList struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// Error information generated by the request handler.
	Error *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	// The user list returned by the request handler.
	Users []*QMSUser `protobuf:"bytes,3,rep,name=users,proto3" json:"users,omitempty"`
}

func (x *QMSUserList) Reset() {
	*x = QMSUserList{}
	mi := &file_qms_users_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *QMSUserList) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*QMSUserList) ProtoMessage() {}

func (x *QMSUserList) ProtoReflect() protoreflect.Message {
	mi := &file_qms_users_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use QMSUserList.ProtoReflect.Descriptor instead.
func (*QMSUserList) Descriptor() ([]byte, []int) {
	return file_qms_users_proto_rawDescGZIP(), []int{2}
}

func (x *QMSUserList) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *QMSUserList) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *QMSUserList) GetUsers() []*QMSUser {
	if x != nil {
		return x.Users
	}
	return nil
}

// *
// A request to add a user to the QMS system.
type AddUserRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// The username for the user being added to the system.
	Username string `protobuf:"bytes,3,opt,name=username,proto3" json:"username,omitempty"`
	// The name of the plan the user should be subscribed to.
	PlanName string `protobuf:"bytes,4,opt,name=plan_name,json=planName,proto3" json:"plan_name,omitempty"`
	// True if the user paid for the subscription.
	Paid bool `protobuf:"varint,5,opt,name=paid,proto3" json:"paid,omitempty"`
	// The number of subscription periods that the subscription will be good for. The subscription period is one year,
	// so purchasing a subscription for 3 periods will create a subscription for 3 years. Consumable resources are also
	// allocated based on the number of periods, so if a subscription plan comes with 2000 CPU Hours, for example, then
	// a user who purchases 3 subscription periods will get 6000 CPU hours to use over the course of three years.
	Periods int32 `protobuf:"varint,6,opt,name=periods,proto3" json:"periods,omitempty"`
	// The end-date of the subscription. Accepted formats are `YYYY-MM-DD`, `YYYY-MM-DDThh:mm:ss`,
	// `YYYY-MM-DDThh:mm:ssZ` and `YYYY-MM-DDThh:mm:ss+hh:mm`. Date and tiestamps without time zones are assumed to
	// be in the time zone used by the CyVerse Discovery Environment itself.
	EndDate string `protobuf:"bytes,7,opt,name=end_date,json=endDate,proto3" json:"end_date,omitempty"`
	// True if the user should get a new subscription even if they already have a subscription with the same plan.
	Force bool `protobuf:"varint,8,opt,name=force,proto3" json:"force,omitempty"`
}

func (x *AddUserRequest) Reset() {
	*x = AddUserRequest{}
	mi := &file_qms_users_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *AddUserRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AddUserRequest) ProtoMessage() {}

func (x *AddUserRequest) ProtoReflect() protoreflect.Message {
	mi := &file_qms_users_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AddUserRequest.ProtoReflect.Descriptor instead.
func (*AddUserRequest) Descriptor() ([]byte, []int) {
	return file_qms_users_proto_rawDescGZIP(), []int{3}
}

func (x *AddUserRequest) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AddUserRequest) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *AddUserRequest) GetPlanName() string {
	if x != nil {
		return x.PlanName
	}
	return ""
}

func (x *AddUserRequest) GetPaid() bool {
	if x != nil {
		return x.Paid
	}
	return false
}

func (x *AddUserRequest) GetPeriods() int32 {
	if x != nil {
		return x.Periods
	}
	return 0
}

func (x *AddUserRequest) GetEndDate() string {
	if x != nil {
		return x.EndDate
	}
	return ""
}

func (x *AddUserRequest) GetForce() bool {
	if x != nil {
		return x.Force
	}
	return false
}

// *
// A response to a request to add a user to the QMS system.
type AddUserResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// Error information generated by the request handler.
	Error *svcerror.ServiceError `protobuf:"bytes,2,opt,name=error,proto3" json:"error,omitempty"`
	// The unique identfier of the newly added user.
	Uuid string `protobuf:"bytes,3,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// The username of the newly added user.
	Username string `protobuf:"bytes,4,opt,name=username,proto3" json:"username,omitempty"`
	// The name of the plan the newly added user is subscribed to.
	PlanName string `protobuf:"bytes,5,opt,name=plan_name,json=planName,proto3" json:"plan_name,omitempty"`
	// The unique identifier for the plan the newly added user is subscribed to.
	PlanUuid string `protobuf:"bytes,6,opt,name=plan_uuid,json=planUuid,proto3" json:"plan_uuid,omitempty"`
}

func (x *AddUserResponse) Reset() {
	*x = AddUserResponse{}
	mi := &file_qms_users_proto_msgTypes[4]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *AddUserResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AddUserResponse) ProtoMessage() {}

func (x *AddUserResponse) ProtoReflect() protoreflect.Message {
	mi := &file_qms_users_proto_msgTypes[4]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AddUserResponse.ProtoReflect.Descriptor instead.
func (*AddUserResponse) Descriptor() ([]byte, []int) {
	return file_qms_users_proto_rawDescGZIP(), []int{4}
}

func (x *AddUserResponse) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AddUserResponse) GetError() *svcerror.ServiceError {
	if x != nil {
		return x.Error
	}
	return nil
}

func (x *AddUserResponse) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *AddUserResponse) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

func (x *AddUserResponse) GetPlanName() string {
	if x != nil {
		return x.PlanName
	}
	return ""
}

func (x *AddUserResponse) GetPlanUuid() string {
	if x != nil {
		return x.PlanUuid
	}
	return ""
}

var File_qms_users_proto protoreflect.FileDescriptor

var file_qms_users_proto_rawDesc = []byte{
	0x0a, 0x0f, 0x71, 0x6d, 0x73, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x12, 0x03, 0x71, 0x6d, 0x73, 0x1a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x22, 0x39, 0x0a, 0x07, 0x51, 0x4d, 0x53, 0x55, 0x73, 0x65, 0x72, 0x12,
	0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75,
	0x75, 0x69, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22,
	0x89, 0x01, 0x0a, 0x0f, 0x51, 0x4d, 0x53, 0x55, 0x73, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65, 0x61,
	0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x05, 0x65,
	0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x76, 0x63,
	0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72, 0x72,
	0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x20, 0x0a, 0x04, 0x75, 0x73, 0x65,
	0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x71, 0x6d, 0x73, 0x2e, 0x51, 0x4d,
	0x53, 0x55, 0x73, 0x65, 0x72, 0x52, 0x04, 0x75, 0x73, 0x65, 0x72, 0x22, 0x87, 0x01, 0x0a, 0x0b,
	0x51, 0x4d, 0x53, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x68,
	0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65,
	0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61,
	0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53, 0x65,
	0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f,
	0x72, 0x12, 0x22, 0x0a, 0x05, 0x75, 0x73, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
	0x32, 0x0c, 0x2e, 0x71, 0x6d, 0x73, 0x2e, 0x51, 0x4d, 0x53, 0x55, 0x73, 0x65, 0x72, 0x52, 0x05,
	0x75, 0x73, 0x65, 0x72, 0x73, 0x22, 0xd0, 0x01, 0x0a, 0x0e, 0x41, 0x64, 0x64, 0x55, 0x73, 0x65,
	0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09,
	0x70, 0x6c, 0x61, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x08, 0x70, 0x6c, 0x61, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x69,
	0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x70, 0x61, 0x69, 0x64, 0x12, 0x18, 0x0a,
	0x07, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07,
	0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x5f, 0x64,
	0x61, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x65, 0x6e, 0x64, 0x44, 0x61,
	0x74, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28,
	0x08, 0x52, 0x05, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x22, 0xd1, 0x01, 0x0a, 0x0f, 0x41, 0x64, 0x64,
	0x55, 0x73, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06,
	0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68,
	0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65,
	0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x76, 0x63, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x53,
	0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72,
	0x6f, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61,
	0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61,
	0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x70, 0x6c, 0x61, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
	0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x6c, 0x61, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12,
	0x1b, 0x0a, 0x09, 0x70, 0x6c, 0x61, 0x6e, 0x5f, 0x75, 0x75, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x08, 0x70, 0x6c, 0x61, 0x6e, 0x55, 0x75, 0x69, 0x64, 0x42, 0x4e, 0x0a, 0x18,
	0x6f, 0x72, 0x67, 0x2e, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x42, 0x10, 0x51, 0x4d, 0x53, 0x55, 0x53, 0x65,
	0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x50, 0x01, 0x5a, 0x1e, 0x67, 0x69,
	0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65,
	0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x71, 0x6d, 0x73, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_qms_users_proto_rawDescOnce sync.Once
	file_qms_users_proto_rawDescData = file_qms_users_proto_rawDesc
)

func file_qms_users_proto_rawDescGZIP() []byte {
	file_qms_users_proto_rawDescOnce.Do(func() {
		file_qms_users_proto_rawDescData = protoimpl.X.CompressGZIP(file_qms_users_proto_rawDescData)
	})
	return file_qms_users_proto_rawDescData
}

var file_qms_users_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_qms_users_proto_goTypes = []any{
	(*QMSUser)(nil),               // 0: qms.QMSUser
	(*QMSUserResponse)(nil),       // 1: qms.QMSUserResponse
	(*QMSUserList)(nil),           // 2: qms.QMSUserList
	(*AddUserRequest)(nil),        // 3: qms.AddUserRequest
	(*AddUserResponse)(nil),       // 4: qms.AddUserResponse
	(*header.Header)(nil),         // 5: header.Header
	(*svcerror.ServiceError)(nil), // 6: svcerror.ServiceError
}
var file_qms_users_proto_depIdxs = []int32{
	5, // 0: qms.QMSUserResponse.header:type_name -> header.Header
	6, // 1: qms.QMSUserResponse.error:type_name -> svcerror.ServiceError
	0, // 2: qms.QMSUserResponse.user:type_name -> qms.QMSUser
	5, // 3: qms.QMSUserList.header:type_name -> header.Header
	6, // 4: qms.QMSUserList.error:type_name -> svcerror.ServiceError
	0, // 5: qms.QMSUserList.users:type_name -> qms.QMSUser
	5, // 6: qms.AddUserRequest.header:type_name -> header.Header
	5, // 7: qms.AddUserResponse.header:type_name -> header.Header
	6, // 8: qms.AddUserResponse.error:type_name -> svcerror.ServiceError
	9, // [9:9] is the sub-list for method output_type
	9, // [9:9] is the sub-list for method input_type
	9, // [9:9] is the sub-list for extension type_name
	9, // [9:9] is the sub-list for extension extendee
	0, // [0:9] is the sub-list for field type_name
}

func init() { file_qms_users_proto_init() }
func file_qms_users_proto_init() {
	if File_qms_users_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_qms_users_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_qms_users_proto_goTypes,
		DependencyIndexes: file_qms_users_proto_depIdxs,
		MessageInfos:      file_qms_users_proto_msgTypes,
	}.Build()
	File_qms_users_proto = out.File
	file_qms_users_proto_rawDesc = nil
	file_qms_users_proto_goTypes = nil
	file_qms_users_proto_depIdxs = nil
}
