// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: svcerror.proto

package org.cyverse.de.protobufs;

public final class ServiceErrorProtobufs {
  private ServiceErrorProtobufs() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_ServiceError_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_ServiceError_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\016svcerror.proto\032\014header.proto\"\206\001\n\014Servi" +
      "ceError\022\027\n\006header\030\001 \001(\0132\007.Header\022*\n\nerro" +
      "r_code\030\002 \001(\0162\n.ErrorCodeR\nerror_code\022 \n\013" +
      "status_code\030\003 \001(\005R\013status_code\022\017\n\007messag" +
      "e\030\004 \001(\t*\257\001\n\tErrorCode\022\t\n\005UNSET\020\000\022\017\n\013UNSP" +
      "ECIFIED\020\001\022\014\n\010INTERNAL\020\002\022\r\n\tNOT_FOUND\020\003\022\017" +
      "\n\013BAD_REQUEST\020\004\022\023\n\017MARSHAL_FAILURE\020\005\022\025\n\021" +
      "UNMARSHAL_FAILURE\020\006\022\025\n\021PARAMETER_MISSING" +
      "\020\007\022\025\n\021PARAMETER_INVALID\020\010BX\n\030org.cyverse" +
      ".de.protobufsB\025ServiceErrorProtobufsP\001Z#" +
      "github.com/cyverse-de/p/go/svcerrorb\006pro" +
      "to3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
        });
    internal_static_ServiceError_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_ServiceError_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_ServiceError_descriptor,
        new java.lang.String[] { "Header", "ErrorCode", "StatusCode", "Message", });
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
