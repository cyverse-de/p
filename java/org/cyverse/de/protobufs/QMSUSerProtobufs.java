// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_users.proto

package org.cyverse.de.protobufs;

public final class QMSUSerProtobufs {
  private QMSUSerProtobufs() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_QMSUser_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_QMSUser_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_QMSUserResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_QMSUserResponse_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_QMSUserList_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_QMSUserList_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_AddUserRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_AddUserRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_AddUserResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_AddUserResponse_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\017qms_users.proto\032\014header.proto\032\016svcerro" +
      "r.proto\")\n\007QMSUser\022\014\n\004uuid\030\001 \001(\t\022\020\n\010user" +
      "name\030\002 \001(\t\"`\n\017QMSUserResponse\022\027\n\006header\030" +
      "\001 \001(\0132\007.Header\022\034\n\005error\030\002 \001(\0132\r.ServiceE" +
      "rror\022\026\n\004user\030\003 \001(\0132\010.QMSUser\"]\n\013QMSUserL" +
      "ist\022\027\n\006header\030\001 \001(\0132\007.Header\022\034\n\005error\030\002 " +
      "\001(\0132\r.ServiceError\022\027\n\005users\030\003 \003(\0132\010.QMSU" +
      "ser\"N\n\016AddUserRequest\022\027\n\006header\030\001 \001(\0132\007." +
      "Header\022\020\n\010username\030\003 \001(\t\022\021\n\tplan_name\030\004 " +
      "\001(\t\"\216\001\n\017AddUserResponse\022\027\n\006header\030\001 \001(\0132" +
      "\007.Header\022\034\n\005error\030\002 \001(\0132\r.ServiceError\022\014" +
      "\n\004uuid\030\003 \001(\t\022\020\n\010username\030\004 \001(\t\022\021\n\tplan_n" +
      "ame\030\005 \001(\t\022\021\n\tplan_uuid\030\006 \001(\tBN\n\030org.cyve" +
      "rse.de.protobufsB\020QMSUSerProtobufsP\001Z\036gi" +
      "thub.com/cyverse-de/p/go/qmsb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
          org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor(),
        });
    internal_static_QMSUser_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_QMSUser_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_QMSUser_descriptor,
        new java.lang.String[] { "Uuid", "Username", });
    internal_static_QMSUserResponse_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_QMSUserResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_QMSUserResponse_descriptor,
        new java.lang.String[] { "Header", "Error", "User", });
    internal_static_QMSUserList_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_QMSUserList_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_QMSUserList_descriptor,
        new java.lang.String[] { "Header", "Error", "Users", });
    internal_static_AddUserRequest_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_AddUserRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_AddUserRequest_descriptor,
        new java.lang.String[] { "Header", "Username", "PlanName", });
    internal_static_AddUserResponse_descriptor =
      getDescriptor().getMessageTypes().get(4);
    internal_static_AddUserResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_AddUserResponse_descriptor,
        new java.lang.String[] { "Header", "Error", "Uuid", "Username", "PlanName", "PlanUuid", });
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
    org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
