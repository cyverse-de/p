// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_addons.proto

package org.cyverse.de.protobufs;

/**
 * <pre>
 **
 * Contains the subscription add-on returned by the request handler.
 * </pre>
 *
 * Protobuf type {@code SubscriptionAddonResponse}
 */
public final class SubscriptionAddonResponse extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:SubscriptionAddonResponse)
    SubscriptionAddonResponseOrBuilder {
private static final long serialVersionUID = 0L;
  // Use SubscriptionAddonResponse.newBuilder() to construct.
  private SubscriptionAddonResponse(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private SubscriptionAddonResponse() {
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new SubscriptionAddonResponse();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_SubscriptionAddonResponse_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_SubscriptionAddonResponse_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.SubscriptionAddonResponse.class, org.cyverse.de.protobufs.SubscriptionAddonResponse.Builder.class);
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

  public static final int ERROR_FIELD_NUMBER = 2;
  private org.cyverse.de.protobufs.ServiceError error_;
  /**
   * <pre>
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  @java.lang.Override
  public boolean hasError() {
    return error_ != null;
  }
  /**
   * <pre>
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   * @return The error.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ServiceError getError() {
    return error_ == null ? org.cyverse.de.protobufs.ServiceError.getDefaultInstance() : error_;
  }
  /**
   * <pre>
   * Error information generated by the request handler.
   * </pre>
   *
   * <code>.ServiceError error = 2;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder() {
    return error_ == null ? org.cyverse.de.protobufs.ServiceError.getDefaultInstance() : error_;
  }

  public static final int SUBSCRIPTION_ADDON_FIELD_NUMBER = 3;
  private org.cyverse.de.protobufs.SubscriptionAddon subscriptionAddon_;
  /**
   * <pre>
   * The subscription add-on returned by the request handler.
   * </pre>
   *
   * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
   * @return Whether the subscriptionAddon field is set.
   */
  @java.lang.Override
  public boolean hasSubscriptionAddon() {
    return subscriptionAddon_ != null;
  }
  /**
   * <pre>
   * The subscription add-on returned by the request handler.
   * </pre>
   *
   * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
   * @return The subscriptionAddon.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.SubscriptionAddon getSubscriptionAddon() {
    return subscriptionAddon_ == null ? org.cyverse.de.protobufs.SubscriptionAddon.getDefaultInstance() : subscriptionAddon_;
  }
  /**
   * <pre>
   * The subscription add-on returned by the request handler.
   * </pre>
   *
   * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.SubscriptionAddonOrBuilder getSubscriptionAddonOrBuilder() {
    return subscriptionAddon_ == null ? org.cyverse.de.protobufs.SubscriptionAddon.getDefaultInstance() : subscriptionAddon_;
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
    if (subscriptionAddon_ != null) {
      output.writeMessage(3, getSubscriptionAddon());
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
    if (subscriptionAddon_ != null) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(3, getSubscriptionAddon());
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
    if (!(obj instanceof org.cyverse.de.protobufs.SubscriptionAddonResponse)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.SubscriptionAddonResponse other = (org.cyverse.de.protobufs.SubscriptionAddonResponse) obj;

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
    if (hasSubscriptionAddon() != other.hasSubscriptionAddon()) return false;
    if (hasSubscriptionAddon()) {
      if (!getSubscriptionAddon()
          .equals(other.getSubscriptionAddon())) return false;
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
    if (hasSubscriptionAddon()) {
      hash = (37 * hash) + SUBSCRIPTION_ADDON_FIELD_NUMBER;
      hash = (53 * hash) + getSubscriptionAddon().hashCode();
    }
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.SubscriptionAddonResponse parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.SubscriptionAddonResponse prototype) {
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
   * Contains the subscription add-on returned by the request handler.
   * </pre>
   *
   * Protobuf type {@code SubscriptionAddonResponse}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:SubscriptionAddonResponse)
      org.cyverse.de.protobufs.SubscriptionAddonResponseOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_SubscriptionAddonResponse_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_SubscriptionAddonResponse_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.SubscriptionAddonResponse.class, org.cyverse.de.protobufs.SubscriptionAddonResponse.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.SubscriptionAddonResponse.newBuilder()
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
      error_ = null;
      if (errorBuilder_ != null) {
        errorBuilder_.dispose();
        errorBuilder_ = null;
      }
      subscriptionAddon_ = null;
      if (subscriptionAddonBuilder_ != null) {
        subscriptionAddonBuilder_.dispose();
        subscriptionAddonBuilder_ = null;
      }
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_SubscriptionAddonResponse_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.SubscriptionAddonResponse getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.SubscriptionAddonResponse.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.SubscriptionAddonResponse build() {
      org.cyverse.de.protobufs.SubscriptionAddonResponse result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.SubscriptionAddonResponse buildPartial() {
      org.cyverse.de.protobufs.SubscriptionAddonResponse result = new org.cyverse.de.protobufs.SubscriptionAddonResponse(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.SubscriptionAddonResponse result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.header_ = headerBuilder_ == null
            ? header_
            : headerBuilder_.build();
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.error_ = errorBuilder_ == null
            ? error_
            : errorBuilder_.build();
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.subscriptionAddon_ = subscriptionAddonBuilder_ == null
            ? subscriptionAddon_
            : subscriptionAddonBuilder_.build();
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
      if (other instanceof org.cyverse.de.protobufs.SubscriptionAddonResponse) {
        return mergeFrom((org.cyverse.de.protobufs.SubscriptionAddonResponse)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.SubscriptionAddonResponse other) {
      if (other == org.cyverse.de.protobufs.SubscriptionAddonResponse.getDefaultInstance()) return this;
      if (other.hasHeader()) {
        mergeHeader(other.getHeader());
      }
      if (other.hasError()) {
        mergeError(other.getError());
      }
      if (other.hasSubscriptionAddon()) {
        mergeSubscriptionAddon(other.getSubscriptionAddon());
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
            case 18: {
              input.readMessage(
                  getErrorFieldBuilder().getBuilder(),
                  extensionRegistry);
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 26: {
              input.readMessage(
                  getSubscriptionAddonFieldBuilder().getBuilder(),
                  extensionRegistry);
              bitField0_ |= 0x00000004;
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

    private org.cyverse.de.protobufs.ServiceError error_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.ServiceError, org.cyverse.de.protobufs.ServiceError.Builder, org.cyverse.de.protobufs.ServiceErrorOrBuilder> errorBuilder_;
    /**
     * <pre>
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
     * @return Whether the error field is set.
     */
    public boolean hasError() {
      return ((bitField0_ & 0x00000002) != 0);
    }
    /**
     * <pre>
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
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
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
     */
    public Builder setError(org.cyverse.de.protobufs.ServiceError value) {
      if (errorBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        error_ = value;
      } else {
        errorBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
     */
    public Builder setError(
        org.cyverse.de.protobufs.ServiceError.Builder builderForValue) {
      if (errorBuilder_ == null) {
        error_ = builderForValue.build();
      } else {
        errorBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
     */
    public Builder mergeError(org.cyverse.de.protobufs.ServiceError value) {
      if (errorBuilder_ == null) {
        if (((bitField0_ & 0x00000002) != 0) &&
          error_ != null &&
          error_ != org.cyverse.de.protobufs.ServiceError.getDefaultInstance()) {
          getErrorBuilder().mergeFrom(value);
        } else {
          error_ = value;
        }
      } else {
        errorBuilder_.mergeFrom(value);
      }
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
     */
    public Builder clearError() {
      bitField0_ = (bitField0_ & ~0x00000002);
      error_ = null;
      if (errorBuilder_ != null) {
        errorBuilder_.dispose();
        errorBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
     */
    public org.cyverse.de.protobufs.ServiceError.Builder getErrorBuilder() {
      bitField0_ |= 0x00000002;
      onChanged();
      return getErrorFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
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
     * Error information generated by the request handler.
     * </pre>
     *
     * <code>.ServiceError error = 2;</code>
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

    private org.cyverse.de.protobufs.SubscriptionAddon subscriptionAddon_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.SubscriptionAddon, org.cyverse.de.protobufs.SubscriptionAddon.Builder, org.cyverse.de.protobufs.SubscriptionAddonOrBuilder> subscriptionAddonBuilder_;
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     * @return Whether the subscriptionAddon field is set.
     */
    public boolean hasSubscriptionAddon() {
      return ((bitField0_ & 0x00000004) != 0);
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     * @return The subscriptionAddon.
     */
    public org.cyverse.de.protobufs.SubscriptionAddon getSubscriptionAddon() {
      if (subscriptionAddonBuilder_ == null) {
        return subscriptionAddon_ == null ? org.cyverse.de.protobufs.SubscriptionAddon.getDefaultInstance() : subscriptionAddon_;
      } else {
        return subscriptionAddonBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     */
    public Builder setSubscriptionAddon(org.cyverse.de.protobufs.SubscriptionAddon value) {
      if (subscriptionAddonBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        subscriptionAddon_ = value;
      } else {
        subscriptionAddonBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     */
    public Builder setSubscriptionAddon(
        org.cyverse.de.protobufs.SubscriptionAddon.Builder builderForValue) {
      if (subscriptionAddonBuilder_ == null) {
        subscriptionAddon_ = builderForValue.build();
      } else {
        subscriptionAddonBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     */
    public Builder mergeSubscriptionAddon(org.cyverse.de.protobufs.SubscriptionAddon value) {
      if (subscriptionAddonBuilder_ == null) {
        if (((bitField0_ & 0x00000004) != 0) &&
          subscriptionAddon_ != null &&
          subscriptionAddon_ != org.cyverse.de.protobufs.SubscriptionAddon.getDefaultInstance()) {
          getSubscriptionAddonBuilder().mergeFrom(value);
        } else {
          subscriptionAddon_ = value;
        }
      } else {
        subscriptionAddonBuilder_.mergeFrom(value);
      }
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     */
    public Builder clearSubscriptionAddon() {
      bitField0_ = (bitField0_ & ~0x00000004);
      subscriptionAddon_ = null;
      if (subscriptionAddonBuilder_ != null) {
        subscriptionAddonBuilder_.dispose();
        subscriptionAddonBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     */
    public org.cyverse.de.protobufs.SubscriptionAddon.Builder getSubscriptionAddonBuilder() {
      bitField0_ |= 0x00000004;
      onChanged();
      return getSubscriptionAddonFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     */
    public org.cyverse.de.protobufs.SubscriptionAddonOrBuilder getSubscriptionAddonOrBuilder() {
      if (subscriptionAddonBuilder_ != null) {
        return subscriptionAddonBuilder_.getMessageOrBuilder();
      } else {
        return subscriptionAddon_ == null ?
            org.cyverse.de.protobufs.SubscriptionAddon.getDefaultInstance() : subscriptionAddon_;
      }
    }
    /**
     * <pre>
     * The subscription add-on returned by the request handler.
     * </pre>
     *
     * <code>.SubscriptionAddon subscription_addon = 3 [json_name = "subscription_addon"];</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.SubscriptionAddon, org.cyverse.de.protobufs.SubscriptionAddon.Builder, org.cyverse.de.protobufs.SubscriptionAddonOrBuilder> 
        getSubscriptionAddonFieldBuilder() {
      if (subscriptionAddonBuilder_ == null) {
        subscriptionAddonBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            org.cyverse.de.protobufs.SubscriptionAddon, org.cyverse.de.protobufs.SubscriptionAddon.Builder, org.cyverse.de.protobufs.SubscriptionAddonOrBuilder>(
                getSubscriptionAddon(),
                getParentForChildren(),
                isClean());
        subscriptionAddon_ = null;
      }
      return subscriptionAddonBuilder_;
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


    // @@protoc_insertion_point(builder_scope:SubscriptionAddonResponse)
  }

  // @@protoc_insertion_point(class_scope:SubscriptionAddonResponse)
  private static final org.cyverse.de.protobufs.SubscriptionAddonResponse DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.SubscriptionAddonResponse();
  }

  public static org.cyverse.de.protobufs.SubscriptionAddonResponse getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<SubscriptionAddonResponse>
      PARSER = new com.google.protobuf.AbstractParser<SubscriptionAddonResponse>() {
    @java.lang.Override
    public SubscriptionAddonResponse parsePartialFrom(
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

  public static com.google.protobuf.Parser<SubscriptionAddonResponse> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<SubscriptionAddonResponse> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.SubscriptionAddonResponse getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

