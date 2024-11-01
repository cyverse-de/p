// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_plans.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

public interface AddPlanRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.AddPlanRequest)
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
   * The plan to add to the system.
   * </pre>
   *
   * <code>.qms.Plan plan = 2;</code>
   * @return Whether the plan field is set.
   */
  boolean hasPlan();
  /**
   * <pre>
   * The plan to add to the system.
   * </pre>
   *
   * <code>.qms.Plan plan = 2;</code>
   * @return The plan.
   */
  org.cyverse.de.protobufs.Plan getPlan();
  /**
   * <pre>
   * The plan to add to the system.
   * </pre>
   *
   * <code>.qms.Plan plan = 2;</code>
   */
  org.cyverse.de.protobufs.PlanOrBuilder getPlanOrBuilder();
}
