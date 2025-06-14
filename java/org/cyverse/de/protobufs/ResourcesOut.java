// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: groups.proto
// Protobuf Java Version: 4.31.1

package org.cyverse.de.protobufs;

/**
 * <pre>
 * *
 * A list of resource.
 * </pre>
 *
 * Protobuf type {@code groups.ResourcesOut}
 */
@com.google.protobuf.Generated
public final class ResourcesOut extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:groups.ResourcesOut)
    ResourcesOutOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 31,
      /* patch= */ 1,
      /* suffix= */ "",
      ResourcesOut.class.getName());
  }
  // Use ResourcesOut.newBuilder() to construct.
  private ResourcesOut(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private ResourcesOut() {
    resources_ = java.util.Collections.emptyList();
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_ResourcesOut_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_ResourcesOut_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.ResourcesOut.class, org.cyverse.de.protobufs.ResourcesOut.Builder.class);
  }

  public static final int RESOURCES_FIELD_NUMBER = 1;
  @SuppressWarnings("serial")
  private java.util.List<org.cyverse.de.protobufs.ResourceOut> resources_;
  /**
   * <pre>
   * The list of resources.
   * </pre>
   *
   * <code>repeated .groups.ResourceOut resources = 1;</code>
   */
  @java.lang.Override
  public java.util.List<org.cyverse.de.protobufs.ResourceOut> getResourcesList() {
    return resources_;
  }
  /**
   * <pre>
   * The list of resources.
   * </pre>
   *
   * <code>repeated .groups.ResourceOut resources = 1;</code>
   */
  @java.lang.Override
  public java.util.List<? extends org.cyverse.de.protobufs.ResourceOutOrBuilder> 
      getResourcesOrBuilderList() {
    return resources_;
  }
  /**
   * <pre>
   * The list of resources.
   * </pre>
   *
   * <code>repeated .groups.ResourceOut resources = 1;</code>
   */
  @java.lang.Override
  public int getResourcesCount() {
    return resources_.size();
  }
  /**
   * <pre>
   * The list of resources.
   * </pre>
   *
   * <code>repeated .groups.ResourceOut resources = 1;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceOut getResources(int index) {
    return resources_.get(index);
  }
  /**
   * <pre>
   * The list of resources.
   * </pre>
   *
   * <code>repeated .groups.ResourceOut resources = 1;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceOutOrBuilder getResourcesOrBuilder(
      int index) {
    return resources_.get(index);
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
    for (int i = 0; i < resources_.size(); i++) {
      output.writeMessage(1, resources_.get(i));
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    for (int i = 0; i < resources_.size(); i++) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(1, resources_.get(i));
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
    if (!(obj instanceof org.cyverse.de.protobufs.ResourcesOut)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.ResourcesOut other = (org.cyverse.de.protobufs.ResourcesOut) obj;

    if (!getResourcesList()
        .equals(other.getResourcesList())) return false;
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
    if (getResourcesCount() > 0) {
      hash = (37 * hash) + RESOURCES_FIELD_NUMBER;
      hash = (53 * hash) + getResourcesList().hashCode();
    }
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static org.cyverse.de.protobufs.ResourcesOut parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static org.cyverse.de.protobufs.ResourcesOut parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.ResourcesOut parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  @java.lang.Override
  public Builder newBuilderForType() { return newBuilder(); }
  public static Builder newBuilder() {
    return DEFAULT_INSTANCE.toBuilder();
  }
  public static Builder newBuilder(org.cyverse.de.protobufs.ResourcesOut prototype) {
    return DEFAULT_INSTANCE.toBuilder().mergeFrom(prototype);
  }
  @java.lang.Override
  public Builder toBuilder() {
    return this == DEFAULT_INSTANCE
        ? new Builder() : new Builder().mergeFrom(this);
  }

  @java.lang.Override
  protected Builder newBuilderForType(
      com.google.protobuf.GeneratedMessage.BuilderParent parent) {
    Builder builder = new Builder(parent);
    return builder;
  }
  /**
   * <pre>
   * *
   * A list of resource.
   * </pre>
   *
   * Protobuf type {@code groups.ResourcesOut}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:groups.ResourcesOut)
      org.cyverse.de.protobufs.ResourcesOutOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_ResourcesOut_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_ResourcesOut_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.ResourcesOut.class, org.cyverse.de.protobufs.ResourcesOut.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.ResourcesOut.newBuilder()
    private Builder() {

    }

    private Builder(
        com.google.protobuf.GeneratedMessage.BuilderParent parent) {
      super(parent);

    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      bitField0_ = 0;
      if (resourcesBuilder_ == null) {
        resources_ = java.util.Collections.emptyList();
      } else {
        resources_ = null;
        resourcesBuilder_.clear();
      }
      bitField0_ = (bitField0_ & ~0x00000001);
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_ResourcesOut_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.ResourcesOut getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.ResourcesOut.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.ResourcesOut build() {
      org.cyverse.de.protobufs.ResourcesOut result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.ResourcesOut buildPartial() {
      org.cyverse.de.protobufs.ResourcesOut result = new org.cyverse.de.protobufs.ResourcesOut(this);
      buildPartialRepeatedFields(result);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartialRepeatedFields(org.cyverse.de.protobufs.ResourcesOut result) {
      if (resourcesBuilder_ == null) {
        if (((bitField0_ & 0x00000001) != 0)) {
          resources_ = java.util.Collections.unmodifiableList(resources_);
          bitField0_ = (bitField0_ & ~0x00000001);
        }
        result.resources_ = resources_;
      } else {
        result.resources_ = resourcesBuilder_.build();
      }
    }

    private void buildPartial0(org.cyverse.de.protobufs.ResourcesOut result) {
      int from_bitField0_ = bitField0_;
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.ResourcesOut) {
        return mergeFrom((org.cyverse.de.protobufs.ResourcesOut)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.ResourcesOut other) {
      if (other == org.cyverse.de.protobufs.ResourcesOut.getDefaultInstance()) return this;
      if (resourcesBuilder_ == null) {
        if (!other.resources_.isEmpty()) {
          if (resources_.isEmpty()) {
            resources_ = other.resources_;
            bitField0_ = (bitField0_ & ~0x00000001);
          } else {
            ensureResourcesIsMutable();
            resources_.addAll(other.resources_);
          }
          onChanged();
        }
      } else {
        if (!other.resources_.isEmpty()) {
          if (resourcesBuilder_.isEmpty()) {
            resourcesBuilder_.dispose();
            resourcesBuilder_ = null;
            resources_ = other.resources_;
            bitField0_ = (bitField0_ & ~0x00000001);
            resourcesBuilder_ = 
              com.google.protobuf.GeneratedMessage.alwaysUseFieldBuilders ?
                 internalGetResourcesFieldBuilder() : null;
          } else {
            resourcesBuilder_.addAllMessages(other.resources_);
          }
        }
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
              org.cyverse.de.protobufs.ResourceOut m =
                  input.readMessage(
                      org.cyverse.de.protobufs.ResourceOut.parser(),
                      extensionRegistry);
              if (resourcesBuilder_ == null) {
                ensureResourcesIsMutable();
                resources_.add(m);
              } else {
                resourcesBuilder_.addMessage(m);
              }
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

    private java.util.List<org.cyverse.de.protobufs.ResourceOut> resources_ =
      java.util.Collections.emptyList();
    private void ensureResourcesIsMutable() {
      if (!((bitField0_ & 0x00000001) != 0)) {
        resources_ = new java.util.ArrayList<org.cyverse.de.protobufs.ResourceOut>(resources_);
        bitField0_ |= 0x00000001;
       }
    }

    private com.google.protobuf.RepeatedFieldBuilder<
        org.cyverse.de.protobufs.ResourceOut, org.cyverse.de.protobufs.ResourceOut.Builder, org.cyverse.de.protobufs.ResourceOutOrBuilder> resourcesBuilder_;

    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public java.util.List<org.cyverse.de.protobufs.ResourceOut> getResourcesList() {
      if (resourcesBuilder_ == null) {
        return java.util.Collections.unmodifiableList(resources_);
      } else {
        return resourcesBuilder_.getMessageList();
      }
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public int getResourcesCount() {
      if (resourcesBuilder_ == null) {
        return resources_.size();
      } else {
        return resourcesBuilder_.getCount();
      }
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public org.cyverse.de.protobufs.ResourceOut getResources(int index) {
      if (resourcesBuilder_ == null) {
        return resources_.get(index);
      } else {
        return resourcesBuilder_.getMessage(index);
      }
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder setResources(
        int index, org.cyverse.de.protobufs.ResourceOut value) {
      if (resourcesBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        ensureResourcesIsMutable();
        resources_.set(index, value);
        onChanged();
      } else {
        resourcesBuilder_.setMessage(index, value);
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder setResources(
        int index, org.cyverse.de.protobufs.ResourceOut.Builder builderForValue) {
      if (resourcesBuilder_ == null) {
        ensureResourcesIsMutable();
        resources_.set(index, builderForValue.build());
        onChanged();
      } else {
        resourcesBuilder_.setMessage(index, builderForValue.build());
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder addResources(org.cyverse.de.protobufs.ResourceOut value) {
      if (resourcesBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        ensureResourcesIsMutable();
        resources_.add(value);
        onChanged();
      } else {
        resourcesBuilder_.addMessage(value);
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder addResources(
        int index, org.cyverse.de.protobufs.ResourceOut value) {
      if (resourcesBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        ensureResourcesIsMutable();
        resources_.add(index, value);
        onChanged();
      } else {
        resourcesBuilder_.addMessage(index, value);
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder addResources(
        org.cyverse.de.protobufs.ResourceOut.Builder builderForValue) {
      if (resourcesBuilder_ == null) {
        ensureResourcesIsMutable();
        resources_.add(builderForValue.build());
        onChanged();
      } else {
        resourcesBuilder_.addMessage(builderForValue.build());
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder addResources(
        int index, org.cyverse.de.protobufs.ResourceOut.Builder builderForValue) {
      if (resourcesBuilder_ == null) {
        ensureResourcesIsMutable();
        resources_.add(index, builderForValue.build());
        onChanged();
      } else {
        resourcesBuilder_.addMessage(index, builderForValue.build());
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder addAllResources(
        java.lang.Iterable<? extends org.cyverse.de.protobufs.ResourceOut> values) {
      if (resourcesBuilder_ == null) {
        ensureResourcesIsMutable();
        com.google.protobuf.AbstractMessageLite.Builder.addAll(
            values, resources_);
        onChanged();
      } else {
        resourcesBuilder_.addAllMessages(values);
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder clearResources() {
      if (resourcesBuilder_ == null) {
        resources_ = java.util.Collections.emptyList();
        bitField0_ = (bitField0_ & ~0x00000001);
        onChanged();
      } else {
        resourcesBuilder_.clear();
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public Builder removeResources(int index) {
      if (resourcesBuilder_ == null) {
        ensureResourcesIsMutable();
        resources_.remove(index);
        onChanged();
      } else {
        resourcesBuilder_.remove(index);
      }
      return this;
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public org.cyverse.de.protobufs.ResourceOut.Builder getResourcesBuilder(
        int index) {
      return internalGetResourcesFieldBuilder().getBuilder(index);
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public org.cyverse.de.protobufs.ResourceOutOrBuilder getResourcesOrBuilder(
        int index) {
      if (resourcesBuilder_ == null) {
        return resources_.get(index);  } else {
        return resourcesBuilder_.getMessageOrBuilder(index);
      }
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public java.util.List<? extends org.cyverse.de.protobufs.ResourceOutOrBuilder> 
         getResourcesOrBuilderList() {
      if (resourcesBuilder_ != null) {
        return resourcesBuilder_.getMessageOrBuilderList();
      } else {
        return java.util.Collections.unmodifiableList(resources_);
      }
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public org.cyverse.de.protobufs.ResourceOut.Builder addResourcesBuilder() {
      return internalGetResourcesFieldBuilder().addBuilder(
          org.cyverse.de.protobufs.ResourceOut.getDefaultInstance());
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public org.cyverse.de.protobufs.ResourceOut.Builder addResourcesBuilder(
        int index) {
      return internalGetResourcesFieldBuilder().addBuilder(
          index, org.cyverse.de.protobufs.ResourceOut.getDefaultInstance());
    }
    /**
     * <pre>
     * The list of resources.
     * </pre>
     *
     * <code>repeated .groups.ResourceOut resources = 1;</code>
     */
    public java.util.List<org.cyverse.de.protobufs.ResourceOut.Builder> 
         getResourcesBuilderList() {
      return internalGetResourcesFieldBuilder().getBuilderList();
    }
    private com.google.protobuf.RepeatedFieldBuilder<
        org.cyverse.de.protobufs.ResourceOut, org.cyverse.de.protobufs.ResourceOut.Builder, org.cyverse.de.protobufs.ResourceOutOrBuilder> 
        internalGetResourcesFieldBuilder() {
      if (resourcesBuilder_ == null) {
        resourcesBuilder_ = new com.google.protobuf.RepeatedFieldBuilder<
            org.cyverse.de.protobufs.ResourceOut, org.cyverse.de.protobufs.ResourceOut.Builder, org.cyverse.de.protobufs.ResourceOutOrBuilder>(
                resources_,
                ((bitField0_ & 0x00000001) != 0),
                getParentForChildren(),
                isClean());
        resources_ = null;
      }
      return resourcesBuilder_;
    }

    // @@protoc_insertion_point(builder_scope:groups.ResourcesOut)
  }

  // @@protoc_insertion_point(class_scope:groups.ResourcesOut)
  private static final org.cyverse.de.protobufs.ResourcesOut DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.ResourcesOut();
  }

  public static org.cyverse.de.protobufs.ResourcesOut getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<ResourcesOut>
      PARSER = new com.google.protobuf.AbstractParser<ResourcesOut>() {
    @java.lang.Override
    public ResourcesOut parsePartialFrom(
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

  public static com.google.protobuf.Parser<ResourcesOut> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<ResourcesOut> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.ResourcesOut getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

