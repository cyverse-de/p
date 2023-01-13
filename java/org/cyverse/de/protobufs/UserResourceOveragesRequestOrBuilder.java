// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_requests.proto

package org.cyverse.de.protobufs;

public interface UserResourceOveragesRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:UserResourceOveragesRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * Contains telemetry data.
   * </pre>
   *
   * <code>.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <pre>
   * Contains telemetry data.
   * </pre>
   *
   * <code>.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <pre>
   * Contains telemetry data.
   * </pre>
   *
   * <code>.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <pre>
   * A user's username.
   * </pre>
   *
   * <code>string username = 2;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <pre>
   * A user's username.
   * </pre>
   *
   * <code>string username = 2;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();

  /**
   * <pre>
   * The name of the resource type to look up overages for.
   * </pre>
   *
   * <code>string resource_name = 3 [json_name = "resource_name"];</code>
   * @return The resourceName.
   */
  java.lang.String getResourceName();
  /**
   * <pre>
   * The name of the resource type to look up overages for.
   * </pre>
   *
   * <code>string resource_name = 3 [json_name = "resource_name"];</code>
   * @return The bytes for resourceName.
   */
  com.google.protobuf.ByteString
      getResourceNameBytes();
}
