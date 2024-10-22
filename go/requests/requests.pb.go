// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        v3.19.6
// source: requests.proto

package requests

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

// *
// Request a resource by the username of a user.
type ByUsername struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header   *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	Username string         `protobuf:"bytes,2,opt,name=username,proto3" json:"username,omitempty"`
}

func (x *ByUsername) Reset() {
	*x = ByUsername{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ByUsername) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ByUsername) ProtoMessage() {}

func (x *ByUsername) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ByUsername.ProtoReflect.Descriptor instead.
func (*ByUsername) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{0}
}

func (x *ByUsername) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUsername) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

// *
// Request a resource by the user ID of a user.
type ByUserID struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	UserId string         `protobuf:"bytes,2,opt,name=user_id,proto3" json:"user_id,omitempty"`
}

func (x *ByUserID) Reset() {
	*x = ByUserID{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ByUserID) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ByUserID) ProtoMessage() {}

func (x *ByUserID) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ByUserID.ProtoReflect.Descriptor instead.
func (*ByUserID) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{1}
}

func (x *ByUserID) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUserID) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

// *
// Send a message that does not request any parameters. Common for triggering
// side-effects or for retrieving lists of resources as an administrator.
type NoParams struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
}

func (x *NoParams) Reset() {
	*x = NoParams{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *NoParams) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*NoParams) ProtoMessage() {}

func (x *NoParams) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use NoParams.ProtoReflect.Descriptor instead.
func (*NoParams) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{2}
}

func (x *NoParams) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

// *
// Request a resource by its UUID.
type ByUUID struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// The UUID of the resource being requested.
	Uuid string `protobuf:"bytes,2,opt,name=uuid,proto3" json:"uuid,omitempty"`
}

func (x *ByUUID) Reset() {
	*x = ByUUID{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ByUUID) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ByUUID) ProtoMessage() {}

func (x *ByUUID) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ByUUID.ProtoReflect.Descriptor instead.
func (*ByUUID) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{3}
}

func (x *ByUUID) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUUID) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

// *
// Request a resource by its UUID and a username. Useful in situations where a
// user's ability to access a resource needs to be checked as part of the
// request handler logic.
type ByUUIDAndUsername struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// The UUID of the resource being requested
	Uuid string `protobuf:"bytes,2,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// The username associated with the request.
	Username string `protobuf:"bytes,3,opt,name=username,proto3" json:"username,omitempty"`
}

func (x *ByUUIDAndUsername) Reset() {
	*x = ByUUIDAndUsername{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ByUUIDAndUsername) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ByUUIDAndUsername) ProtoMessage() {}

func (x *ByUUIDAndUsername) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ByUUIDAndUsername.ProtoReflect.Descriptor instead.
func (*ByUUIDAndUsername) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{4}
}

func (x *ByUUIDAndUsername) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUUIDAndUsername) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *ByUUIDAndUsername) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

// *
// Request a resource by its UUID and a user's UUID. Useful when the user's
// access to the resource must be verified.
type ByUUIDAndUserID struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// The UUID of the resource being requested
	Uuid string `protobuf:"bytes,2,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// The user ID of the user associated with the request.
	UserId string `protobuf:"bytes,3,opt,name=user_id,proto3" json:"user_id,omitempty"`
}

func (x *ByUUIDAndUserID) Reset() {
	*x = ByUUIDAndUserID{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ByUUIDAndUserID) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ByUUIDAndUserID) ProtoMessage() {}

func (x *ByUUIDAndUserID) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ByUUIDAndUserID.ProtoReflect.Descriptor instead.
func (*ByUUIDAndUserID) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{5}
}

func (x *ByUUIDAndUserID) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *ByUUIDAndUserID) GetUuid() string {
	if x != nil {
		return x.Uuid
	}
	return ""
}

func (x *ByUUIDAndUserID) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

