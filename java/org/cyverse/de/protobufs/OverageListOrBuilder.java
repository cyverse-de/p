// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_overages.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface OverageListOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.OverageList)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The header used for passing telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <pre>
   * The header used for passing telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <pre>
   * The header used for passing telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <pre>
   * Contains any errors generated by the handler emitting the response.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <pre>
   * Contains any errors generated by the handler emitting the response.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <pre>
   * Contains any errors generated by the handler emitting the response.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <pre>
   * The list of overages returned by the handler emitting the response.
   * </pre>
   *
   * <code>repeated .qms.Overage overages = 3;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Overage> 
      getOveragesList();
  /**
   * <pre>
   * The list of overages returned by the handler emitting the response.
   * </pre>
   *
   * <code>repeated .qms.Overage overages = 3;</code>
   */
  org.cyverse.de.protobufs.Overage getOverages(int index);
  /**
   * <pre>
   * The list of overages returned by the handler emitting the response.
   * </pre>
   *
   * <code>repeated .qms.Overage overages = 3;</code>
   */
  int getOveragesCount();
  /**
   * <pre>
   * The list of overages returned by the handler emitting the response.
   * </pre>
   *
   * <code>repeated .qms.Overage overages = 3;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.OverageOrBuilder> 
      getOveragesOrBuilderList();
  /**
   * <pre>
   * The list of overages returned by the handler emitting the response.
   * </pre>
   *
   * <code>repeated .qms.Overage overages = 3;</code>
   */
  org.cyverse.de.protobufs.OverageOrBuilder getOveragesOrBuilder(
      int index);
}
