// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: requests.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public final class Requests {
  private Requests() {}
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 29,
      /* patch= */ 0,
      /* suffix= */ "",
      Requests.class.getName());
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
    internal_static_requests_ByUsername_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_requests_ByUsername_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_requests_ByUserID_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_requests_ByUserID_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_requests_NoParams_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_requests_NoParams_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_requests_ByUUID_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_requests_ByUUID_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_requests_ByUUIDAndUsername_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_requests_ByUUIDAndUsername_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_requests_ByUUIDAndUserID_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_requests_ByUUIDAndUserID_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_requests_AssociateByUUIDs_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_requests_AssociateByUUIDs_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\016requests.proto\022\010requests\032\014header.proto" +
      "\">\n\nByUsername\022\036\n\006header\030\001 \001(\0132\016.header." +
      "Header\022\020\n\010username\030\002 \001(\t\"D\n\010ByUserID\022\036\n\006" +
      "header\030\001 \001(\0132\016.header.Header\022\030\n\007user_id\030" +
      "\002 \001(\tR\007user_id\"*\n\010NoParams\022\036\n\006header\030\001 \001" +
      "(\0132\016.header.Header\"6\n\006ByUUID\022\036\n\006header\030\001" +
      " \001(\0132\016.header.Header\022\014\n\004uuid\030\002 \001(\t\"S\n\021By" +
      "UUIDAndUsername\022\036\n\006header\030\001 \001(\0132\016.header" +
      ".Header\022\014\n\004uuid\030\002 \001(\t\022\020\n\010username\030\003 \001(\t\"" +
      "Y\n\017ByUUIDAndUserID\022\036\n\006header\030\001 \001(\0132\016.hea" +
      "der.Header\022\014\n\004uuid\030\002 \001(\t\022\030\n\007user_id\030\003 \001(" +
      "\tR\007user_id\"t\n\020AssociateByUUIDs\022\036\n\006header" +
      "\030\001 \001(\0132\016.header.Header\022 \n\013parent_uuid\030\002 " +
      "\001(\tR\013parent_uuid\022\036\n\nchild_uuid\030\003 \001(\tR\nch" +
      "ild_uuidBK\n\030org.cyverse.de.protobufsB\010Re" +
      "questsP\001Z#github.com/cyverse-de/p/go/req" +
      "uestsb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
        });
    internal_static_requests_ByUsername_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_requests_ByUsername_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_requests_ByUsername_descriptor,
        new java.lang.String[] { "Header", "Username", });
    internal_static_requests_ByUserID_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_requests_ByUserID_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_requests_ByUserID_descriptor,
        new java.lang.String[] { "Header", "UserId", });
    internal_static_requests_NoParams_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_requests_NoParams_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_requests_NoParams_descriptor,
        new java.lang.String[] { "Header", });
    internal_static_requests_ByUUID_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_requests_ByUUID_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_requests_ByUUID_descriptor,
        new java.lang.String[] { "Header", "Uuid", });
    internal_static_requests_ByUUIDAndUsername_descriptor =
      getDescriptor().getMessageTypes().get(4);
    internal_static_requests_ByUUIDAndUsername_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_requests_ByUUIDAndUsername_descriptor,
        new java.lang.String[] { "Header", "Uuid", "Username", });
    internal_static_requests_ByUUIDAndUserID_descriptor =
      getDescriptor().getMessageTypes().get(5);
    internal_static_requests_ByUUIDAndUserID_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_requests_ByUUIDAndUserID_descriptor,
        new java.lang.String[] { "Header", "Uuid", "UserId", });
    internal_static_requests_AssociateByUUIDs_descriptor =
      getDescriptor().getMessageTypes().get(6);
    internal_static_requests_AssociateByUUIDs_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_requests_AssociateByUUIDs_descriptor,
        new java.lang.String[] { "Header", "ParentUuid", "ChildUuid", });
    descriptor.resolveAllFeaturesImmutable();
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
