// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: analysis_requests.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface AnalysisRecordListOrBuilder extends
    // @@protoc_insertion_point(interface_extends:analysis.AnalysisRecordList)
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
   * <code>repeated .analysis.Analysis analyses = 2;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Analysis> 
      getAnalysesList();
  /**
   * <code>repeated .analysis.Analysis analyses = 2;</code>
   */
  org.cyverse.de.protobufs.Analysis getAnalyses(int index);
  /**
   * <code>repeated .analysis.Analysis analyses = 2;</code>
   */
  int getAnalysesCount();
  /**
   * <code>repeated .analysis.Analysis analyses = 2;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.AnalysisOrBuilder> 
      getAnalysesOrBuilderList();
  /**
   * <code>repeated .analysis.Analysis analyses = 2;</code>
   */
  org.cyverse.de.protobufs.AnalysisOrBuilder getAnalysesOrBuilder(
      int index);

  /**
   * <code>.svcerror.ServiceError error = 7;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <code>.svcerror.ServiceError error = 7;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <code>.svcerror.ServiceError error = 7;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();
}
