// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: groups.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

public interface AbbreviatedPermissionOrBuilder extends
    // @@protoc_insertion_point(interface_extends:groups.AbbreviatedPermission)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The permission identifier.
   * </pre>
   *
   * <code>string permission_id = 1 [json_name = "permission_id"];</code>
   * @return The permissionId.
   */
  java.lang.String getPermissionId();
  /**
   * <pre>
   * The permission identifier.
   * </pre>
   *
   * <code>string permission_id = 1 [json_name = "permission_id"];</code>
   * @return The bytes for permissionId.
   */
  com.google.protobuf.ByteString
      getPermissionIdBytes();

  /**
   * <pre>
   * The name of the resource.
   * </pre>
   *
   * <code>string resource_name = 2 [json_name = "resource_name"];</code>
   * @return The resourceName.
   */
  java.lang.String getResourceName();
  /**
   * <pre>
   * The name of the resource.
   * </pre>
   *
   * <code>string resource_name = 2 [json_name = "resource_name"];</code>
   * @return The bytes for resourceName.
   */
  com.google.protobuf.ByteString
      getResourceNameBytes();

  /**
   * <pre>
   * The type of the resource.
   * </pre>
   *
   * <code>string resource_type = 3 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  java.lang.String getResourceType();
  /**
   * <pre>
   * The type of the resource.
   * </pre>
   *
   * <code>string resource_type = 3 [json_name = "resource_type"];</code>
   * @return The bytes for resourceType.
   */
  com.google.protobuf.ByteString
      getResourceTypeBytes();

  /**
   * <pre>
   * The permission level.
   * </pre>
   *
   * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
   * @return The enum numeric value on the wire for permissionLevel.
   */
  int getPermissionLevelValue();
  /**
   * <pre>
   * The permission level.
   * </pre>
   *
   * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
   * @return The permissionLevel.
   */
  org.cyverse.de.protobufs.PermissionLevel getPermissionLevel();
}
