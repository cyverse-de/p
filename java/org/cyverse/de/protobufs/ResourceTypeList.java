// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_resource_types.proto

package org.cyverse.de.protobufs;

/**
 * Protobuf type {@code ResourceTypeList}
 */
public final class ResourceTypeList extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:ResourceTypeList)
    ResourceTypeListOrBuilder {
private static final long serialVersionUID = 0L;
  // Use ResourceTypeList.newBuilder() to construct.
  private ResourceTypeList(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private ResourceTypeList() {
    resourceTypes_ = java.util.Collections.emptyList();
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new ResourceTypeList();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  private ResourceTypeList(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    this();
    if (extensionRegistry == null) {
      throw new java.lang.NullPointerException();
    }
    int mutable_bitField0_ = 0;
    com.google.protobuf.UnknownFieldSet.Builder unknownFields =
        com.google.protobuf.UnknownFieldSet.newBuilder();
    try {
      boolean done = false;
      while (!done) {
        int tag = input.readTag();
        switch (tag) {
          case 0:
            done = true;
            break;
          case 10: {
            org.cyverse.de.protobufs.Header.Builder subBuilder = null;
            if (header_ != null) {
              subBuilder = header_.toBuilder();
            }
            header_ = input.readMessage(org.cyverse.de.protobufs.Header.parser(), extensionRegistry);
            if (subBuilder != null) {
              subBuilder.mergeFrom(header_);
              header_ = subBuilder.buildPartial();
            }

            break;
          }
          case 18: {
            org.cyverse.de.protobufs.ServiceError.Builder subBuilder = null;
            if (error_ != null) {
              subBuilder = error_.toBuilder();
            }
            error_ = input.readMessage(org.cyverse.de.protobufs.ServiceError.parser(), extensionRegistry);
            if (subBuilder != null) {
              subBuilder.mergeFrom(error_);
              error_ = subBuilder.buildPartial();
            }

            break;
          }
          case 26: {
            if (!((mutable_bitField0_ & 0x00000001) != 0)) {
              resourceTypes_ = new java.util.ArrayList<org.cyverse.de.protobufs.ResourceType>();
              mutable_bitField0_ |= 0x00000001;
            }
            resourceTypes_.add(
                input.readMessage(org.cyverse.de.protobufs.ResourceType.parser(), extensionRegistry));
            break;
          }
          default: {
            if (!parseUnknownField(
                input, unknownFields, extensionRegistry, tag)) {
              done = true;
            }
            break;
          }
        }
      }
    } catch (com.google.protobuf.InvalidProtocolBufferException e) {
      throw e.setUnfinishedMessage(this);
    } catch (java.io.IOException e) {
      throw new com.google.protobuf.InvalidProtocolBufferException(
          e).setUnfinishedMessage(this);
    } finally {
      if (((mutable_bitField0_ & 0x00000001) != 0)) {
        resourceTypes_ = java.util.Collections.unmodifiableList(resourceTypes_);
      }
      this.unknownFields = unknownFields.build();
      makeExtensionsImmutable();
    }
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.QMSResourceTypeProtobufs.internal_static_ResourceTypeList_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.QMSResourceTypeProtobufs.internal_static_ResourceTypeList_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.ResourceTypeList.class, org.cyverse.de.protobufs.ResourceTypeList.Builder.class);
  }

  public static final int HEADER_FIELD_NUMBER = 1;
  private org.cyverse.de.protobufs.Header header_;
  /**
   * <code>.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  @java.lang.Override
  public boolean hasHeader() {
    return header_ != null;
  }
  /**
   * <code>.Header header = 1;</code>
   * @return The header.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.Header getHeader() {
    return header_ == null ? org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
  }
  /**
   * <code>.Header header = 1;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder() {
    return getHeader();
  }

  public static final int ERROR_FIELD_NUMBER = 2;
  private org.cyverse.de.protobufs.ServiceError error_;
  /**
   * <code>.ServiceError error = 2;</code>
   * @return Whether the error field is set.
   */
  @java.lang.Override
  public boolean hasError() {
    return error_ != null;
  }
  /**
   * <code>.ServiceError error = 2;</code>
   * @return The error.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ServiceError getError() {
    return error_ == null ? org.cyverse.de.protobufs.ServiceError.getDefaultInstance() : error_;
  }
  /**
   * <code>.ServiceError error = 2;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder() {
    return getError();
  }

  public static final int RESOURCE_TYPES_FIELD_NUMBER = 3;
  private java.util.List<org.cyverse.de.protobufs.ResourceType> resourceTypes_;
  /**
   * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
   */
  @java.lang.Override
  public java.util.List<org.cyverse.de.protobufs.ResourceType> getResourceTypesList() {
    return resourceTypes_;
  }
  /**
   * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
   */
  @java.lang.Override
  public java.util.List<? extends org.cyverse.de.protobufs.ResourceTypeOrBuilder> 
      getResourceTypesOrBuilderList() {
    return resourceTypes_;
  }
  /**
   * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
   */
  @java.lang.Override
  public int getResourceTypesCount() {
    return resourceTypes_.size();
  }
  /**
   * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceType getResourceTypes(int index) {
    return resourceTypes_.get(index);
  }
  /**
   * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypesOrBuilder(
      int index) {
    return resourceTypes_.get(index);
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
    for (int i = 0; i < resourceTypes_.size(); i++) {
      output.writeMessage(3, resourceTypes_.get(i));
    }
    unknownFields.writeTo(output);
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
    for (int i = 0; i < resourceTypes_.size(); i++) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(3, resourceTypes_.get(i));
    }
    size += unknownFields.getSerializedSize();
    memoizedSize = size;
    return size;
  }

  @java.lang.Override
  public boolean equals(final java.lang.Object obj) {
    if (obj == this) {
     return true;
    }
    if (!(obj instanceof org.cyverse.de.protobufs.ResourceTypeList)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.ResourceTypeList other = (org.cyverse.de.protobufs.ResourceTypeList) obj;

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
    if (!getResourceTypesList()
        .equals(other.getResourceTypesList())) return false;
    if (!unknownFields.equals(other.unknownFields)) return false;
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
    if (getResourceTypesCount() > 0) {
      hash = (37 * hash) + RESOURCE_TYPES_FIELD_NUMBER;
      hash = (53 * hash) + getResourceTypesList().hashCode();
    }
    hash = (29 * hash) + unknownFields.hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.ResourceTypeList parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.ResourceTypeList prototype) {
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
   * Protobuf type {@code ResourceTypeList}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:ResourceTypeList)
      org.cyverse.de.protobufs.ResourceTypeListOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.QMSResourceTypeProtobufs.internal_static_ResourceTypeList_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.QMSResourceTypeProtobufs.internal_static_ResourceTypeList_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.ResourceTypeList.class, org.cyverse.de.protobufs.ResourceTypeList.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.ResourceTypeList.newBuilder()
    private Builder() {
      maybeForceBuilderInitialization();
    }

    private Builder(
        com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
      super(parent);
      maybeForceBuilderInitialization();
    }
    private void maybeForceBuilderInitialization() {
      if (com.google.protobuf.GeneratedMessageV3
              .alwaysUseFieldBuilders) {
        getResourceTypesFieldBuilder();
      }
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
      if (resourceTypesBuilder_ == null) {
        resourceTypes_ = java.util.Collections.emptyList();
        bitField0_ = (bitField0_ & ~0x00000001);
      } else {
        resourceTypesBuilder_.clear();
      }
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.QMSResourceTypeProtobufs.internal_static_ResourceTypeList_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.ResourceTypeList getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.ResourceTypeList.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.ResourceTypeList build() {
      org.cyverse.de.protobufs.ResourceTypeList result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.ResourceTypeList buildPartial() {
      org.cyverse.de.protobufs.ResourceTypeList result = new org.cyverse.de.protobufs.ResourceTypeList(this);
      int from_bitField0_ = bitField0_;
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
      if (resourceTypesBuilder_ == null) {
        if (((bitField0_ & 0x00000001) != 0)) {
          resourceTypes_ = java.util.Collections.unmodifiableList(resourceTypes_);
          bitField0_ = (bitField0_ & ~0x00000001);
        }
        result.resourceTypes_ = resourceTypes_;
      } else {
        result.resourceTypes_ = resourceTypesBuilder_.build();
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
      if (other instanceof org.cyverse.de.protobufs.ResourceTypeList) {
        return mergeFrom((org.cyverse.de.protobufs.ResourceTypeList)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.ResourceTypeList other) {
      if (other == org.cyverse.de.protobufs.ResourceTypeList.getDefaultInstance()) return this;
      if (other.hasHeader()) {
        mergeHeader(other.getHeader());
      }
      if (other.hasError()) {
        mergeError(other.getError());
      }
      if (resourceTypesBuilder_ == null) {
        if (!other.resourceTypes_.isEmpty()) {
          if (resourceTypes_.isEmpty()) {
            resourceTypes_ = other.resourceTypes_;
            bitField0_ = (bitField0_ & ~0x00000001);
          } else {
            ensureResourceTypesIsMutable();
            resourceTypes_.addAll(other.resourceTypes_);
          }
          onChanged();
        }
      } else {
        if (!other.resourceTypes_.isEmpty()) {
          if (resourceTypesBuilder_.isEmpty()) {
            resourceTypesBuilder_.dispose();
            resourceTypesBuilder_ = null;
            resourceTypes_ = other.resourceTypes_;
            bitField0_ = (bitField0_ & ~0x00000001);
            resourceTypesBuilder_ = 
              com.google.protobuf.GeneratedMessageV3.alwaysUseFieldBuilders ?
                 getResourceTypesFieldBuilder() : null;
          } else {
            resourceTypesBuilder_.addAllMessages(other.resourceTypes_);
          }
        }
      }
      this.mergeUnknownFields(other.unknownFields);
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
      org.cyverse.de.protobufs.ResourceTypeList parsedMessage = null;
      try {
        parsedMessage = PARSER.parsePartialFrom(input, extensionRegistry);
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        parsedMessage = (org.cyverse.de.protobufs.ResourceTypeList) e.getUnfinishedMessage();
        throw e.unwrapIOException();
      } finally {
        if (parsedMessage != null) {
          mergeFrom(parsedMessage);
        }
      }
      return this;
    }
    private int bitField0_;

    private org.cyverse.de.protobufs.Header header_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder> headerBuilder_;
    /**
     * <code>.Header header = 1;</code>
     * @return Whether the header field is set.
     */
    public boolean hasHeader() {
      return headerBuilder_ != null || header_ != null;
    }
    /**
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
     * <code>.Header header = 1;</code>
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
     * <code>.Header header = 1;</code>
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
     * <code>.Header header = 1;</code>
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
     * <code>.Header header = 1;</code>
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
     * <code>.Header header = 1;</code>
     */
    public org.cyverse.de.protobufs.Header.Builder getHeaderBuilder() {
      
      onChanged();
      return getHeaderFieldBuilder().getBuilder();
    }
    /**
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
     * <code>.ServiceError error = 2;</code>
     * @return Whether the error field is set.
     */
    public boolean hasError() {
      return errorBuilder_ != null || error_ != null;
    }
    /**
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
     * <code>.ServiceError error = 2;</code>
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
     * <code>.ServiceError error = 2;</code>
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
     * <code>.ServiceError error = 2;</code>
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
     * <code>.ServiceError error = 2;</code>
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
     * <code>.ServiceError error = 2;</code>
     */
    public org.cyverse.de.protobufs.ServiceError.Builder getErrorBuilder() {
      
      onChanged();
      return getErrorFieldBuilder().getBuilder();
    }
    /**
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

    private java.util.List<org.cyverse.de.protobufs.ResourceType> resourceTypes_ =
      java.util.Collections.emptyList();
    private void ensureResourceTypesIsMutable() {
      if (!((bitField0_ & 0x00000001) != 0)) {
        resourceTypes_ = new java.util.ArrayList<org.cyverse.de.protobufs.ResourceType>(resourceTypes_);
        bitField0_ |= 0x00000001;
       }
    }

    private com.google.protobuf.RepeatedFieldBuilderV3<
        org.cyverse.de.protobufs.ResourceType, org.cyverse.de.protobufs.ResourceType.Builder, org.cyverse.de.protobufs.ResourceTypeOrBuilder> resourceTypesBuilder_;

    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public java.util.List<org.cyverse.de.protobufs.ResourceType> getResourceTypesList() {
      if (resourceTypesBuilder_ == null) {
        return java.util.Collections.unmodifiableList(resourceTypes_);
      } else {
        return resourceTypesBuilder_.getMessageList();
      }
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public int getResourceTypesCount() {
      if (resourceTypesBuilder_ == null) {
        return resourceTypes_.size();
      } else {
        return resourceTypesBuilder_.getCount();
      }
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public org.cyverse.de.protobufs.ResourceType getResourceTypes(int index) {
      if (resourceTypesBuilder_ == null) {
        return resourceTypes_.get(index);
      } else {
        return resourceTypesBuilder_.getMessage(index);
      }
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder setResourceTypes(
        int index, org.cyverse.de.protobufs.ResourceType value) {
      if (resourceTypesBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        ensureResourceTypesIsMutable();
        resourceTypes_.set(index, value);
        onChanged();
      } else {
        resourceTypesBuilder_.setMessage(index, value);
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder setResourceTypes(
        int index, org.cyverse.de.protobufs.ResourceType.Builder builderForValue) {
      if (resourceTypesBuilder_ == null) {
        ensureResourceTypesIsMutable();
        resourceTypes_.set(index, builderForValue.build());
        onChanged();
      } else {
        resourceTypesBuilder_.setMessage(index, builderForValue.build());
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder addResourceTypes(org.cyverse.de.protobufs.ResourceType value) {
      if (resourceTypesBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        ensureResourceTypesIsMutable();
        resourceTypes_.add(value);
        onChanged();
      } else {
        resourceTypesBuilder_.addMessage(value);
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder addResourceTypes(
        int index, org.cyverse.de.protobufs.ResourceType value) {
      if (resourceTypesBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        ensureResourceTypesIsMutable();
        resourceTypes_.add(index, value);
        onChanged();
      } else {
        resourceTypesBuilder_.addMessage(index, value);
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder addResourceTypes(
        org.cyverse.de.protobufs.ResourceType.Builder builderForValue) {
      if (resourceTypesBuilder_ == null) {
        ensureResourceTypesIsMutable();
        resourceTypes_.add(builderForValue.build());
        onChanged();
      } else {
        resourceTypesBuilder_.addMessage(builderForValue.build());
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder addResourceTypes(
        int index, org.cyverse.de.protobufs.ResourceType.Builder builderForValue) {
      if (resourceTypesBuilder_ == null) {
        ensureResourceTypesIsMutable();
        resourceTypes_.add(index, builderForValue.build());
        onChanged();
      } else {
        resourceTypesBuilder_.addMessage(index, builderForValue.build());
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder addAllResourceTypes(
        java.lang.Iterable<? extends org.cyverse.de.protobufs.ResourceType> values) {
      if (resourceTypesBuilder_ == null) {
        ensureResourceTypesIsMutable();
        com.google.protobuf.AbstractMessageLite.Builder.addAll(
            values, resourceTypes_);
        onChanged();
      } else {
        resourceTypesBuilder_.addAllMessages(values);
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder clearResourceTypes() {
      if (resourceTypesBuilder_ == null) {
        resourceTypes_ = java.util.Collections.emptyList();
        bitField0_ = (bitField0_ & ~0x00000001);
        onChanged();
      } else {
        resourceTypesBuilder_.clear();
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public Builder removeResourceTypes(int index) {
      if (resourceTypesBuilder_ == null) {
        ensureResourceTypesIsMutable();
        resourceTypes_.remove(index);
        onChanged();
      } else {
        resourceTypesBuilder_.remove(index);
      }
      return this;
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public org.cyverse.de.protobufs.ResourceType.Builder getResourceTypesBuilder(
        int index) {
      return getResourceTypesFieldBuilder().getBuilder(index);
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypesOrBuilder(
        int index) {
      if (resourceTypesBuilder_ == null) {
        return resourceTypes_.get(index);  } else {
        return resourceTypesBuilder_.getMessageOrBuilder(index);
      }
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public java.util.List<? extends org.cyverse.de.protobufs.ResourceTypeOrBuilder> 
         getResourceTypesOrBuilderList() {
      if (resourceTypesBuilder_ != null) {
        return resourceTypesBuilder_.getMessageOrBuilderList();
      } else {
        return java.util.Collections.unmodifiableList(resourceTypes_);
      }
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public org.cyverse.de.protobufs.ResourceType.Builder addResourceTypesBuilder() {
      return getResourceTypesFieldBuilder().addBuilder(
          org.cyverse.de.protobufs.ResourceType.getDefaultInstance());
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public org.cyverse.de.protobufs.ResourceType.Builder addResourceTypesBuilder(
        int index) {
      return getResourceTypesFieldBuilder().addBuilder(
          index, org.cyverse.de.protobufs.ResourceType.getDefaultInstance());
    }
    /**
     * <code>repeated .ResourceType resource_types = 3 [json_name = "resource_types"];</code>
     */
    public java.util.List<org.cyverse.de.protobufs.ResourceType.Builder> 
         getResourceTypesBuilderList() {
      return getResourceTypesFieldBuilder().getBuilderList();
    }
    private com.google.protobuf.RepeatedFieldBuilderV3<
        org.cyverse.de.protobufs.ResourceType, org.cyverse.de.protobufs.ResourceType.Builder, org.cyverse.de.protobufs.ResourceTypeOrBuilder> 
        getResourceTypesFieldBuilder() {
      if (resourceTypesBuilder_ == null) {
        resourceTypesBuilder_ = new com.google.protobuf.RepeatedFieldBuilderV3<
            org.cyverse.de.protobufs.ResourceType, org.cyverse.de.protobufs.ResourceType.Builder, org.cyverse.de.protobufs.ResourceTypeOrBuilder>(
                resourceTypes_,
                ((bitField0_ & 0x00000001) != 0),
                getParentForChildren(),
                isClean());
        resourceTypes_ = null;
      }
      return resourceTypesBuilder_;
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


    // @@protoc_insertion_point(builder_scope:ResourceTypeList)
  }

  // @@protoc_insertion_point(class_scope:ResourceTypeList)
  private static final org.cyverse.de.protobufs.ResourceTypeList DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.ResourceTypeList();
  }

  public static org.cyverse.de.protobufs.ResourceTypeList getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<ResourceTypeList>
      PARSER = new com.google.protobuf.AbstractParser<ResourceTypeList>() {
    @java.lang.Override
    public ResourceTypeList parsePartialFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws com.google.protobuf.InvalidProtocolBufferException {
      return new ResourceTypeList(input, extensionRegistry);
    }
  };

  public static com.google.protobuf.Parser<ResourceTypeList> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<ResourceTypeList> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceTypeList getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}
