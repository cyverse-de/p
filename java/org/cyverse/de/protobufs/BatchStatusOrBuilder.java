// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: analysis_record.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface BatchStatusOrBuilder extends
    // @@protoc_insertion_point(interface_extends:analysis.BatchStatus)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>int64 total = 1;</code>
   * @return The total.
   */
  long getTotal();

  /**
   * <code>int64 completed = 2;</code>
   * @return The completed.
   */
  long getCompleted();

  /**
   * <code>int64 running = 3;</code>
   * @return The running.
   */
  long getRunning();

  /**
   * <code>int64 submitted = 4;</code>
   * @return The submitted.
   */
  long getSubmitted();
}
