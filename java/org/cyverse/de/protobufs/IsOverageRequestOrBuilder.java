// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_requests.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface IsOverageRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.IsOverageRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <pre>
   * A username.
   * </pre>
   *
   * <code>string username = 2;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <pre>
   * A username.
   * </pre>
   *
   * <code>string username = 2;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();

  /**
   * <pre>
   * The name of the resource type to check for usage overages by the user.
   * </pre>
   *
   * <code>string resource_name = 3 [json_name = "resource_name"];</code>
   * @return The resourceName.
   */
  java.lang.String getResourceName();
  /**
   * <pre>
   * The name of the resource type to check for usage overages by the user.
   * </pre>
   *
   * <code>string resource_name = 3 [json_name = "resource_name"];</code>
   * @return The bytes for resourceName.
   */
  com.google.protobuf.ByteString
      getResourceNameBytes();
}
