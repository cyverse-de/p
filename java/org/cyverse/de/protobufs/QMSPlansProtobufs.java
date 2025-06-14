// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_plans.proto
// Protobuf Java Version: 4.31.1

package org.cyverse.de.protobufs;

@com.google.protobuf.Generated
public final class QMSPlansProtobufs {
  private QMSPlansProtobufs() {}
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 31,
      /* patch= */ 1,
      /* suffix= */ "",
      QMSPlansProtobufs.class.getName());
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
    internal_static_qms_QuotaDefault_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_QuotaDefault_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_QuotaDefaultResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_QuotaDefaultResponse_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_QuotaDefaultList_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_QuotaDefaultList_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_PlanRate_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_PlanRate_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_PlanRateResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_PlanRateResponse_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_PlanRateList_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_PlanRateList_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_Plan_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_Plan_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_PlanResponse_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_PlanResponse_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_PlanList_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_PlanList_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_PlanRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_PlanRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_AddPlanRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_AddPlanRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_AddPlanQuotaDefaultRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_AddPlanQuotaDefaultRequest_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_qms_AddPlanRateRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_qms_AddPlanRateRequest_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\017qms_plans.proto\022\003qms\032\037google/protobuf/" +
      "timestamp.proto\032\014header.proto\032\016svcerror." +
      "proto\032\030qms_resource_types.proto\"\273\001\n\014Quot" +
      "aDefault\022\014\n\004uuid\030\001 \001(\t\022 \n\013quota_value\030\002 " +
      "\001(\001R\013quota_value\0227\n\rresource_type\030\003 \001(\0132" +
      "\021.qms.ResourceTypeR\rresource_type\022B\n\016eff" +
      "ective_date\030\004 \001(\0132\032.google.protobuf.Time" +
      "stampR\016effective_date\"\226\001\n\024QuotaDefaultRe" +
      "sponse\022\036\n\006header\030\001 \001(\0132\016.header.Header\022%" +
      "\n\005error\030\002 \001(\0132\026.svcerror.ServiceError\0227\n" +
      "\rquota_default\030\003 \001(\0132\021.qms.QuotaDefaultR" +
      "\rquota_default\"\224\001\n\020QuotaDefaultList\022\036\n\006h" +
      "eader\030\001 \001(\0132\016.header.Header\022%\n\005error\030\002 \001" +
      "(\0132\026.svcerror.ServiceError\0229\n\016quota_defa" +
      "ults\030\003 \003(\0132\021.qms.QuotaDefaultR\016quota_def" +
      "aults\"j\n\010PlanRate\022\014\n\004uuid\030\001 \001(\t\022\014\n\004rate\030" +
      "\002 \001(\001\022B\n\016effective_date\030\003 \001(\0132\032.google.p" +
      "rotobuf.TimestampR\016effective_date\"\206\001\n\020Pl" +
      "anRateResponse\022\036\n\006header\030\001 \001(\0132\016.header." +
      "Header\022%\n\005error\030\002 \001(\0132\026.svcerror.Service" +
      "Error\022+\n\tplan_rate\030\003 \001(\0132\r.qms.PlanRateR" +
      "\tplan_rate\"\204\001\n\014PlanRateList\022\036\n\006header\030\001 " +
      "\001(\0132\016.header.Header\022%\n\005error\030\002 \001(\0132\026.svc" +
      "error.ServiceError\022-\n\nplan_rates\030\003 \003(\0132\r" +
      ".qms.PlanRateR\nplan_rates\"\253\001\n\004Plan\022\014\n\004uu" +
      "id\030\001 \001(\t\022\014\n\004name\030\002 \001(\t\022\023\n\013description\030\003 " +
      "\001(\t\022C\n\023plan_quota_defaults\030\004 \003(\0132\021.qms.Q" +
      "uotaDefaultR\023plan_quota_defaults\022-\n\nplan" +
      "_rates\030\005 \003(\0132\r.qms.PlanRateR\nplan_rates\"" +
      "n\n\014PlanResponse\022\036\n\006header\030\001 \001(\0132\016.header" +
      ".Header\022%\n\005error\030\002 \001(\0132\026.svcerror.Servic" +
      "eError\022\027\n\004plan\030\003 \001(\0132\t.qms.Plan\"k\n\010PlanL" +
      "ist\022\036\n\006header\030\001 \001(\0132\016.header.Header\022%\n\005e" +
      "rror\030\002 \001(\0132\026.svcerror.ServiceError\022\030\n\005pl" +
      "ans\030\003 \003(\0132\t.qms.Plan\"G\n\013PlanRequest\022\036\n\006h" +
      "eader\030\001 \001(\0132\016.header.Header\022\030\n\007plan_id\030\002" +
      " \001(\tR\007plan_id\"I\n\016AddPlanRequest\022\036\n\006heade" +
      "r\030\001 \001(\0132\016.header.Header\022\027\n\004plan\030\002 \001(\0132\t." +
      "qms.Plan\"\223\001\n\032AddPlanQuotaDefaultRequest\022" +
      "\036\n\006header\030\001 \001(\0132\016.header.Header\022\034\n\tplan_" +
      "name\030\002 \001(\tR\tplan_name\0227\n\rquota_default\030\003" +
      " \001(\0132\021.qms.QuotaDefaultR\rquota_default\"\177" +
      "\n\022AddPlanRateRequest\022\036\n\006header\030\001 \001(\0132\016.h" +
      "eader.Header\022\034\n\tplan_name\030\002 \001(\tR\tplan_na" +
      "me\022+\n\tplan_rate\030\003 \001(\0132\r.qms.PlanRateR\tpl" +
      "an_rateBO\n\030org.cyverse.de.protobufsB\021QMS" +
      "PlansProtobufsP\001Z\036github.com/cyverse-de/" +
      "p/go/qmsb\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          com.google.protobuf.TimestampProto.getDescriptor(),
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
          org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor(),
          org.cyverse.de.protobufs.QMSResourceTypeProtobufs.getDescriptor(),
        });
    internal_static_qms_QuotaDefault_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_qms_QuotaDefault_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_QuotaDefault_descriptor,
        new java.lang.String[] { "Uuid", "QuotaValue", "ResourceType", "EffectiveDate", });
    internal_static_qms_QuotaDefaultResponse_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_qms_QuotaDefaultResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_QuotaDefaultResponse_descriptor,
        new java.lang.String[] { "Header", "Error", "QuotaDefault", });
    internal_static_qms_QuotaDefaultList_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_qms_QuotaDefaultList_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_QuotaDefaultList_descriptor,
        new java.lang.String[] { "Header", "Error", "QuotaDefaults", });
    internal_static_qms_PlanRate_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_qms_PlanRate_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_PlanRate_descriptor,
        new java.lang.String[] { "Uuid", "Rate", "EffectiveDate", });
    internal_static_qms_PlanRateResponse_descriptor =
      getDescriptor().getMessageTypes().get(4);
    internal_static_qms_PlanRateResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_PlanRateResponse_descriptor,
        new java.lang.String[] { "Header", "Error", "PlanRate", });
    internal_static_qms_PlanRateList_descriptor =
      getDescriptor().getMessageTypes().get(5);
    internal_static_qms_PlanRateList_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_PlanRateList_descriptor,
        new java.lang.String[] { "Header", "Error", "PlanRates", });
    internal_static_qms_Plan_descriptor =
      getDescriptor().getMessageTypes().get(6);
    internal_static_qms_Plan_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_Plan_descriptor,
        new java.lang.String[] { "Uuid", "Name", "Description", "PlanQuotaDefaults", "PlanRates", });
    internal_static_qms_PlanResponse_descriptor =
      getDescriptor().getMessageTypes().get(7);
    internal_static_qms_PlanResponse_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_PlanResponse_descriptor,
        new java.lang.String[] { "Header", "Error", "Plan", });
    internal_static_qms_PlanList_descriptor =
      getDescriptor().getMessageTypes().get(8);
    internal_static_qms_PlanList_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_PlanList_descriptor,
        new java.lang.String[] { "Header", "Error", "Plans", });
    internal_static_qms_PlanRequest_descriptor =
      getDescriptor().getMessageTypes().get(9);
    internal_static_qms_PlanRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_PlanRequest_descriptor,
        new java.lang.String[] { "Header", "PlanId", });
    internal_static_qms_AddPlanRequest_descriptor =
      getDescriptor().getMessageTypes().get(10);
    internal_static_qms_AddPlanRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_AddPlanRequest_descriptor,
        new java.lang.String[] { "Header", "Plan", });
    internal_static_qms_AddPlanQuotaDefaultRequest_descriptor =
      getDescriptor().getMessageTypes().get(11);
    internal_static_qms_AddPlanQuotaDefaultRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_AddPlanQuotaDefaultRequest_descriptor,
        new java.lang.String[] { "Header", "PlanName", "QuotaDefault", });
    internal_static_qms_AddPlanRateRequest_descriptor =
      getDescriptor().getMessageTypes().get(12);
    internal_static_qms_AddPlanRateRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_qms_AddPlanRateRequest_descriptor,
        new java.lang.String[] { "Header", "PlanName", "PlanRate", });
    descriptor.resolveAllFeaturesImmutable();
    com.google.protobuf.TimestampProto.getDescriptor();
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
    org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor();
    org.cyverse.de.protobufs.QMSResourceTypeProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
