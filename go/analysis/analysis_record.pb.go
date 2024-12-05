// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.1
// 	protoc        v5.29.0
// source: analysis_record.proto

package analysis

import (
	apps "github.com/cyverse-de/p/go/apps"
	user "github.com/cyverse-de/p/go/user"
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

type BatchStatus struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Total     int64 `protobuf:"varint,1,opt,name=total,proto3" json:"total,omitempty"`
	Completed int64 `protobuf:"varint,2,opt,name=completed,proto3" json:"completed,omitempty"`
	Running   int64 `protobuf:"varint,3,opt,name=running,proto3" json:"running,omitempty"`
	Submitted int64 `protobuf:"varint,4,opt,name=submitted,proto3" json:"submitted,omitempty"`
}

func (x *BatchStatus) Reset() {
	*x = BatchStatus{}
	mi := &file_analysis_record_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *BatchStatus) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BatchStatus) ProtoMessage() {}

func (x *BatchStatus) ProtoReflect() protoreflect.Message {
	mi := &file_analysis_record_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BatchStatus.ProtoReflect.Descriptor instead.
func (*BatchStatus) Descriptor() ([]byte, []int) {
	return file_analysis_record_proto_rawDescGZIP(), []int{0}
}

func (x *BatchStatus) GetTotal() int64 {
	if x != nil {
		return x.Total
	}
	return 0
}

func (x *BatchStatus) GetCompleted() int64 {
	if x != nil {
		return x.Completed
	}
	return 0
}

func (x *BatchStatus) GetRunning() int64 {
	if x != nil {
		return x.Running
	}
	return 0
}

func (x *BatchStatus) GetSubmitted() int64 {
	if x != nil {
		return x.Submitted
	}
	return 0
}

// *
// Corresponds to the job_types table in the database.
type AnalysisType struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The UUID for the analysis type.
	Id string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	// The name of the analysis type.
	Name string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	// The kind of system the analysis should run on.
	SystemId string `protobuf:"bytes,3,opt,name=system_id,proto3" json:"system_id,omitempty"`
}

func (x *AnalysisType) Reset() {
	*x = AnalysisType{}
	mi := &file_analysis_record_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *AnalysisType) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AnalysisType) ProtoMessage() {}

func (x *AnalysisType) ProtoReflect() protoreflect.Message {
	mi := &file_analysis_record_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AnalysisType.ProtoReflect.Descriptor instead.
func (*AnalysisType) Descriptor() ([]byte, []int) {
	return file_analysis_record_proto_rawDescGZIP(), []int{1}
}

func (x *AnalysisType) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *AnalysisType) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *AnalysisType) GetSystemId() string {
	if x != nil {
		return x.SystemId
	}
	return ""
}

// *
// An analysis in the system.
//
// An analysis is an app that was run by a user.
type Analysis struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The UUID for the analysis
	Id string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	// The description of the analysis provided by the user.
	Description string `protobuf:"bytes,2,opt,name=description,proto3" json:"description,omitempty"`
	// The name of the analysis provided by the user.
	Name string `protobuf:"bytes,3,opt,name=name,proto3" json:"name,omitempty"`
	// App information about the analysis.
	App *apps.App `protobuf:"bytes,4,opt,name=app,proto3" json:"app,omitempty"`
	// App version information for the analysis.
	AppVersion *apps.AppVersion `protobuf:"bytes,5,opt,name=app_version,proto3" json:"app_version,omitempty"`
	// The analysis type, which tells which environment to run the analysis in.
	Kind *AnalysisType `protobuf:"bytes,6,opt,name=kind,json=type,proto3" json:"kind,omitempty"`
	// The path to the folder containing analysis outputs.
	ResultFolderPath string `protobuf:"bytes,7,opt,name=result_folder_path,proto3" json:"result_folder_path,omitempty"`
	// The date the analysis was submitted.
	StartDate *timestamppb.Timestamp `protobuf:"bytes,8,opt,name=start_date,proto3" json:"start_date,omitempty"`
	// The date the analyses finished.
	EndDate *timestamppb.Timestamp `protobuf:"bytes,9,opt,name=end_date,proto3" json:"end_date,omitempty"`
	// The date the analysis was scheduled to end.
	PlannedEndDate *timestamppb.Timestamp `protobuf:"bytes,10,opt,name=planned_end_date,proto3" json:"planned_end_date,omitempty"`
	// The status of the analysis
	Status string `protobuf:"bytes,11,opt,name=status,proto3" json:"status,omitempty"`
	// Whether the analysis was deleted.
	Deleted bool `protobuf:"varint,12,opt,name=deleted,proto3" json:"deleted,omitempty"`
	// Whether notifications should be emitted on status changes.
	Notify bool `protobuf:"varint,13,opt,name=notify,proto3" json:"notify,omitempty"`
	// The user that submitted the analysis.
	User *user.User `protobuf:"bytes,14,opt,name=user,proto3" json:"user,omitempty"`
	// The subdomain assigned to the analysis for VICE.
	Subdomain string `protobuf:"bytes,15,opt,name=subdomain,proto3" json:"subdomain,omitempty"`
	// The UUID of the analysis that spawned this analysis. Used for batch analyses.
	ParentId string `protobuf:"bytes,16,opt,name=parent_id,proto3" json:"parent_id,omitempty"`
	// The number of millicores reserved for the analysis.
	MillicoresReserved float64 `protobuf:"fixed64,17,opt,name=millicores_reserved,json=millicore_reserved,proto3" json:"millicores_reserved,omitempty"`
}

