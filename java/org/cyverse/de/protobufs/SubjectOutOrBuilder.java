// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: groups.proto
// Protobuf Java Version: 4.31.1

package org.cyverse.de.protobufs;

@com.google.protobuf.Generated
public interface SubjectOutOrBuilder extends
    // @@protoc_insertion_point(interface_extends:groups.SubjectOut)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The internal subject id.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The id.
   */
  java.lang.String getId();
  /**
   * <pre>
   * The internal subject id.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The bytes for id.
   */
  com.google.protobuf.ByteString
      getIdBytes();

  /**
   * <pre>
   * The external subject id.
   * </pre>
   *
   * <code>string subject_id = 2 [json_name = "subject_id"];</code>
   * @return The subjectId.
   */
  java.lang.String getSubjectId();
  /**
   * <pre>
   * The external subject id.
   * </pre>
   *
   * <code>string subject_id = 2 [json_name = "subject_id"];</code>
   * @return The bytes for subjectId.
   */
  com.google.protobuf.ByteString
      getSubjectIdBytes();

  /**
   * <pre>
   * The subject type.
   * </pre>
   *
   * <code>string subject_type = 3 [json_name = "subject_type"];</code>
   * @return The subjectType.
   */
  java.lang.String getSubjectType();
  /**
   * <pre>
   * The subject type.
   * </pre>
   *
   * <code>string subject_type = 3 [json_name = "subject_type"];</code>
   * @return The bytes for subjectType.
   */
  com.google.protobuf.ByteString
      getSubjectTypeBytes();

  /**
   * <pre>
   * The subject source ID.
   * </pre>
   *
   * <code>string subject_source_id = 4 [json_name = "subject_source_id"];</code>
   * @return The subjectSourceId.
   */
  java.lang.String getSubjectSourceId();
  /**
   * <pre>
   * The subject source ID.
   * </pre>
   *
   * <code>string subject_source_id = 4 [json_name = "subject_source_id"];</code>
   * @return The bytes for subjectSourceId.
   */
  com.google.protobuf.ByteString
      getSubjectSourceIdBytes();
}
