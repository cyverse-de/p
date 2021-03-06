// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_overages.proto

package org.cyverse.de.protobufs;

public interface OverageListOrBuilder extends
    // @@protoc_insertion_point(interface_extends:OverageList)
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
   * <code>repeated .Overage overages = 3;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Overage> 
      getOveragesList();
  /**
   * <code>repeated .Overage overages = 3;</code>
   */
  org.cyverse.de.protobufs.Overage getOverages(int index);
  /**
   * <code>repeated .Overage overages = 3;</code>
   */
  int getOveragesCount();
  /**
   * <code>repeated .Overage overages = 3;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.OverageOrBuilder> 
      getOveragesOrBuilderList();
  /**
   * <code>repeated .Overage overages = 3;</code>
   */
  org.cyverse.de.protobufs.OverageOrBuilder getOveragesOrBuilder(
      int index);
}
