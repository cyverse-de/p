// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: containers.proto
// Protobuf Java Version: 4.29.3

package org.cyverse.de.protobufs;

/**
 * <pre>
 * *
 * A port the container exposes.
 *
 * Correlates to the 'container_ports' table in the 'de' database.
 * </pre>
 *
 * Protobuf type {@code containers.Port}
 */
public final class Port extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:containers.Port)
    PortOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 29,
      /* patch= */ 3,
      /* suffix= */ "",
      Port.class.getName());
  }
  // Use Port.newBuilder() to construct.
  private Port(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private Port() {
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.ContainersProtobufs.internal_static_containers_Port_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.ContainersProtobufs.internal_static_containers_Port_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.Port.class, org.cyverse.de.protobufs.Port.Builder.class);
  }

  public static final int HOST_PORT_FIELD_NUMBER = 1;
  private int hostPort_ = 0;
  /**
   * <pre>
   * The port on the host that the container port should be mapped to. Usually unset.
   * </pre>
   *
   * <code>int32 host_port = 1 [json_name = "host_port"];</code>
   * @return The hostPort.
   */
  @java.lang.Override
  public int getHostPort() {
    return hostPort_;
  }

  public static final int CONTAINER_PORT_FIELD_NUMBER = 2;
  private int containerPort_ = 0;
  /**
   * <pre>
   * The port the contained process needs to have open.
   * </pre>
   *
   * <code>int32 container_port = 2 [json_name = "container_port"];</code>
   * @return The containerPort.
   */
  @java.lang.Override
  public int getContainerPort() {
    return containerPort_;
  }

  public static final int BIND_TO_HOST_FIELD_NUMBER = 3;
  private boolean bindToHost_ = false;
  /**
   * <pre>
   * Whether to bind the container port to the host port. Normally left to false/null/None.
   * </pre>
   *
   * <code>bool bind_to_host = 3 [json_name = "bind_to_host"];</code>
   * @return The bindToHost.
   */
  @java.lang.Override
  public boolean getBindToHost() {
    return bindToHost_;
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
    if (hostPort_ != 0) {
      output.writeInt32(1, hostPort_);
    }
    if (containerPort_ != 0) {
      output.writeInt32(2, containerPort_);
    }
    if (bindToHost_ != false) {
      output.writeBool(3, bindToHost_);
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (hostPort_ != 0) {
      size += com.google.protobuf.CodedOutputStream
        .computeInt32Size(1, hostPort_);
    }
    if (containerPort_ != 0) {
      size += com.google.protobuf.CodedOutputStream
        .computeInt32Size(2, containerPort_);
    }
    if (bindToHost_ != false) {
      size += com.google.protobuf.CodedOutputStream
        .computeBoolSize(3, bindToHost_);
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
    if (!(obj instanceof org.cyverse.de.protobufs.Port)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.Port other = (org.cyverse.de.protobufs.Port) obj;

    if (getHostPort()
        != other.getHostPort()) return false;
    if (getContainerPort()
        != other.getContainerPort()) return false;
    if (getBindToHost()
        != other.getBindToHost()) return false;
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
    hash = (37 * hash) + HOST_PORT_FIELD_NUMBER;
    hash = (53 * hash) + getHostPort();
    hash = (37 * hash) + CONTAINER_PORT_FIELD_NUMBER;
    hash = (53 * hash) + getContainerPort();
    hash = (37 * hash) + BIND_TO_HOST_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashBoolean(
        getBindToHost());
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.Port parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static org.cyverse.de.protobufs.Port parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static org.cyverse.de.protobufs.Port parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.Port parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.Port prototype) {
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
   * A port the container exposes.
   *
   * Correlates to the 'container_ports' table in the 'de' database.
   * </pre>
   *
   * Protobuf type {@code containers.Port}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:containers.Port)
      org.cyverse.de.protobufs.PortOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.ContainersProtobufs.internal_static_containers_Port_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.ContainersProtobufs.internal_static_containers_Port_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.Port.class, org.cyverse.de.protobufs.Port.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.Port.newBuilder()
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
      hostPort_ = 0;
      containerPort_ = 0;
      bindToHost_ = false;
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.ContainersProtobufs.internal_static_containers_Port_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Port getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.Port.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Port build() {
      org.cyverse.de.protobufs.Port result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Port buildPartial() {
      org.cyverse.de.protobufs.Port result = new org.cyverse.de.protobufs.Port(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.Port result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.hostPort_ = hostPort_;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.containerPort_ = containerPort_;
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.bindToHost_ = bindToHost_;
      }
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.Port) {
        return mergeFrom((org.cyverse.de.protobufs.Port)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.Port other) {
      if (other == org.cyverse.de.protobufs.Port.getDefaultInstance()) return this;
      if (other.getHostPort() != 0) {
        setHostPort(other.getHostPort());
      }
      if (other.getContainerPort() != 0) {
        setContainerPort(other.getContainerPort());
      }
      if (other.getBindToHost() != false) {
        setBindToHost(other.getBindToHost());
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
            case 8: {
              hostPort_ = input.readInt32();
              bitField0_ |= 0x00000001;
              break;
            } // case 8
            case 16: {
              containerPort_ = input.readInt32();
              bitField0_ |= 0x00000002;
              break;
            } // case 16
            case 24: {
              bindToHost_ = input.readBool();
              bitField0_ |= 0x00000004;
              break;
            } // case 24
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

    private int hostPort_ ;
    /**
     * <pre>
     * The port on the host that the container port should be mapped to. Usually unset.
     * </pre>
     *
     * <code>int32 host_port = 1 [json_name = "host_port"];</code>
     * @return The hostPort.
     */
    @java.lang.Override
    public int getHostPort() {
      return hostPort_;
    }
    /**
     * <pre>
     * The port on the host that the container port should be mapped to. Usually unset.
     * </pre>
     *
     * <code>int32 host_port = 1 [json_name = "host_port"];</code>
     * @param value The hostPort to set.
     * @return This builder for chaining.
     */
    public Builder setHostPort(int value) {

      hostPort_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The port on the host that the container port should be mapped to. Usually unset.
     * </pre>
     *
     * <code>int32 host_port = 1 [json_name = "host_port"];</code>
     * @return This builder for chaining.
     */
    public Builder clearHostPort() {
      bitField0_ = (bitField0_ & ~0x00000001);
      hostPort_ = 0;
      onChanged();
      return this;
    }

    private int containerPort_ ;
    /**
     * <pre>
     * The port the contained process needs to have open.
     * </pre>
     *
     * <code>int32 container_port = 2 [json_name = "container_port"];</code>
     * @return The containerPort.
     */
    @java.lang.Override
    public int getContainerPort() {
      return containerPort_;
    }
    /**
     * <pre>
     * The port the contained process needs to have open.
     * </pre>
     *
     * <code>int32 container_port = 2 [json_name = "container_port"];</code>
     * @param value The containerPort to set.
     * @return This builder for chaining.
     */
    public Builder setContainerPort(int value) {

      containerPort_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The port the contained process needs to have open.
     * </pre>
     *
     * <code>int32 container_port = 2 [json_name = "container_port"];</code>
     * @return This builder for chaining.
     */
    public Builder clearContainerPort() {
      bitField0_ = (bitField0_ & ~0x00000002);
      containerPort_ = 0;
      onChanged();
      return this;
    }

    private boolean bindToHost_ ;
    /**
     * <pre>
     * Whether to bind the container port to the host port. Normally left to false/null/None.
     * </pre>
     *
     * <code>bool bind_to_host = 3 [json_name = "bind_to_host"];</code>
     * @return The bindToHost.
     */
    @java.lang.Override
    public boolean getBindToHost() {
      return bindToHost_;
    }
    /**
     * <pre>
     * Whether to bind the container port to the host port. Normally left to false/null/None.
     * </pre>
     *
     * <code>bool bind_to_host = 3 [json_name = "bind_to_host"];</code>
     * @param value The bindToHost to set.
     * @return This builder for chaining.
     */
    public Builder setBindToHost(boolean value) {

      bindToHost_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Whether to bind the container port to the host port. Normally left to false/null/None.
     * </pre>
     *
     * <code>bool bind_to_host = 3 [json_name = "bind_to_host"];</code>
     * @return This builder for chaining.
     */
    public Builder clearBindToHost() {
      bitField0_ = (bitField0_ & ~0x00000004);
      bindToHost_ = false;
      onChanged();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:containers.Port)
  }

  // @@protoc_insertion_point(class_scope:containers.Port)
  private static final org.cyverse.de.protobufs.Port DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.Port();
  }

  public static org.cyverse.de.protobufs.Port getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<Port>
      PARSER = new com.google.protobuf.AbstractParser<Port>() {
    @java.lang.Override
    public Port parsePartialFrom(
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

  public static com.google.protobuf.Parser<Port> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<Port> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.Port getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

