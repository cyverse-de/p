// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: requests.proto

package org.cyverse.de.protobufs;

public interface ByUsernameOrBuilder extends
    // @@protoc_insertion_point(interface_extends:ByUsername)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <code>string username = 2;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <code>string username = 2;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();
}