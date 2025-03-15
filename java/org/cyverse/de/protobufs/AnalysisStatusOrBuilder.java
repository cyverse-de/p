// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: analysis_status.proto
// Protobuf Java Version: 4.29.3

package org.cyverse.de.protobufs;

public interface AnalysisStatusOrBuilder extends
    // @@protoc_insertion_point(interface_extends:analysis.AnalysisStatus)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <code>.header.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <code>.svcerror.ServiceError error = 2;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();

  /**
   * <code>.analysis.AnalysisSubmission job = 3;</code>
   * @return Whether the job field is set.
   */
  boolean hasJob();
  /**
   * <code>.analysis.AnalysisSubmission job = 3;</code>
   * @return The job.
   */
  org.cyverse.de.protobufs.AnalysisSubmission getJob();
  /**
   * <code>.analysis.AnalysisSubmission job = 3;</code>
   */
  org.cyverse.de.protobufs.AnalysisSubmissionOrBuilder getJobOrBuilder();

  /**
   * <code>int32 version = 4;</code>
   * @return The version.
   */
  int getVersion();

  /**
   * <code>string state = 5;</code>
   * @return The state.
   */
  java.lang.String getState();
  /**
   * <code>string state = 5;</code>
   * @return The bytes for state.
   */
  com.google.protobuf.ByteString
      getStateBytes();

  /**
   * <code>string message = 6;</code>
   * @return The message.
   */
  java.lang.String getMessage();
  /**
   * <code>string message = 6;</code>
   * @return The bytes for message.
   */
  com.google.protobuf.ByteString
      getMessageBytes();

  /**
   * <code>string sent_on = 7 [json_name = "sent_on"];</code>
   * @return The sentOn.
   */
  java.lang.String getSentOn();
  /**
   * <code>string sent_on = 7 [json_name = "sent_on"];</code>
   * @return The bytes for sentOn.
   */
  com.google.protobuf.ByteString
      getSentOnBytes();

  /**
   * <code>string sender = 8;</code>
   * @return The sender.
   */
  java.lang.String getSender();
  /**
   * <code>string sender = 8;</code>
   * @return The bytes for sender.
   */
  com.google.protobuf.ByteString
      getSenderBytes();
}