func (x *Analysis) Reset() {
	*x = Analysis{}
	mi := &file_analysis_record_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Analysis) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Analysis) ProtoMessage() {}

func (x *Analysis) ProtoReflect() protoreflect.Message {
	mi := &file_analysis_record_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Analysis.ProtoReflect.Descriptor instead.
func (*Analysis) Descriptor() ([]byte, []int) {
	return file_analysis_record_proto_rawDescGZIP(), []int{2}
}

func (x *Analysis) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Analysis) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Analysis) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Analysis) GetApp() *apps.App {
	if x != nil {
		return x.App
	}
	return nil
}

func (x *Analysis) GetAppVersion() *apps.AppVersion {
	if x != nil {
		return x.AppVersion
	}
	return nil
}

func (x *Analysis) GetKind() *AnalysisType {
	if x != nil {
		return x.Kind
	}
	return nil
}

func (x *Analysis) GetResultFolderPath() string {
	if x != nil {
		return x.ResultFolderPath
	}
	return ""
}

func (x *Analysis) GetStartDate() *timestamppb.Timestamp {
	if x != nil {
		return x.StartDate
	}
	return nil
}

func (x *Analysis) GetEndDate() *timestamppb.Timestamp {
	if x != nil {
		return x.EndDate
	}
	return nil
}

func (x *Analysis) GetPlannedEndDate() *timestamppb.Timestamp {
	if x != nil {
		return x.PlannedEndDate
	}
	return nil
}

func (x *Analysis) GetStatus() string {
	if x != nil {
		return x.Status
	}
	return ""
}

func (x *Analysis) GetDeleted() bool {
	if x != nil {
		return x.Deleted
	}
	return false
}

func (x *Analysis) GetNotify() bool {
	if x != nil {
		return x.Notify
	}
	return false
}

func (x *Analysis) GetUser() *user.User {
	if x != nil {
		return x.User
	}
	return nil
}

func (x *Analysis) GetSubdomain() string {
	if x != nil {
		return x.Subdomain
	}
	return ""
}

func (x *Analysis) GetParentId() string {
	if x != nil {
		return x.ParentId
	}
	return ""
}

func (x *Analysis) GetMillicoresReserved() float64 {
	if x != nil {
		return x.MillicoresReserved
	}
	return 0
}

var File_analysis_record_proto protoreflect.FileDescriptor

