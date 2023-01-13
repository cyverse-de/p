// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_usages.proto

package org.cyverse.de.protobufs;

public interface UsageResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:UsageResponse)
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
   * <pre>
   * Error information returned by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <pre>
   * Error information returned by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <pre>
   * Error information returned by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <pre>
   * Contains the usage info returned by the request handler.
   * </pre>
   *
   * <code>.Usage usage = 3;</code>
   * @return Whether the usage field is set.
   */
  boolean hasUsage();
  /**
   * <pre>
   * Contains the usage info returned by the request handler.
   * </pre>
   *
   * <code>.Usage usage = 3;</code>
   * @return The usage.
   */
  org.cyverse.de.protobufs.Usage getUsage();
  /**
   * <pre>
   * Contains the usage info returned by the request handler.
   * </pre>
   *
   * <code>.Usage usage = 3;</code>
   */
  org.cyverse.de.protobufs.UsageOrBuilder getUsageOrBuilder();
}
