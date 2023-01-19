// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_addons.proto

package org.cyverse.de.protobufs;

public interface AddonResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:AddonResponse)
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
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <pre>
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <pre>
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <pre>
   * The add-on returned by the request handler.
   * </pre>
   *
   * <code>.Addon addon = 3;</code>
   * @return Whether the addon field is set.
   */
  boolean hasAddon();
  /**
   * <pre>
   * The add-on returned by the request handler.
   * </pre>
   *
   * <code>.Addon addon = 3;</code>
   * @return The addon.
   */
  org.cyverse.de.protobufs.Addon getAddon();
  /**
   * <pre>
   * The add-on returned by the request handler.
   * </pre>
   *
   * <code>.Addon addon = 3;</code>
   */
  org.cyverse.de.protobufs.AddonOrBuilder getAddonOrBuilder();
}