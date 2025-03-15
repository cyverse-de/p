// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_overages.proto
// Protobuf Java Version: 4.29.3

package org.cyverse.de.protobufs;

public final class QMSOveragesProtobufs {
  private QMSOveragesProtobufs() {}
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 29,
      /* patch= */ 3,
      /* suffix= */ "",
      QMSOveragesProtobufs.class.getName());
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
    internal_static_qms_Overage_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_Overage_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_OverageResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_OverageResponse_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_OverageList_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_OverageList_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_IsOverage_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_IsOverage_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\022qms_overages.proto\022\003qms\032\014header.proto\032" +
      "\016svcerror.proto\"M\n\007Overage\022$\n\rresource_n" +
      "ame\030\001 \001(\tR\rresource_name\022\r\n\005quota\030\002 \001(\001\022" +
      "\r\n\005usage\030\003 \001(\001\"w\n\017OverageResponse\022\036\n\006hea" +
      "der\030\001 \001(\0132\016.header.Header\022%\n\005error\030\002 \001(\013" +
      "2\026.svcerror.ServiceError\022\035\n\007overage\030\003 \001(" +
      "\0132\014.qms.Overage\"t\n\013OverageList\022\036\n\006header" +
      "\030\001 \001(\0132\016.header.Header\022%\n\005error\030\002 \001(\0132\026." +
      "svcerror.ServiceError\022\036\n\010overages\030\003 \003(\0132" +
      "\014.qms.Overage\"r\n\tIsOverage\022\036\n\006header\030\001 \001" +
      "(\0132\016.header.Header\022%\n\005error\030\002 \001(\0132\026.svce" +
      "rror.ServiceError\022\036\n\nis_overage\030\003 \001(\010R\ni" +
      "s_overageBR\n\030org.cyverse.de.protobufsB\024Q" +
      "MSOveragesProtobufsP\001Z\036github.com/cyvers" +
      "e-de/p/go/qmsb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
          org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor(),
        });
    internal_static_qms_Overage_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_qms_Overage_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_Overage_descriptor,
        new java.lang.String[] { "ResourceName", "Quota", "Usage", });
    internal_static_qms_OverageResponse_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_qms_OverageResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_OverageResponse_descriptor,
        new java.lang.String[] { "Header", "Error", "Overage", });
    internal_static_qms_OverageList_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_qms_OverageList_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_OverageList_descriptor,
        new java.lang.String[] { "Header", "Error", "Overages", });
    internal_static_qms_IsOverage_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_qms_IsOverage_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_IsOverage_descriptor,
        new java.lang.String[] { "Header", "Error", "IsOverage", });
    descriptor.resolveAllFeaturesImmutable();
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
    org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
