// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_usages.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface UsageOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.Usage)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The unique identifier
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <pre>
   * The unique identifier
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <pre>
   * How much the resource has been used.
   * </pre>
   *
   * <code>double usage = 2;</code>
   * @return The usage.
   */
  double getUsage();

  /**
   * <pre>
   * The unique identifier for the subscription the usage is associated with.
   * </pre>
   *
   * <code>string subscription_id = 3 [json_name = "subscription_id"];</code>
   * @return The subscriptionId.
   */
  java.lang.String getSubscriptionId();
  /**
   * <pre>
   * The unique identifier for the subscription the usage is associated with.
   * </pre>
   *
   * <code>string subscription_id = 3 [json_name = "subscription_id"];</code>
   * @return The bytes for subscriptionId.
   */
  com.google.protobuf.ByteString
      getSubscriptionIdBytes();

  /**
   * <pre>
   * The resource type the usage applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   * @return Whether the resourceType field is set.
   */
  boolean hasResourceType();
  /**
   * <pre>
   * The resource type the usage applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  org.cyverse.de.protobufs.ResourceType getResourceType();
  /**
   * <pre>
   * The resource type the usage applies to.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   */
  org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypeOrBuilder();

  /**
   * <pre>
   * Who created the usage record. Probably not the name of a user.
   * </pre>
   *
   * <code>string CreatedBy = 5 [json_name = "created_by"];</code>
   * @return The createdBy.
   */
  java.lang.String getCreatedBy();
  /**
   * <pre>
   * Who created the usage record. Probably not the name of a user.
   * </pre>
   *
   * <code>string CreatedBy = 5 [json_name = "created_by"];</code>
   * @return The bytes for createdBy.
   */
  com.google.protobuf.ByteString
      getCreatedByBytes();

  /**
   * <pre>
   * When the usage record was created.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp CreatedAt = 6 [json_name = "created_at"];</code>
   * @return Whether the createdAt field is set.
   */
  boolean hasCreatedAt();
  /**
   * <pre>
   * When the usage record was created.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp CreatedAt = 6 [json_name = "created_at"];</code>
   * @return The createdAt.
   */
  com.google.protobuf.Timestamp getCreatedAt();
  /**
   * <pre>
   * When the usage record was created.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp CreatedAt = 6 [json_name = "created_at"];</code>
   */
  com.google.protobuf.TimestampOrBuilder getCreatedAtOrBuilder();

  /**
   * <pre>
   * Who last modified the usage record. Probably not the name of a user.
   * </pre>
   *
   * <code>string LastModifiedBy = 7 [json_name = "last_modified_by"];</code>
   * @return The lastModifiedBy.
   */
  java.lang.String getLastModifiedBy();
  /**
   * <pre>
   * Who last modified the usage record. Probably not the name of a user.
   * </pre>
   *
   * <code>string LastModifiedBy = 7 [json_name = "last_modified_by"];</code>
   * @return The bytes for lastModifiedBy.
   */
  com.google.protobuf.ByteString
      getLastModifiedByBytes();

  /**
   * <pre>
   * When the usage record was last modified.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp LastModifiedAt = 8 [json_name = "last_modified_at"];</code>
   * @return Whether the lastModifiedAt field is set.
   */
  boolean hasLastModifiedAt();
  /**
   * <pre>
   * When the usage record was last modified.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp LastModifiedAt = 8 [json_name = "last_modified_at"];</code>
   * @return The lastModifiedAt.
   */
  com.google.protobuf.Timestamp getLastModifiedAt();
  /**
   * <pre>
   * When the usage record was last modified.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp LastModifiedAt = 8 [json_name = "last_modified_at"];</code>
   */
  com.google.protobuf.TimestampOrBuilder getLastModifiedAtOrBuilder();
}
