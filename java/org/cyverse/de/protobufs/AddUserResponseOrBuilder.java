// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_users.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface AddUserResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.AddUserResponse)
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
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <pre>
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <pre>
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <pre>
   * The unique identfier of the newly added user.
   * </pre>
   *
   * <code>string uuid = 3;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <pre>
   * The unique identfier of the newly added user.
   * </pre>
   *
   * <code>string uuid = 3;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <pre>
   * The username of the newly added user.
   * </pre>
   *
   * <code>string username = 4;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <pre>
   * The username of the newly added user.
   * </pre>
   *
   * <code>string username = 4;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();

  /**
   * <pre>
   * The name of the plan the newly added user is subscribed to.
   * </pre>
   *
   * <code>string plan_name = 5;</code>
   * @return The planName.
   */
  java.lang.String getPlanName();
  /**
   * <pre>
   * The name of the plan the newly added user is subscribed to.
   * </pre>
   *
   * <code>string plan_name = 5;</code>
   * @return The bytes for planName.
   */
  com.google.protobuf.ByteString
      getPlanNameBytes();

  /**
   * <pre>
   * The unique identifier for the plan the newly added user is subscribed to.
   * </pre>
   *
   * <code>string plan_uuid = 6;</code>
   * @return The planUuid.
   */
  java.lang.String getPlanUuid();
  /**
   * <pre>
   * The unique identifier for the plan the newly added user is subscribed to.
   * </pre>
   *
   * <code>string plan_uuid = 6;</code>
   * @return The bytes for planUuid.
   */
  com.google.protobuf.ByteString
      getPlanUuidBytes();
}
