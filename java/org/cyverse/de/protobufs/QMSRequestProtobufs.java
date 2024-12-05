// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_requests.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public final class QMSRequestProtobufs {
  private QMSRequestProtobufs() {}
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 29,
      /* patch= */ 0,
      /* suffix= */ "",
      QMSRequestProtobufs.class.getName());
  }
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_AllUserOveragesRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_AllUserOveragesRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_UserResourceOveragesRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_UserResourceOveragesRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_IsOverageRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_IsOverageRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_AddUsage_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_AddUsage_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_GetUsages_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_GetUsages_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_RequestByUsername_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_RequestByUsername_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_RequestByUserID_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_RequestByUserID_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_NoParamsRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_NoParamsRequest_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\022qms_requests.proto\022\003qms\032\014header.proto\"" +
      "J\n\026AllUserOveragesRequest\022\036\n\006header\030\001 \001(" +
      "\0132\016.header.Header\022\020\n\010username\030\002 \001(\t\"u\n\033U" +
      "serResourceOveragesRequest\022\036\n\006header\030\001 \001" +
      "(\0132\016.header.Header\022\020\n\010username\030\002 \001(\t\022$\n\r" +
      "resource_name\030\003 \001(\tR\rresource_name\"j\n\020Is" +
      "OverageRequest\022\036\n\006header\030\001 \001(\0132\016.header." +
      "Header\022\020\n\010username\030\002 \001(\t\022$\n\rresource_nam" +
      "e\030\003 \001(\tR\rresource_name\"\314\001\n\010AddUsage\022\036\n\006h" +
      "eader\030\001 \001(\0132\016.header.Header\022\020\n\010username\030" +
      "\002 \001(\t\022$\n\rresource_name\030\003 \001(\tR\rresource_n" +
      "ame\022 \n\013update_type\030\004 \001(\tR\013update_type\022 \n" +
      "\013usage_value\030\005 \001(\001R\013usage_value\022$\n\rresou" +
      "rce_unit\030\006 \001(\tR\rresource_unit\"=\n\tGetUsag" +
      "es\022\036\n\006header\030\001 \001(\0132\016.header.Header\022\020\n\010us" +
      "ername\030\002 \001(\t\"E\n\021RequestByUsername\022\036\n\006hea" +
      "der\030\001 \001(\0132\016.header.Header\022\020\n\010username\030\002 " +
      "\001(\t\"K\n\017RequestByUserID\022\036\n\006header\030\001 \001(\0132\016" +
      ".header.Header\022\030\n\007user_id\030\002 \001(\tR\007user_id" +
      "\"1\n\017NoParamsRequest\022\036\n\006header\030\001 \001(\0132\016.he" +
      "ader.HeaderBQ\n\030org.cyverse.de.protobufsB" +
      "\023QMSRequestProtobufsP\001Z\036github.com/cyver" +
      "se-de/p/go/qmsb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
        });
    internal_static_qms_AllUserOveragesRequest_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_qms_AllUserOveragesRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_AllUserOveragesRequest_descriptor,
        new java.lang.String[] { "Header", "Username", });
    internal_static_qms_UserResourceOveragesRequest_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_qms_UserResourceOveragesRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_UserResourceOveragesRequest_descriptor,
        new java.lang.String[] { "Header", "Username", "ResourceName", });
    internal_static_qms_IsOverageRequest_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_qms_IsOverageRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_IsOverageRequest_descriptor,
        new java.lang.String[] { "Header", "Username", "ResourceName", });
    internal_static_qms_AddUsage_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_qms_AddUsage_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_AddUsage_descriptor,
        new java.lang.String[] { "Header", "Username", "ResourceName", "UpdateType", "UsageValue", "ResourceUnit", });
    internal_static_qms_GetUsages_descriptor =
      getDescriptor().getMessageTypes().get(4);
    internal_static_qms_GetUsages_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_GetUsages_descriptor,
        new java.lang.String[] { "Header", "Username", });
    internal_static_qms_RequestByUsername_descriptor =
      getDescriptor().getMessageTypes().get(5);
    internal_static_qms_RequestByUsername_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_RequestByUsername_descriptor,
        new java.lang.String[] { "Header", "Username", });
    internal_static_qms_RequestByUserID_descriptor =
      getDescriptor().getMessageTypes().get(6);
    internal_static_qms_RequestByUserID_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_RequestByUserID_descriptor,
        new java.lang.String[] { "Header", "UserId", });
    internal_static_qms_NoParamsRequest_descriptor =
      getDescriptor().getMessageTypes().get(7);
    internal_static_qms_NoParamsRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_NoParamsRequest_descriptor,
        new java.lang.String[] { "Header", });
    descriptor.resolveAllFeaturesImmutable();
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