// *
// Request that two resources be associated.
type AssociateByUUIDs struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Contains telemetry information.
	Header *header.Header `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
	// The UUID of the parent/owner/primary resource.
	ParentUuid string `protobuf:"bytes,2,opt,name=parent_uuid,proto3" json:"parent_uuid,omitempty"`
	// The UUID of the child/object/secondary resource.
	ChildUuid string `protobuf:"bytes,3,opt,name=child_uuid,proto3" json:"child_uuid,omitempty"`
}

func (x *AssociateByUUIDs) Reset() {
	*x = AssociateByUUIDs{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AssociateByUUIDs) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AssociateByUUIDs) ProtoMessage() {}

func (x *AssociateByUUIDs) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AssociateByUUIDs.ProtoReflect.Descriptor instead.
func (*AssociateByUUIDs) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{6}
}

func (x *AssociateByUUIDs) GetHeader() *header.Header {
	if x != nil {
		return x.Header
	}
	return nil
}

func (x *AssociateByUUIDs) GetParentUuid() string {
	if x != nil {
		return x.ParentUuid
	}
	return ""
}

func (x *AssociateByUUIDs) GetChildUuid() string {
	if x != nil {
		return x.ChildUuid
	}
	return ""
}

var File_requests_proto protoreflect.FileDescriptor

var file_requests_proto_rawDesc = []byte{
	0x0a, 0x0e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x12, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x1a, 0x0c, 0x68, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x50, 0x0a, 0x0a, 0x42, 0x79, 0x55, 0x73,
	0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e,
	0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x1a,
	0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x4c, 0x0a, 0x08, 0x42, 0x79,
	0x55, 0x73, 0x65, 0x72, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e,
	0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x18,
	0x0a, 0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x22, 0x32, 0x0a, 0x08, 0x4e, 0x6f, 0x50, 0x61,
	0x72, 0x61, 0x6d, 0x73, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65,
	0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x44, 0x0a, 0x06,
	0x42, 0x79, 0x55, 0x55, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e,
	0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x12,
	0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x75,
	0x69, 0x64, 0x22, 0x6b, 0x0a, 0x11, 0x42, 0x79, 0x55, 0x55, 0x49, 0x44, 0x41, 0x6e, 0x64, 0x55,
	0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12,
	0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75,
	0x75, 0x69, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22,
	0x67, 0x0a, 0x0f, 0x42, 0x79, 0x55, 0x55, 0x49, 0x44, 0x41, 0x6e, 0x64, 0x55, 0x73, 0x65, 0x72,
	0x49, 0x44, 0x12, 0x26, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75,
	0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x18,
	0x0a, 0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x22, 0x7c, 0x0a, 0x10, 0x41, 0x73, 0x73, 0x6f,
	0x63, 0x69, 0x61, 0x74, 0x65, 0x42, 0x79, 0x55, 0x55, 0x49, 0x44, 0x73, 0x12, 0x26, 0x0a, 0x06,
	0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x68,
	0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65,
	0x61, 0x64, 0x65, 0x72, 0x12, 0x20, 0x0a, 0x0b, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x75,
	0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70, 0x61, 0x72, 0x65, 0x6e,
	0x74, 0x5f, 0x75, 0x75, 0x69, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x68, 0x69, 0x6c, 0x64, 0x5f,
	0x75, 0x75, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x68, 0x69, 0x6c,
	0x64, 0x5f, 0x75, 0x75, 0x69, 0x64, 0x42, 0x4b, 0x0a, 0x18, 0x6f, 0x72, 0x67, 0x2e, 0x63, 0x79,
	0x76, 0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
	0x66, 0x73, 0x42, 0x08, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x50, 0x01, 0x5a, 0x23,
	0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79, 0x76, 0x65, 0x72,
	0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x72, 0x65, 0x71, 0x75, 0x65,
	0x73, 0x74, 0x73, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_requests_proto_rawDescOnce sync.Once
	file_requests_proto_rawDescData = file_requests_proto_rawDesc
)

func file_requests_proto_rawDescGZIP() []byte {
	file_requests_proto_rawDescOnce.Do(func() {
		file_requests_proto_rawDescData = protoimpl.X.CompressGZIP(file_requests_proto_rawDescData)
	})
	return file_requests_proto_rawDescData
}

var file_requests_proto_msgTypes = make([]protoimpl.MessageInfo, 7)
var file_requests_proto_goTypes = []interface{}{
	(*ByUsername)(nil),        // 0: requests.ByUsername
	(*ByUserID)(nil),          // 1: requests.ByUserID
	(*NoParams)(nil),          // 2: requests.NoParams
	(*ByUUID)(nil),            // 3: requests.ByUUID
	(*ByUUIDAndUsername)(nil), // 4: requests.ByUUIDAndUsername
	(*ByUUIDAndUserID)(nil),   // 5: requests.ByUUIDAndUserID
	(*AssociateByUUIDs)(nil),  // 6: requests.AssociateByUUIDs
	(*header.Header)(nil),     // 7: header.Header
}
var file_requests_proto_depIdxs = []int32{
	7, // 0: requests.ByUsername.header:type_name -> header.Header
	7, // 1: requests.ByUserID.header:type_name -> header.Header
	7, // 2: requests.NoParams.header:type_name -> header.Header
	7, // 3: requests.ByUUID.header:type_name -> header.Header
	7, // 4: requests.ByUUIDAndUsername.header:type_name -> header.Header
	7, // 5: requests.ByUUIDAndUserID.header:type_name -> header.Header
	7, // 6: requests.AssociateByUUIDs.header:type_name -> header.Header
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_requests_proto_init() }
func file_requests_proto_init() {
	if File_requests_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_requests_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ByUsername); i {
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
		file_requests_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ByUserID); i {
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
		file_requests_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*NoParams); i {
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
		file_requests_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ByUUID); i {
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
		file_requests_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ByUUIDAndUsername); i {
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
		file_requests_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ByUUIDAndUserID); i {
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
		file_requests_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AssociateByUUIDs); i {
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
			RawDescriptor: file_requests_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   7,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_requests_proto_goTypes,
		DependencyIndexes: file_requests_proto_depIdxs,
		MessageInfos:      file_requests_proto_msgTypes,
	}.Build()
	File_requests_proto = out.File
	file_requests_proto_rawDesc = nil
	file_requests_proto_goTypes = nil
	file_requests_proto_depIdxs = nil
}
