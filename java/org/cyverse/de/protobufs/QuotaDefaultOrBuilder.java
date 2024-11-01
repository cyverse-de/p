// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_plans.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

public interface QuotaDefaultOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.QuotaDefault)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The unique identifier/primary key for the quota default.
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <pre>
   * The unique identifier/primary key for the quota default.
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <pre>
   * The value of the quota default.
   * </pre>
   *
   * <code>double quota_value = 2 [json_name = "quota_value"];</code>
   * @return The quotaValue.
   */
  double getQuotaValue();

  /**
   * <pre>
   * The resource type the quota applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 3 [json_name = "resource_type"];</code>
   * @return Whether the resourceType field is set.
   */
  boolean hasResourceType();
  /**
   * <pre>
   * The resource type the quota applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 3 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  org.cyverse.de.protobufs.ResourceType getResourceType();
  /**
   * <pre>
   * The resource type the quota applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 3 [json_name = "resource_type"];</code>
   */
  org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypeOrBuilder();

  /**
   * <pre>
   * The date that quota default becomes effective. There can be multiple quota defaults for the same resource type,
   * and the quota default that is currently effective is always the one with the most recent effective date that
   * occurs in the past.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_date = 4 [json_name = "effective_date"];</code>
   * @return Whether the effectiveDate field is set.
   */
  boolean hasEffectiveDate();
  /**
   * <pre>
   * The date that quota default becomes effective. There can be multiple quota defaults for the same resource type,
   * and the quota default that is currently effective is always the one with the most recent effective date that
   * occurs in the past.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_date = 4 [json_name = "effective_date"];</code>
   * @return The effectiveDate.
   */
  com.google.protobuf.Timestamp getEffectiveDate();
  /**
   * <pre>
   * The date that quota default becomes effective. There can be multiple quota defaults for the same resource type,
   * and the quota default that is currently effective is always the one with the most recent effective date that
   * occurs in the past.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_date = 4 [json_name = "effective_date"];</code>
   */
  com.google.protobuf.TimestampOrBuilder getEffectiveDateOrBuilder();
}