var file_analysis_record_proto_rawDesc = []byte{
	0x0a, 0x15, 0x61, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x6f, 0x72,
	0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x61, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69,
	0x73, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
	0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x1a, 0x0a, 0x75, 0x73, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a,
	0x61, 0x70, 0x70, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x79, 0x0a, 0x0b, 0x42, 0x61,
	0x74, 0x63, 0x68, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x74,
	0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x12,
	0x1c, 0x0a, 0x09, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x03, 0x52, 0x09, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x12, 0x18, 0x0a,
	0x07, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x07,
	0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x75, 0x62, 0x6d, 0x69,
	0x74, 0x74, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x73, 0x75, 0x62, 0x6d,
	0x69, 0x74, 0x74, 0x65, 0x64, 0x22, 0x50, 0x0a, 0x0c, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69,
	0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x79, 0x73,
	0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x79,
	0x73, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x22, 0x90, 0x05, 0x0a, 0x08, 0x41, 0x6e, 0x61, 0x6c,
	0x79, 0x73, 0x69, 0x73, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x02, 0x69, 0x64, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
	0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72,
	0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x03, 0x61, 0x70,
	0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x61, 0x70, 0x70, 0x73, 0x2e, 0x41,
	0x70, 0x70, 0x52, 0x03, 0x61, 0x70, 0x70, 0x12, 0x32, 0x0a, 0x0b, 0x61, 0x70, 0x70, 0x5f, 0x76,
	0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x61,
	0x70, 0x70, 0x73, 0x2e, 0x41, 0x70, 0x70, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x0b,
	0x61, 0x70, 0x70, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x2a, 0x0a, 0x04, 0x6b,
	0x69, 0x6e, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x61, 0x6e, 0x61, 0x6c,
	0x79, 0x73, 0x69, 0x73, 0x2e, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69, 0x73, 0x54, 0x79, 0x70,
	0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x2e, 0x0a, 0x12, 0x72, 0x65, 0x73, 0x75, 0x6c,
	0x74, 0x5f, 0x66, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x07, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x12, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x66, 0x6f, 0x6c, 0x64,
	0x65, 0x72, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x12, 0x3a, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74,
	0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f,
	0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69,
	0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x64,
	0x61, 0x74, 0x65, 0x12, 0x36, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18,
	0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
	0x70, 0x52, 0x08, 0x65, 0x6e, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x12, 0x46, 0x0a, 0x10, 0x70,
	0x6c, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18,
	0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
	0x70, 0x52, 0x10, 0x70, 0x6c, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x64,
	0x61, 0x74, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x0b, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x64,
	0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x64, 0x65,
	0x6c, 0x65, 0x74, 0x65, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x18,
	0x0d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x12, 0x1e, 0x0a,
	0x04, 0x75, 0x73, 0x65, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x75, 0x73,
	0x65, 0x72, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x52, 0x04, 0x75, 0x73, 0x65, 0x72, 0x12, 0x1c, 0x0a,
	0x09, 0x73, 0x75, 0x62, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x09, 0x73, 0x75, 0x62, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x12, 0x1c, 0x0a, 0x09, 0x70,
	0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
	0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x12, 0x2f, 0x0a, 0x13, 0x6d, 0x69, 0x6c,
	0x6c, 0x69, 0x63, 0x6f, 0x72, 0x65, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64,
	0x18, 0x11, 0x20, 0x01, 0x28, 0x01, 0x52, 0x12, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x63, 0x6f, 0x72,
	0x65, 0x5f, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x42, 0x5a, 0x0a, 0x18, 0x6f, 0x72,
	0x67, 0x2e, 0x63, 0x79, 0x76, 0x65, 0x72, 0x73, 0x65, 0x2e, 0x64, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x42, 0x17, 0x41, 0x6e, 0x61, 0x6c, 0x79, 0x73, 0x69, 0x73,
	0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x50,
	0x01, 0x5a, 0x23, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x79,
	0x76, 0x65, 0x72, 0x73, 0x65, 0x2d, 0x64, 0x65, 0x2f, 0x70, 0x2f, 0x67, 0x6f, 0x2f, 0x61, 0x6e,
	0x61, 0x6c, 0x79, 0x73, 0x69, 0x73, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_analysis_record_proto_rawDescOnce sync.Once
	file_analysis_record_proto_rawDescData = file_analysis_record_proto_rawDesc
)

func file_analysis_record_proto_rawDescGZIP() []byte {
	file_analysis_record_proto_rawDescOnce.Do(func() {
		file_analysis_record_proto_rawDescData = protoimpl.X.CompressGZIP(file_analysis_record_proto_rawDescData)
	})
	return file_analysis_record_proto_rawDescData
}

var file_analysis_record_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_analysis_record_proto_goTypes = []any{
	(*BatchStatus)(nil),           // 0: analysis.BatchStatus
	(*AnalysisType)(nil),          // 1: analysis.AnalysisType
	(*Analysis)(nil),              // 2: analysis.Analysis
	(*apps.App)(nil),              // 3: apps.App
	(*apps.AppVersion)(nil),       // 4: apps.AppVersion
	(*timestamppb.Timestamp)(nil), // 5: google.protobuf.Timestamp
	(*user.User)(nil),             // 6: user.User
}
var file_analysis_record_proto_depIdxs = []int32{
	3, // 0: analysis.Analysis.app:type_name -> apps.App
	4, // 1: analysis.Analysis.app_version:type_name -> apps.AppVersion
	1, // 2: analysis.Analysis.kind:type_name -> analysis.AnalysisType
	5, // 3: analysis.Analysis.start_date:type_name -> google.protobuf.Timestamp
	5, // 4: analysis.Analysis.end_date:type_name -> google.protobuf.Timestamp
	5, // 5: analysis.Analysis.planned_end_date:type_name -> google.protobuf.Timestamp
	6, // 6: analysis.Analysis.user:type_name -> user.User
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_analysis_record_proto_init() }
func file_analysis_record_proto_init() {
	if File_analysis_record_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_analysis_record_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_analysis_record_proto_goTypes,
		DependencyIndexes: file_analysis_record_proto_depIdxs,
		MessageInfos:      file_analysis_record_proto_msgTypes,
	}.Build()
	File_analysis_record_proto = out.File
	file_analysis_record_proto_rawDesc = nil
	file_analysis_record_proto_goTypes = nil
	file_analysis_record_proto_depIdxs = nil
}
