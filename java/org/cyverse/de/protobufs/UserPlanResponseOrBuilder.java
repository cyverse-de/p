// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_user_plans.proto

package org.cyverse.de.protobufs;

public interface UserPlanResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:UserPlanResponse)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <code>.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <code>.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <code>.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <code>.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <code>.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <code>.UserPlan user_plan = 3 [json_name = "user_plan"];</code>
   * @return Whether the userPlan field is set.
   */
  boolean hasUserPlan();
  /**
   * <code>.UserPlan user_plan = 3 [json_name = "user_plan"];</code>
   * @return The userPlan.
   */
  org.cyverse.de.protobufs.UserPlan getUserPlan();
  /**
   * <code>.UserPlan user_plan = 3 [json_name = "user_plan"];</code>
   */
  org.cyverse.de.protobufs.UserPlanOrBuilder getUserPlanOrBuilder();
}
