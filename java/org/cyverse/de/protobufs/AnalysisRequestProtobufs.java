// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: analysis_requests.proto

package org.cyverse.de.protobufs;

public final class AnalysisRequestProtobufs {
  private AnalysisRequestProtobufs() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_analysis_AnalysisRecordLookupRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_analysis_AnalysisRecordLookupRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_analysis_AnalysisRecordResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_analysis_AnalysisRecordResponse_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_analysis_AnalysisRecordResponse_StatusCountRecord_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_analysis_AnalysisRecordResponse_StatusCountRecord_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_analysis_AnalysisRecordList_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_analysis_AnalysisRecordList_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\027analysis_requests.proto\022\010analysis\032\014hea" +
      "der.proto\032\025analysis_record.proto\032\016svcerr" +
      "or.proto\"\271\001\n\033AnalysisRecordLookupRequest" +
      "\022\025\n\013analysis_id\030\001 \001(\tH\000\022\025\n\013external_id\030\002" +
      " \001(\tH\000\022\021\n\007user_id\030\003 \001(\tH\000\022\022\n\010username\030\004 " +
      "\001(\tH\000\022\036\n\006header\030\005 \001(\0132\016.header.Header\022\027\n" +
      "\017requesting_user\030\006 \001(\tB\014\n\nlookup_ids\"\263\002\n" +
      "\026AnalysisRecordResponse\022\036\n\006header\030\001 \001(\0132" +
      "\016.header.Header\022$\n\010analyses\030\002 \003(\0132\022.anal" +
      "ysis.Analysis\022\021\n\ttimestamp\030\003 \001(\t\022\r\n\005tota" +
      "l\030\004 \001(\003\022V\n\014status_count\030\005 \003(\01322.analysis" +
      ".AnalysisRecordResponse.StatusCountRecor" +
      "dR\014status-count\022%\n\005error\030\006 \001(\0132\026.svcerro" +
      "r.ServiceError\0322\n\021StatusCountRecord\022\r\n\005c" +
      "ount\030\001 \001(\003\022\016\n\006status\030\002 \001(\t\"\201\001\n\022AnalysisR" +
      "ecordList\022\036\n\006header\030\001 \001(\0132\016.header.Heade" +
      "r\022$\n\010analyses\030\002 \003(\0132\022.analysis.Analysis\022" +
      "%\n\005error\030\007 \001(\0132\026.svcerror.ServiceErrorB[" +
      "\n\030org.cyverse.de.protobufsB\030AnalysisRequ" +
      "estProtobufsP\001Z#github.com/cyverse-de/p/" +
      "go/analysisb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
          org.cyverse.de.protobufs.AnalysisRecordProtobufs.getDescriptor(),
          org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor(),
        });
    internal_static_analysis_AnalysisRecordLookupRequest_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_analysis_AnalysisRecordLookupRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_analysis_AnalysisRecordLookupRequest_descriptor,
        new java.lang.String[] { "AnalysisId", "ExternalId", "UserId", "Username", "Header", "RequestingUser", "LookupIds", });
    internal_static_analysis_AnalysisRecordResponse_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_analysis_AnalysisRecordResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_analysis_AnalysisRecordResponse_descriptor,
        new java.lang.String[] { "Header", "Analyses", "Timestamp", "Total", "StatusCount", "Error", });
    internal_static_analysis_AnalysisRecordResponse_StatusCountRecord_descriptor =
      internal_static_analysis_AnalysisRecordResponse_descriptor.getNestedTypes().get(0);
    internal_static_analysis_AnalysisRecordResponse_StatusCountRecord_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_analysis_AnalysisRecordResponse_StatusCountRecord_descriptor,
        new java.lang.String[] { "Count", "Status", });
    internal_static_analysis_AnalysisRecordList_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_analysis_AnalysisRecordList_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_analysis_AnalysisRecordList_descriptor,
        new java.lang.String[] { "Header", "Analyses", "Error", });
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
    org.cyverse.de.protobufs.AnalysisRecordProtobufs.getDescriptor();
    org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
