// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_plans.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface PlanRateListOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.PlanRateList)
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
   * The list of plan rate objects returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.PlanRate plan_rates = 3 [json_name = "plan_rates"];</code>
   */
  java.util.List<org.cyverse.de.protobufs.PlanRate> 
      getPlanRatesList();
  /**
   * <pre>
   * The list of plan rate objects returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.PlanRate plan_rates = 3 [json_name = "plan_rates"];</code>
   */
  org.cyverse.de.protobufs.PlanRate getPlanRates(int index);
  /**
   * <pre>
   * The list of plan rate objects returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.PlanRate plan_rates = 3 [json_name = "plan_rates"];</code>
   */
  int getPlanRatesCount();
  /**
   * <pre>
   * The list of plan rate objects returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.PlanRate plan_rates = 3 [json_name = "plan_rates"];</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.PlanRateOrBuilder> 
      getPlanRatesOrBuilderList();
  /**
   * <pre>
   * The list of plan rate objects returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.PlanRate plan_rates = 3 [json_name = "plan_rates"];</code>
   */
  org.cyverse.de.protobufs.PlanRateOrBuilder getPlanRatesOrBuilder(
      int index);
}
