// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_plans.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface PlanListOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.PlanList)
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
   * Contains error data returned by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <pre>
   * Contains error data returned by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <pre>
   * Contains error data returned by the request handler.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <pre>
   * A list of plans returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Plan plans = 3;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Plan> 
      getPlansList();
  /**
   * <pre>
   * A list of plans returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Plan plans = 3;</code>
   */
  org.cyverse.de.protobufs.Plan getPlans(int index);
  /**
   * <pre>
   * A list of plans returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Plan plans = 3;</code>
   */
  int getPlansCount();
  /**
   * <pre>
   * A list of plans returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Plan plans = 3;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.PlanOrBuilder> 
      getPlansOrBuilderList();
  /**
   * <pre>
   * A list of plans returned by the request handler.
   * </pre>
   *
   * <code>repeated .qms.Plan plans = 3;</code>
   */
  org.cyverse.de.protobufs.PlanOrBuilder getPlansOrBuilder(
      int index);
}
