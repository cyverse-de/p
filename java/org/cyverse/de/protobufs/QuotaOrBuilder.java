// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_quotas.proto
// Protobuf Java Version: 4.29.3

package org.cyverse.de.protobufs;

public interface QuotaOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.Quota)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The unique identifier.
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <pre>
   * The unique identifier.
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <pre>
   * The quota value (aka limit).
   * </pre>
   *
   * <code>double quota = 2;</code>
   * @return The quota.
   */
  double getQuota();

  /**
   * <pre>
   * The resource type the quota value applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 3 [json_name = "resource_type"];</code>
   * @return Whether the resourceType field is set.
   */
  boolean hasResourceType();
  /**
   * <pre>
   * The resource type the quota value applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 3 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  org.cyverse.de.protobufs.ResourceType getResourceType();
  /**
   * <pre>
   * The resource type the quota value applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 3 [json_name = "resource_type"];</code>
   */
  org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypeOrBuilder();

  /**
   * <pre>
   * A freeform text field containing info about who created the quota.
   * </pre>
   *
   * <code>string CreatedBy = 4 [json_name = "created_by"];</code>
   * @return The createdBy.
   */
  java.lang.String getCreatedBy();
  /**
   * <pre>
   * A freeform text field containing info about who created the quota.
   * </pre>
   *
   * <code>string CreatedBy = 4 [json_name = "created_by"];</code>
   * @return The bytes for createdBy.
   */
  com.google.protobuf.ByteString
      getCreatedByBytes();

  /**
   * <pre>
   * When the quota was created.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp CreatedAt = 5 [json_name = "created_at"];</code>
   * @return Whether the createdAt field is set.
   */
  boolean hasCreatedAt();
  /**
   * <pre>
   * When the quota was created.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp CreatedAt = 5 [json_name = "created_at"];</code>
   * @return The createdAt.
   */
  com.google.protobuf.Timestamp getCreatedAt();
  /**
   * <pre>
   * When the quota was created.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp CreatedAt = 5 [json_name = "created_at"];</code>
   */
  com.google.protobuf.TimestampOrBuilder getCreatedAtOrBuilder();

  /**
   * <pre>
   * A freeform text field containing info about who last modified the quota.
   * </pre>
   *
   * <code>string LastModifiedBy = 6 [json_name = "last_modified_by"];</code>
   * @return The lastModifiedBy.
   */
  java.lang.String getLastModifiedBy();
  /**
   * <pre>
   * A freeform text field containing info about who last modified the quota.
   * </pre>
   *
   * <code>string LastModifiedBy = 6 [json_name = "last_modified_by"];</code>
   * @return The bytes for lastModifiedBy.
   */
  com.google.protobuf.ByteString
      getLastModifiedByBytes();

  /**
   * <pre>
   * When the quota was last modified.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp LastModifiedAt = 7 [json_name = "last_modified_at"];</code>
   * @return Whether the lastModifiedAt field is set.
   */
  boolean hasLastModifiedAt();
  /**
   * <pre>
   * When the quota was last modified.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp LastModifiedAt = 7 [json_name = "last_modified_at"];</code>
   * @return The lastModifiedAt.
   */
  com.google.protobuf.Timestamp getLastModifiedAt();
  /**
   * <pre>
   * When the quota was last modified.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp LastModifiedAt = 7 [json_name = "last_modified_at"];</code>
   */
  com.google.protobuf.TimestampOrBuilder getLastModifiedAtOrBuilder();

  /**
   * <pre>
   * The unique identifier of the subscription that the quota is associated with.
   * </pre>
   *
   * <code>string subscription_id = 8 [json_name = "subscription_id"];</code>
   * @return The subscriptionId.
   */
  java.lang.String getSubscriptionId();
  /**
   * <pre>
   * The unique identifier of the subscription that the quota is associated with.
   * </pre>
   *
   * <code>string subscription_id = 8 [json_name = "subscription_id"];</code>
   * @return The bytes for subscriptionId.
   */
  com.google.protobuf.ByteString
      getSubscriptionIdBytes();
}
