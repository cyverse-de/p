// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_subscriptions.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface ChangeSubscriptionRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.ChangeSubscriptionRequest)
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
   * A username for the user whose subscription is being changed.
   * </pre>
   *
   * <code>string username = 2;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <pre>
   * A username for the user whose subscription is being changed.
   * </pre>
   *
   * <code>string username = 2;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();

  /**
   * <code>string uuid = 3;</code>
   * @return Whether the uuid field is set.
   */
  boolean hasUuid();
  /**
   * <code>string uuid = 3;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <code>string uuid = 3;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <code>string name = 4;</code>
   * @return Whether the name field is set.
   */
  boolean hasName();
  /**
   * <code>string name = 4;</code>
   * @return The name.
   */
  java.lang.String getName();
  /**
   * <code>string name = 4;</code>
   * @return The bytes for name.
   */
  com.google.protobuf.ByteString
      getNameBytes();

  /**
   * <pre>
   * The number of subscription periods that the subscription will be good for. The subscription period is one year,
   * so purchasing a subscription for 3 periods will create a subscription for 3 years. Consumable resources are also
   * allocated based on the number of periods, so if a subscription plan comes with 2000 CPU Hours, for example, then
   * a user who purchases 3 subscription periods will get 6000 CPU hours to use over the course of three years.
   * </pre>
   *
   * <code>int32 periods = 5;</code>
   * @return The periods.
   */
  int getPeriods();

  /**
   * <pre>
   * The end-date of the subscription. Accepted formats are `YYYY-MM-DD`, `YYYY-MM-DDThh:mm:ss`,
   * `YYYY-MM-DDThh:mm:ssZ` and `YYYY-MM-DDThh:mm:ss+hh:mm`. Date and tiestamps without time zones are assumed to
   * be in the time zone used by the CyVerse Discovery Environment itself.
   * </pre>
   *
   * <code>string end_date = 6;</code>
   * @return The endDate.
   */
  java.lang.String getEndDate();
  /**
   * <pre>
   * The end-date of the subscription. Accepted formats are `YYYY-MM-DD`, `YYYY-MM-DDThh:mm:ss`,
   * `YYYY-MM-DDThh:mm:ssZ` and `YYYY-MM-DDThh:mm:ss+hh:mm`. Date and tiestamps without time zones are assumed to
   * be in the time zone used by the CyVerse Discovery Environment itself.
   * </pre>
   *
   * <code>string end_date = 6;</code>
   * @return The bytes for endDate.
   */
  com.google.protobuf.ByteString
      getEndDateBytes();

  org.cyverse.de.protobufs.ChangeSubscriptionRequest.PlanCase getPlanCase();
}
