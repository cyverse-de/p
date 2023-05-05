// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: groups.proto

package org.cyverse.de.protobufs;

public interface ResourceInOrBuilder extends
    // @@protoc_insertion_point(interface_extends:groups.ResourceIn)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The resource name.
   * </pre>
   *
   * <code>string name = 1;</code>
   * @return The name.
   */
  java.lang.String getName();
  /**
   * <pre>
   * The resource name.
   * </pre>
   *
   * <code>string name = 1;</code>
   * @return The bytes for name.
   */
  com.google.protobuf.ByteString
      getNameBytes();

  /**
   * <pre>
   * The resource type name.
   * </pre>
   *
   * <code>string resource_type = 2 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  java.lang.String getResourceType();
  /**
   * <pre>
   * The resource type name.
   * </pre>
   *
   * <code>string resource_type = 2 [json_name = "resource_type"];</code>
   * @return The bytes for resourceType.
   */
  com.google.protobuf.ByteString
      getResourceTypeBytes();
}
