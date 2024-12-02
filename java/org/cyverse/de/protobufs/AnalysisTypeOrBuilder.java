// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: analysis_record.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface AnalysisTypeOrBuilder extends
    // @@protoc_insertion_point(interface_extends:analysis.AnalysisType)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The UUID for the analysis type.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The id.
   */
  java.lang.String getId();
  /**
   * <pre>
   * The UUID for the analysis type.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The bytes for id.
   */
  com.google.protobuf.ByteString
      getIdBytes();

  /**
   * <pre>
   * The name of the analysis type.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The name.
   */
  java.lang.String getName();
  /**
   * <pre>
   * The name of the analysis type.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The bytes for name.
   */
  com.google.protobuf.ByteString
      getNameBytes();

  /**
   * <pre>
   * The kind of system the analysis should run on.
   * </pre>
   *
   * <code>string system_id = 3 [json_name = "system_id"];</code>
   * @return The systemId.
   */
  java.lang.String getSystemId();
  /**
   * <pre>
   * The kind of system the analysis should run on.
   * </pre>
   *
   * <code>string system_id = 3 [json_name = "system_id"];</code>
   * @return The bytes for systemId.
   */
  com.google.protobuf.ByteString
      getSystemIdBytes();
}
