// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_updates.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface AddUpdateResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.AddUpdateResponse)
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
   * Error information returned by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <pre>
   * Error information returned by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <pre>
   * Error information returned by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <pre>
   * The update added to the system.
   * </pre>
   *
   * <code>.qms.Update update = 3;</code>
   * @return Whether the update field is set.
   */
  boolean hasUpdate();
  /**
   * <pre>
   * The update added to the system.
   * </pre>
   *
   * <code>.qms.Update update = 3;</code>
   * @return The update.
   */
  org.cyverse.de.protobufs.Update getUpdate();
  /**
   * <pre>
   * The update added to the system.
   * </pre>
   *
   * <code>.qms.Update update = 3;</code>
   */
  org.cyverse.de.protobufs.UpdateOrBuilder getUpdateOrBuilder();
}
