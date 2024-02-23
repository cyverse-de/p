// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_resource_types.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public final class QMSResourceTypeProtobufs {
  private QMSResourceTypeProtobufs() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_ResourceType_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_qms_ResourceType_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_ResourceTypeResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_qms_ResourceTypeResponse_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_ResourceTypeList_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_qms_ResourceTypeList_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\030qms_resource_types.proto\022\003qms\032\014header." +
      "proto\032\016svcerror.proto\"L\n\014ResourceType\022\014\n" +
      "\004uuid\030\001 \001(\t\022\014\n\004name\030\002 \001(\t\022\014\n\004unit\030\003 \001(\t\022" +
      "\022\n\nconsumable\030\004 \001(\010\"\226\001\n\024ResourceTypeResp" +
      "onse\022\036\n\006header\030\001 \001(\0132\016.header.Header\022%\n\005" +
      "error\030\002 \001(\0132\026.svcerror.ServiceError\0227\n\rr" +
      "esource_type\030\003 \001(\0132\021.qms.ResourceTypeR\rr" +
      "esource_type\"\224\001\n\020ResourceTypeList\022\036\n\006hea" +
      "der\030\001 \001(\0132\016.header.Header\022%\n\005error\030\002 \001(\013" +
      "2\026.svcerror.ServiceError\0229\n\016resource_typ" +
      "es\030\003 \003(\0132\021.qms.ResourceTypeR\016resource_ty" +
      "pesBV\n\030org.cyverse.de.protobufsB\030QMSReso" +
      "urceTypeProtobufsP\001Z\036github.com/cyverse-" +
      "de/p/go/qmsb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
          org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor(),
        });
    internal_static_qms_ResourceType_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_qms_ResourceType_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_qms_ResourceType_descriptor,
        new java.lang.String[] { "Uuid", "Name", "Unit", "Consumable", });
    internal_static_qms_ResourceTypeResponse_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_qms_ResourceTypeResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_qms_ResourceTypeResponse_descriptor,
        new java.lang.String[] { "Header", "Error", "ResourceType", });
    internal_static_qms_ResourceTypeList_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_qms_ResourceTypeList_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_qms_ResourceTypeList_descriptor,
        new java.lang.String[] { "Header", "Error", "ResourceTypes", });
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
    org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
