// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: user_requests.proto

package org.cyverse.de.protobufs;

public interface InternalPaginationContinuationTokenOrBuilder extends
    // @@protoc_insertion_point(interface_extends:user_requests.InternalPaginationContinuationToken)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * offset into the result set
   * </pre>
   *
   * <code>optional int32 offset = 1 [(.buf.validate.field) = { ... }</code>
   * @return Whether the offset field is set.
   */
  boolean hasOffset();
  /**
   * <pre>
   * offset into the result set
   * </pre>
   *
   * <code>optional int32 offset = 1 [(.buf.validate.field) = { ... }</code>
   * @return The offset.
   */
  int getOffset();

  /**
   * <pre>
   * page number
   * </pre>
   *
   * <code>optional int32 number = 2 [(.buf.validate.field) = { ... }</code>
   * @return Whether the number field is set.
   */
  boolean hasNumber();
  /**
   * <pre>
   * page number
   * </pre>
   *
   * <code>optional int32 number = 2 [(.buf.validate.field) = { ... }</code>
   * @return The number.
   */
  int getNumber();

  /**
   * <pre>
   * number of results returned in a page.
   * </pre>
   *
   * <code>optional int32 size = 3 [(.buf.validate.field) = { ... }</code>
   * @return Whether the size field is set.
   */
  boolean hasSize();
  /**
   * <pre>
   * number of results returned in a page.
   * </pre>
   *
   * <code>optional int32 size = 3 [(.buf.validate.field) = { ... }</code>
   * @return The size.
   */
  int getSize();
}
