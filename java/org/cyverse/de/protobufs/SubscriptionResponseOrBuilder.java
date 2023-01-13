// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_subscriptions.proto

package org.cyverse.de.protobufs;

public interface SubscriptionResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:SubscriptionResponse)
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
   * The user plan/subscription returned by the request handler.
   * </pre>
   *
   * <code>.Subscription subscription = 3 [json_name = "subscription"];</code>
   * @return Whether the subscription field is set.
   */
  boolean hasSubscription();
  /**
   * <pre>
   * The user plan/subscription returned by the request handler.
   * </pre>
   *
   * <code>.Subscription subscription = 3 [json_name = "subscription"];</code>
   * @return The subscription.
   */
  org.cyverse.de.protobufs.Subscription getSubscription();
  /**
   * <pre>
   * The user plan/subscription returned by the request handler.
   * </pre>
   *
   * <code>.Subscription subscription = 3 [json_name = "subscription"];</code>
   */
  org.cyverse.de.protobufs.SubscriptionOrBuilder getSubscriptionOrBuilder();
}
