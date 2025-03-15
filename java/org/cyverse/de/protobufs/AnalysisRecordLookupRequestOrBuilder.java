// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: analysis_requests.proto
// Protobuf Java Version: 4.29.3

package org.cyverse.de.protobufs;

public interface AnalysisRecordLookupRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:analysis.AnalysisRecordLookupRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>string analysis_id = 1;</code>
   * @return Whether the analysisId field is set.
   */
  boolean hasAnalysisId();
  /**
   * <code>string analysis_id = 1;</code>
   * @return The analysisId.
   */
  java.lang.String getAnalysisId();
  /**
   * <code>string analysis_id = 1;</code>
   * @return The bytes for analysisId.
   */
  com.google.protobuf.ByteString
      getAnalysisIdBytes();

  /**
   * <code>string external_id = 2;</code>
   * @return Whether the externalId field is set.
   */
  boolean hasExternalId();
  /**
   * <code>string external_id = 2;</code>
   * @return The externalId.
   */
  java.lang.String getExternalId();
  /**
   * <code>string external_id = 2;</code>
   * @return The bytes for externalId.
   */
  com.google.protobuf.ByteString
      getExternalIdBytes();

  /**
   * <code>string user_id = 3;</code>
   * @return Whether the userId field is set.
   */
  boolean hasUserId();
  /**
   * <code>string user_id = 3;</code>
   * @return The userId.
   */
  java.lang.String getUserId();
  /**
   * <code>string user_id = 3;</code>
   * @return The bytes for userId.
   */
  com.google.protobuf.ByteString
      getUserIdBytes();

  /**
   * <code>string username = 4;</code>
   * @return Whether the username field is set.
   */
  boolean hasUsername();
  /**
   * <code>string username = 4;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <code>string username = 4;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();

  /**
   * <code>.header.Header header = 5;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <code>.header.Header header = 5;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <code>.header.Header header = 5;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <code>string requesting_user = 6;</code>
   * @return The requestingUser.
   */
  java.lang.String getRequestingUser();
  /**
   * <code>string requesting_user = 6;</code>
   * @return The bytes for requestingUser.
   */
  com.google.protobuf.ByteString
      getRequestingUserBytes();

  org.cyverse.de.protobufs.AnalysisRecordLookupRequest.LookupIdsCase getLookupIdsCase();
}
