// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_addons.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface AddonListResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.AddonListResponse)
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
   * The list of add-ons returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Addon addons = 3;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Addon> 
      getAddonsList();
  /**
   * <pre>
   * The list of add-ons returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Addon addons = 3;</code>
   */
  org.cyverse.de.protobufs.Addon getAddons(int index);
  /**
   * <pre>
   * The list of add-ons returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Addon addons = 3;</code>
   */
  int getAddonsCount();
  /**
   * <pre>
   * The list of add-ons returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Addon addons = 3;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.AddonOrBuilder> 
      getAddonsOrBuilderList();
  /**
   * <pre>
   * The list of add-ons returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Addon addons = 3;</code>
   */
  org.cyverse.de.protobufs.AddonOrBuilder getAddonsOrBuilder(
      int index);
}
