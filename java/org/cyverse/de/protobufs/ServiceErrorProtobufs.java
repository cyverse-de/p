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
    internal_static_svcerror_ServiceError_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_svcerror_ServiceError_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\016svcerror.proto\022\010svcerror\"|\n\014ServiceErr" +
      "or\0223\n\nerror_code\030\002 \001(\0162\023.svcerror.ErrorC" +
      "odeR\nerror_code\022 \n\013status_code\030\003 \001(\005R\013st" +
      "atus_code\022\017\n\007message\030\004 \001(\tJ\004\010\001\020\002*\204\002\n\tErr" +
      "orCode\022\t\n\005UNSET\020\000\022\017\n\013UNSPECIFIED\020\001\022\014\n\010IN" +
      "TERNAL\020\002\022\r\n\tNOT_FOUND\020\003\022\017\n\013BAD_REQUEST\020\004" +
      "\022\023\n\017MARSHAL_FAILURE\020\005\022\025\n\021UNMARSHAL_FAILU" +
      "RE\020\006\022\025\n\021PARAMETER_MISSING\020\007\022\025\n\021PARAMETER" +
      "_INVALID\020\010\022\023\n\017UNAUTHENTICATED\020\t\022\r\n\tFORBI" +
      "DDEN\020\n\022\013\n\007TIMEOUT\020\013\022\017\n\013UNSUPPORTED\020\014\022\021\n\r" +
      "UNIMPLEMENTED\020\rBX\n\030org.cyverse.de.protob" +
      "ufsB\025ServiceErrorProtobufsP\001Z#github.com" +
      "/cyverse-de/p/go/svcerrorb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
        });
    internal_static_svcerror_ServiceError_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_svcerror_ServiceError_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_svcerror_ServiceError_descriptor,
        new java.lang.String[] { "ErrorCode", "StatusCode", "Message", });
  }

  // @@protoc_insertion_point(outer_class_scope)
}
