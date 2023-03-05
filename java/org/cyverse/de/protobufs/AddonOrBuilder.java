// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_addons.proto

package org.cyverse.de.protobufs;

public interface AddonOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.Addon)
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
   * The name of the add-on.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The name.
   */
  java.lang.String getName();
  /**
   * <pre>
   * The name of the add-on.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The bytes for name.
   */
  com.google.protobuf.ByteString
      getNameBytes();

  /**
   * <pre>
   * The description of the add-on.
   * </pre>
   *
   * <code>string description = 3;</code>
   * @return The description.
   */
  java.lang.String getDescription();
  /**
   * <pre>
   * The description of the add-on.
   * </pre>
   *
   * <code>string description = 3;</code>
   * @return The bytes for description.
   */
  com.google.protobuf.ByteString
      getDescriptionBytes();

  /**
   * <pre>
   * The resource type of the add-on.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   * @return Whether the resourceType field is set.
   */
  boolean hasResourceType();
  /**
   * <pre>
   * The resource type of the add-on.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  org.cyverse.de.protobufs.ResourceType getResourceType();
  /**
   * <pre>
   * The resource type of the add-on.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   */
  org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypeOrBuilder();

  /**
   * <pre>
   * How much of the resource type is added to the quota by the add-on.
   * </pre>
   *
   * <code>float default_amount = 5 [json_name = "default_amount"];</code>
   * @return The defaultAmount.
   */
  float getDefaultAmount();

  /**
   * <pre>
   * Whether a user must pay for the add-on. Not whether the user has paid.
   * </pre>
   *
   * <code>bool default_paid = 6 [json_name = "default_paid"];</code>
   * @return The defaultPaid.
   */
  boolean getDefaultPaid();
}
