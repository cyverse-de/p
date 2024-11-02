// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_addons.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

public interface AddonRateResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.AddonRateResponse)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * Can contain telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <pre>
   * Can contain telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <pre>
   * Can contain telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <pre>
   * Contains error info from the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <pre>
   * Contains error info from the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <pre>
   * Contains error info from the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <pre>
   * The addon rate object returned by the request handler.
   * </pre>
   *
   * <code>.qms.AddonRate addon_rate = 3 [json_name = "addon_rate"];</code>
   * @return Whether the addonRate field is set.
   */
  boolean hasAddonRate();
  /**
   * <pre>
   * The addon rate object returned by the request handler.
   * </pre>
   *
   * <code>.qms.AddonRate addon_rate = 3 [json_name = "addon_rate"];</code>
   * @return The addonRate.
   */
  org.cyverse.de.protobufs.AddonRate getAddonRate();
  /**
   * <pre>
   * The addon rate object returned by the request handler.
   * </pre>
   *
   * <code>.qms.AddonRate addon_rate = 3 [json_name = "addon_rate"];</code>
   */
  org.cyverse.de.protobufs.AddonRateOrBuilder getAddonRateOrBuilder();
}
