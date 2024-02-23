// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_quotas.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface QuotaResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.QuotaResponse)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * Contains telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <pre>
   * Contains telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <pre>
   * Contains telemetry data.
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
   * The quota returned by the request handler.
   * </pre>
   *
   * <code>.qms.Quota quota = 3;</code>
   * @return Whether the quota field is set.
   */
  boolean hasQuota();
  /**
   * <pre>
   * The quota returned by the request handler.
   * </pre>
   *
   * <code>.qms.Quota quota = 3;</code>
   * @return The quota.
   */
  org.cyverse.de.protobufs.Quota getQuota();
  /**
   * <pre>
   * The quota returned by the request handler.
   * </pre>
   *
   * <code>.qms.Quota quota = 3;</code>
   */
  org.cyverse.de.protobufs.QuotaOrBuilder getQuotaOrBuilder();
}
