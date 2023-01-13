// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_requests.proto

package org.cyverse.de.protobufs;

/**
 * Protobuf type {@code NoParamsRequest}
 */
public final class NoParamsRequest extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:NoParamsRequest)
    NoParamsRequestOrBuilder {
private static final long serialVersionUID = 0L;
  // Use NoParamsRequest.newBuilder() to construct.
  private NoParamsRequest(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private NoParamsRequest() {
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new NoParamsRequest();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.QMSRequestProtobufs.internal_static_NoParamsRequest_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.QMSRequestProtobufs.internal_static_NoParamsRequest_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.NoParamsRequest.class, org.cyverse.de.protobufs.NoParamsRequest.Builder.class);
  }

  public static final int HEADER_FIELD_NUMBER = 1;
  private org.cyverse.de.protobufs.Header header_;
  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  @java.lang.Override
  public boolean hasHeader() {
    return header_ != null;
  }
  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.Header header = 1;</code>
   * @return The header.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.Header getHeader() {
    return header_ == null ? org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
  }
  /**
   * <pre>
   * Contains telemetry information
   * </pre>
   *
   * <code>.Header header = 1;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder() {
    return header_ == null ? org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
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
    size += getUnknownFields().getSerializedSize();
    memoizedSize = size;
    return size;
  }

  @java.lang.Override
  public boolean equals(final java.lang.Object obj) {
    if (obj == this) {
     return true;
    }
    if (!(obj instanceof org.cyverse.de.protobufs.NoParamsRequest)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.NoParamsRequest other = (org.cyverse.de.protobufs.NoParamsRequest) obj;

    if (hasHeader() != other.hasHeader()) return false;
    if (hasHeader()) {
      if (!getHeader()
          .equals(other.getHeader())) return false;
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
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.NoParamsRequest parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.NoParamsRequest prototype) {
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
   * Protobuf type {@code NoParamsRequest}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:NoParamsRequest)
      org.cyverse.de.protobufs.NoParamsRequestOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.QMSRequestProtobufs.internal_static_NoParamsRequest_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.QMSRequestProtobufs.internal_static_NoParamsRequest_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.NoParamsRequest.class, org.cyverse.de.protobufs.NoParamsRequest.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.NoParamsRequest.newBuilder()
    private Builder() {

    }

    private Builder(
        com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
      super(parent);

    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      bitField0_ = 0;
      header_ = null;
      if (headerBuilder_ != null) {
        headerBuilder_.dispose();
        headerBuilder_ = null;
      }
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.QMSRequestProtobufs.internal_static_NoParamsRequest_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.NoParamsRequest getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.NoParamsRequest.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.NoParamsRequest build() {
      org.cyverse.de.protobufs.NoParamsRequest result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.NoParamsRequest buildPartial() {
      org.cyverse.de.protobufs.NoParamsRequest result = new org.cyverse.de.protobufs.NoParamsRequest(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.NoParamsRequest result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.header_ = headerBuilder_ == null
            ? header_
            : headerBuilder_.build();
      }
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
      if (other instanceof org.cyverse.de.protobufs.NoParamsRequest) {
        return mergeFrom((org.cyverse.de.protobufs.NoParamsRequest)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.NoParamsRequest other) {
      if (other == org.cyverse.de.protobufs.NoParamsRequest.getDefaultInstance()) return this;
      if (other.hasHeader()) {
        mergeHeader(other.getHeader());
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
              bitField0_ |= 0x00000001;
              break;
            } // case 10
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
    private int bitField0_;

    private org.cyverse.de.protobufs.Header header_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder> headerBuilder_;
    /**
     * <pre>
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
     * @return Whether the header field is set.
     */
    public boolean hasHeader() {
      return ((bitField0_ & 0x00000001) != 0);
    }
    /**
     * <pre>
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
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
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
     */
    public Builder setHeader(org.cyverse.de.protobufs.Header value) {
      if (headerBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        header_ = value;
      } else {
        headerBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
     */
    public Builder setHeader(
        org.cyverse.de.protobufs.Header.Builder builderForValue) {
      if (headerBuilder_ == null) {
        header_ = builderForValue.build();
      } else {
        headerBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
     */
    public Builder mergeHeader(org.cyverse.de.protobufs.Header value) {
      if (headerBuilder_ == null) {
        if (((bitField0_ & 0x00000001) != 0) &&
          header_ != null &&
          header_ != org.cyverse.de.protobufs.Header.getDefaultInstance()) {
          getHeaderBuilder().mergeFrom(value);
        } else {
          header_ = value;
        }
      } else {
        headerBuilder_.mergeFrom(value);
      }
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
     */
    public Builder clearHeader() {
      bitField0_ = (bitField0_ & ~0x00000001);
      header_ = null;
      if (headerBuilder_ != null) {
        headerBuilder_.dispose();
        headerBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
     */
    public org.cyverse.de.protobufs.Header.Builder getHeaderBuilder() {
      bitField0_ |= 0x00000001;
      onChanged();
      return getHeaderFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
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
     * Contains telemetry information
     * </pre>
     *
     * <code>.Header header = 1;</code>
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


    // @@protoc_insertion_point(builder_scope:NoParamsRequest)
  }

  // @@protoc_insertion_point(class_scope:NoParamsRequest)
  private static final org.cyverse.de.protobufs.NoParamsRequest DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.NoParamsRequest();
  }

  public static org.cyverse.de.protobufs.NoParamsRequest getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<NoParamsRequest>
      PARSER = new com.google.protobuf.AbstractParser<NoParamsRequest>() {
    @java.lang.Override
    public NoParamsRequest parsePartialFrom(
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

  public static com.google.protobuf.Parser<NoParamsRequest> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<NoParamsRequest> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.NoParamsRequest getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

