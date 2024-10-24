// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_overages.proto

package org.cyverse.de.protobufs;

/**
 * <pre>
 **
 * Returned by handlers in response to overage requests.
 * </pre>
 *
 * Protobuf type {@code qms.OverageResponse}
 */
public final class OverageResponse extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:qms.OverageResponse)
    OverageResponseOrBuilder {
private static final long serialVersionUID = 0L;
  // Use OverageResponse.newBuilder() to construct.
  private OverageResponse(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private OverageResponse() {
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new OverageResponse();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.QMSOveragesProtobufs.internal_static_qms_OverageResponse_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.QMSOveragesProtobufs.internal_static_qms_OverageResponse_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.OverageResponse.class, org.cyverse.de.protobufs.OverageResponse.Builder.class);
  }

  public static final int HEADER_FIELD_NUMBER = 1;
  private org.cyverse.de.protobufs.Header header_;
  /**
   * <pre>
   * The header used for passing telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  @java.lang.Override
  public boolean hasHeader() {
    return header_ != null;
  }
  /**
   * <pre>
   * The header used for passing telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.Header getHeader() {
    return header_ == null ? org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
  }
  /**
   * <pre>
   * The header used for passing telemetry data.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder() {
    return getHeader();
  }

  public static final int ERROR_FIELD_NUMBER = 2;
  private org.cyverse.de.protobufs.ServiceError error_;
  /**
   * <pre>
   * Contains any errors generated by the handler emitting the response.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  @java.lang.Override
  public boolean hasError() {
    return error_ != null;
  }
  /**
   * <pre>
   * Contains any errors generated by the handler emitting the response.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   * @return The error.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ServiceError getError() {
    return error_ == null ? org.cyverse.de.protobufs.ServiceError.getDefaultInstance() : error_;
  }
  /**
   * <pre>
   * Contains any errors generated by the handler emitting the response.
   * </pre>
   *
   * <code>.svcerror.ServiceError error = 2;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder() {
    return getError();
  }

  public static final int OVERAGE_FIELD_NUMBER = 3;
  private org.cyverse.de.protobufs.Overage overage_;
  /**
   * <pre>
   * The overage returned by the handler emitting the response.
   * </pre>
   *
   * <code>.qms.Overage overage = 3;</code>
   * @return Whether the overage field is set.
   */
  @java.lang.Override
  public boolean hasOverage() {
    return overage_ != null;
  }
  /**
   * <pre>
   * The overage returned by the handler emitting the response.
   * </pre>
   *
   * <code>.qms.Overage overage = 3;</code>
   * @return The overage.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.Overage getOverage() {
    return overage_ == null ? org.cyverse.de.protobufs.Overage.getDefaultInstance() : overage_;
  }
  /**
   * <pre>
   * The overage returned by the handler emitting the response.
   * </pre>
   *
   * <code>.qms.Overage overage = 3;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.OverageOrBuilder getOverageOrBuilder() {
    return getOverage();
  }

  private byte memoizedIsInitialized = -1;
  @java.lang.Override
  public final boolean isInitialized() {
    byte isInitialized = memoizedIsInitialized;
    if (isInitialized == 1) return true;
    if (isInitialized == 0) return false;

    memoizedIsInitialized = 1;
    return true;
  }

  @java.lang.Override
  public void writeTo(com.google.protobuf.CodedOutputStream output)
                      throws java.io.IOException {
    if (header_ != null) {
      output.writeMessage(1, getHeader());
    }
    if (error_ != null) {
      output.writeMessage(2, getError());
    }
    if (overage_ != null) {
      output.writeMessage(3, getOverage());
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (header_ != null) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(1, getHeader());
    }
    if (error_ != null) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(2, getError());
    }
    if (overage_ != null) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(3, getOverage());
    }
    size += getUnknownFields().getSerializedSize();
    memoizedSize = size;
    return size;
  }

  @java.lang.Override
  public boolean equals(final java.lang.Object obj) {
    if (obj == this) {
     return true;
    }
    if (!(obj instanceof org.cyverse.de.protobufs.OverageResponse)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.OverageResponse other = (org.cyverse.de.protobufs.OverageResponse) obj;

    if (hasHeader() != other.hasHeader()) return false;
    if (hasHeader()) {
      if (!getHeader()
          .equals(other.getHeader())) return false;
    }
    if (hasError() != other.hasError()) return false;
    if (hasError()) {
      if (!getError()
          .equals(other.getError())) return false;
    }
    if (hasOverage() != other.hasOverage()) return false;
    if (hasOverage()) {
      if (!getOverage()
          .equals(other.getOverage())) return false;
    }
    if (!getUnknownFields().equals(other.getUnknownFields())) return false;
    return true;
  }

  @java.lang.Override
  public int hashCode() {
    if (memoizedHashCode != 0) {
      return memoizedHashCode;
    }
    int hash = 41;
    hash = (19 * hash) + getDescriptor().hashCode();
    if (hasHeader()) {
      hash = (37 * hash) + HEADER_FIELD_NUMBER;
      hash = (53 * hash) + getHeader().hashCode();
    }
    if (hasError()) {
      hash = (37 * hash) + ERROR_FIELD_NUMBER;
      hash = (53 * hash) + getError().hashCode();
    }
    if (hasOverage()) {
      hash = (37 * hash) + OVERAGE_FIELD_NUMBER;
      hash = (53 * hash) + getOverage().hashCode();
    }
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.OverageResponse parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  @java.lang.Override
  public Builder newBuilderForType() { return newBuilder(); }
  public static Builder newBuilder() {
    return DEFAULT_INSTANCE.toBuilder();
  }
  public static Builder newBuilder(org.cyverse.de.protobufs.OverageResponse prototype) {
    return DEFAULT_INSTANCE.toBuilder().mergeFrom(prototype);
  }
  @java.lang.Override
  public Builder toBuilder() {
    return this == DEFAULT_INSTANCE
        ? new Builder() : new Builder().mergeFrom(this);
  }

  @java.lang.Override
  protected Builder newBuilderForType(
      com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
    Builder builder = new Builder(parent);
    return builder;
  }
  /**
   * <pre>
   **
   * Returned by handlers in response to overage requests.
   * </pre>
   *
   * Protobuf type {@code qms.OverageResponse}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:qms.OverageResponse)
      org.cyverse.de.protobufs.OverageResponseOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.QMSOveragesProtobufs.internal_static_qms_OverageResponse_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.QMSOveragesProtobufs.internal_static_qms_OverageResponse_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.OverageResponse.class, org.cyverse.de.protobufs.OverageResponse.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.OverageResponse.newBuilder()
    private Builder() {

    }

    private Builder(
        com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
      super(parent);

    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      if (headerBuilder_ == null) {
        header_ = null;
      } else {
        header_ = null;
        headerBuilder_ = null;
      }
      if (errorBuilder_ == null) {
        error_ = null;
      } else {
        error_ = null;
        errorBuilder_ = null;
      }
      if (overageBuilder_ == null) {
        overage_ = null;
      } else {
        overage_ = null;
        overageBuilder_ = null;
      }
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.QMSOveragesProtobufs.internal_static_qms_OverageResponse_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.OverageResponse getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.OverageResponse.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.OverageResponse build() {
      org.cyverse.de.protobufs.OverageResponse result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.OverageResponse buildPartial() {
      org.cyverse.de.protobufs.OverageResponse result = new org.cyverse.de.protobufs.OverageResponse(this);
      if (headerBuilder_ == null) {
        result.header_ = header_;
      } else {
        result.header_ = headerBuilder_.build();
      }
      if (errorBuilder_ == null) {
        result.error_ = error_;
      } else {
        result.error_ = errorBuilder_.build();
      }
      if (overageBuilder_ == null) {
        result.overage_ = overage_;
      } else {
        result.overage_ = overageBuilder_.build();
      }
      onBuilt();
      return result;
    }

    @java.lang.Override
    public Builder clone() {
      return super.clone();
    }
    @java.lang.Override
    public Builder setField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        java.lang.Object value) {
      return super.setField(field, value);
    }
    @java.lang.Override
    public Builder clearField(
        com.google.protobuf.Descriptors.FieldDescriptor field) {
      return super.clearField(field);
    }
    @java.lang.Override
    public Builder clearOneof(
        com.google.protobuf.Descriptors.OneofDescriptor oneof) {
      return super.clearOneof(oneof);
    }
    @java.lang.Override
    public Builder setRepeatedField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        int index, java.lang.Object value) {
      return super.setRepeatedField(field, index, value);
    }
    @java.lang.Override
    public Builder addRepeatedField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        java.lang.Object value) {
      return super.addRepeatedField(field, value);
    }
    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.OverageResponse) {
        return mergeFrom((org.cyverse.de.protobufs.OverageResponse)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.OverageResponse other) {
      if (other == org.cyverse.de.protobufs.OverageResponse.getDefaultInstance()) return this;
      if (other.hasHeader()) {
        mergeHeader(other.getHeader());
      }
      if (other.hasError()) {
        mergeError(other.getError());
      }
      if (other.hasOverage()) {
        mergeOverage(other.getOverage());
      }
      this.mergeUnknownFields(other.getUnknownFields());
      onChanged();
      return this;
    }

    @java.lang.Override
    public final boolean isInitialized() {
      return true;
    }

    @java.lang.Override
    public Builder mergeFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws java.io.IOException {
      if (extensionRegistry == null) {
        throw new java.lang.NullPointerException();
      }
      try {
        boolean done = false;
        while (!done) {
          int tag = input.readTag();
          switch (tag) {
            case 0:
              done = true;
              break;
            case 10: {
              input.readMessage(
                  getHeaderFieldBuilder().getBuilder(),
                  extensionRegistry);

              break;
            } // case 10
            case 18: {
              input.readMessage(
                  getErrorFieldBuilder().getBuilder(),
                  extensionRegistry);

              break;
            } // case 18
            case 26: {
              input.readMessage(
                  getOverageFieldBuilder().getBuilder(),
                  extensionRegistry);

              break;
            } // case 26
            default: {
              if (!super.parseUnknownField(input, extensionRegistry, tag)) {
                done = true; // was an endgroup tag
              }
              break;
            } // default:
          } // switch (tag)
        } // while (!done)
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        throw e.unwrapIOException();
      } finally {
        onChanged();
      } // finally
      return this;
    }

    private org.cyverse.de.protobufs.Header header_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder> headerBuilder_;
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     * @return Whether the header field is set.
     */
    public boolean hasHeader() {
      return headerBuilder_ != null || header_ != null;
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     * @return The header.
     */
    public org.cyverse.de.protobufs.Header getHeader() {
      if (headerBuilder_ == null) {
        return header_ == null ? org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
      } else {
        return headerBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    public Builder setHeader(org.cyverse.de.protobufs.Header value) {
      if (headerBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        header_ = value;
        onChanged();
      } else {
        headerBuilder_.setMessage(value);
      }

      return this;
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    public Builder setHeader(
        org.cyverse.de.protobufs.Header.Builder builderForValue) {
      if (headerBuilder_ == null) {
        header_ = builderForValue.build();
        onChanged();
      } else {
        headerBuilder_.setMessage(builderForValue.build());
      }

      return this;
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    public Builder mergeHeader(org.cyverse.de.protobufs.Header value) {
      if (headerBuilder_ == null) {
        if (header_ != null) {
          header_ =
            org.cyverse.de.protobufs.Header.newBuilder(header_).mergeFrom(value).buildPartial();
        } else {
          header_ = value;
        }
        onChanged();
      } else {
        headerBuilder_.mergeFrom(value);
      }

      return this;
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    public Builder clearHeader() {
      if (headerBuilder_ == null) {
        header_ = null;
        onChanged();
      } else {
        header_ = null;
        headerBuilder_ = null;
      }

      return this;
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    public org.cyverse.de.protobufs.Header.Builder getHeaderBuilder() {
      
      onChanged();
      return getHeaderFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    public org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder() {
      if (headerBuilder_ != null) {
        return headerBuilder_.getMessageOrBuilder();
      } else {
        return header_ == null ?
            org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
      }
    }
    /**
     * <pre>
     * The header used for passing telemetry data.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder> 
        getHeaderFieldBuilder() {
      if (headerBuilder_ == null) {
        headerBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder>(
                getHeader(),
                getParentForChildren(),
                isClean());
        header_ = null;
      }
      return headerBuilder_;
    }

    private org.cyverse.de.protobufs.ServiceError error_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.ServiceError, org.cyverse.de.protobufs.ServiceError.Builder, org.cyverse.de.protobufs.ServiceErrorOrBuilder> errorBuilder_;
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     * @return Whether the error field is set.
     */
    public boolean hasError() {
      return errorBuilder_ != null || error_ != null;
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     * @return The error.
     */
    public org.cyverse.de.protobufs.ServiceError getError() {
      if (errorBuilder_ == null) {
        return error_ == null ? org.cyverse.de.protobufs.ServiceError.getDefaultInstance() : error_;
      } else {
        return errorBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     */
    public Builder setError(org.cyverse.de.protobufs.ServiceError value) {
      if (errorBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        error_ = value;
        onChanged();
      } else {
        errorBuilder_.setMessage(value);
      }

      return this;
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     */
    public Builder setError(
        org.cyverse.de.protobufs.ServiceError.Builder builderForValue) {
      if (errorBuilder_ == null) {
        error_ = builderForValue.build();
        onChanged();
      } else {
        errorBuilder_.setMessage(builderForValue.build());
      }

      return this;
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     */
    public Builder mergeError(org.cyverse.de.protobufs.ServiceError value) {
      if (errorBuilder_ == null) {
        if (error_ != null) {
          error_ =
            org.cyverse.de.protobufs.ServiceError.newBuilder(error_).mergeFrom(value).buildPartial();
        } else {
          error_ = value;
        }
        onChanged();
      } else {
        errorBuilder_.mergeFrom(value);
      }

      return this;
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     */
    public Builder clearError() {
      if (errorBuilder_ == null) {
        error_ = null;
        onChanged();
      } else {
        error_ = null;
        errorBuilder_ = null;
      }

      return this;
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     */
    public org.cyverse.de.protobufs.ServiceError.Builder getErrorBuilder() {
      
      onChanged();
      return getErrorFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     */
    public org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder() {
      if (errorBuilder_ != null) {
        return errorBuilder_.getMessageOrBuilder();
      } else {
        return error_ == null ?
            org.cyverse.de.protobufs.ServiceError.getDefaultInstance() : error_;
      }
    }
    /**
     * <pre>
     * Contains any errors generated by the handler emitting the response.
     * </pre>
     *
     * <code>.svcerror.ServiceError error = 2;</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.ServiceError, org.cyverse.de.protobufs.ServiceError.Builder, org.cyverse.de.protobufs.ServiceErrorOrBuilder> 
        getErrorFieldBuilder() {
      if (errorBuilder_ == null) {
        errorBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            org.cyverse.de.protobufs.ServiceError, org.cyverse.de.protobufs.ServiceError.Builder, org.cyverse.de.protobufs.ServiceErrorOrBuilder>(
                getError(),
                getParentForChildren(),
                isClean());
        error_ = null;
      }
      return errorBuilder_;
    }

    private org.cyverse.de.protobufs.Overage overage_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Overage, org.cyverse.de.protobufs.Overage.Builder, org.cyverse.de.protobufs.OverageOrBuilder> overageBuilder_;
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     * @return Whether the overage field is set.
     */
    public boolean hasOverage() {
      return overageBuilder_ != null || overage_ != null;
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     * @return The overage.
     */
    public org.cyverse.de.protobufs.Overage getOverage() {
      if (overageBuilder_ == null) {
        return overage_ == null ? org.cyverse.de.protobufs.Overage.getDefaultInstance() : overage_;
      } else {
        return overageBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     */
    public Builder setOverage(org.cyverse.de.protobufs.Overage value) {
      if (overageBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        overage_ = value;
        onChanged();
      } else {
        overageBuilder_.setMessage(value);
      }

      return this;
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     */
    public Builder setOverage(
        org.cyverse.de.protobufs.Overage.Builder builderForValue) {
      if (overageBuilder_ == null) {
        overage_ = builderForValue.build();
        onChanged();
      } else {
        overageBuilder_.setMessage(builderForValue.build());
      }

      return this;
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     */
    public Builder mergeOverage(org.cyverse.de.protobufs.Overage value) {
      if (overageBuilder_ == null) {
        if (overage_ != null) {
          overage_ =
            org.cyverse.de.protobufs.Overage.newBuilder(overage_).mergeFrom(value).buildPartial();
        } else {
          overage_ = value;
        }
        onChanged();
      } else {
        overageBuilder_.mergeFrom(value);
      }

      return this;
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     */
    public Builder clearOverage() {
      if (overageBuilder_ == null) {
        overage_ = null;
        onChanged();
      } else {
        overage_ = null;
        overageBuilder_ = null;
      }

      return this;
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     */
    public org.cyverse.de.protobufs.Overage.Builder getOverageBuilder() {
      
      onChanged();
      return getOverageFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     */
    public org.cyverse.de.protobufs.OverageOrBuilder getOverageOrBuilder() {
      if (overageBuilder_ != null) {
        return overageBuilder_.getMessageOrBuilder();
      } else {
        return overage_ == null ?
            org.cyverse.de.protobufs.Overage.getDefaultInstance() : overage_;
      }
    }
    /**
     * <pre>
     * The overage returned by the handler emitting the response.
     * </pre>
     *
     * <code>.qms.Overage overage = 3;</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Overage, org.cyverse.de.protobufs.Overage.Builder, org.cyverse.de.protobufs.OverageOrBuilder> 
        getOverageFieldBuilder() {
      if (overageBuilder_ == null) {
        overageBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            org.cyverse.de.protobufs.Overage, org.cyverse.de.protobufs.Overage.Builder, org.cyverse.de.protobufs.OverageOrBuilder>(
                getOverage(),
                getParentForChildren(),
                isClean());
        overage_ = null;
      }
      return overageBuilder_;
    }
    @java.lang.Override
    public final Builder setUnknownFields(
        final com.google.protobuf.UnknownFieldSet unknownFields) {
      return super.setUnknownFields(unknownFields);
    }

    @java.lang.Override
    public final Builder mergeUnknownFields(
        final com.google.protobuf.UnknownFieldSet unknownFields) {
      return super.mergeUnknownFields(unknownFields);
    }


    // @@protoc_insertion_point(builder_scope:qms.OverageResponse)
  }

  // @@protoc_insertion_point(class_scope:qms.OverageResponse)
  private static final org.cyverse.de.protobufs.OverageResponse DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.OverageResponse();
  }

  public static org.cyverse.de.protobufs.OverageResponse getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<OverageResponse>
      PARSER = new com.google.protobuf.AbstractParser<OverageResponse>() {
    @java.lang.Override
    public OverageResponse parsePartialFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws com.google.protobuf.InvalidProtocolBufferException {
      Builder builder = newBuilder();
      try {
        builder.mergeFrom(input, extensionRegistry);
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        throw e.setUnfinishedMessage(builder.buildPartial());
      } catch (com.google.protobuf.UninitializedMessageException e) {
        throw e.asInvalidProtocolBufferException().setUnfinishedMessage(builder.buildPartial());
      } catch (java.io.IOException e) {
        throw new com.google.protobuf.InvalidProtocolBufferException(e)
            .setUnfinishedMessage(builder.buildPartial());
      }
      return builder.buildPartial();
    }
  };

  public static com.google.protobuf.Parser<OverageResponse> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<OverageResponse> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.OverageResponse getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

