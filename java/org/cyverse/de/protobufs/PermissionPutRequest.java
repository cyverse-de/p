// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: groups.proto

package org.cyverse.de.protobufs;

/**
 * <pre>
 **
 * Specifies the permission level to assign.
 * </pre>
 *
 * Protobuf type {@code groups.PermissionPutRequest}
 */
public final class PermissionPutRequest extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:groups.PermissionPutRequest)
    PermissionPutRequestOrBuilder {
private static final long serialVersionUID = 0L;
  // Use PermissionPutRequest.newBuilder() to construct.
  private PermissionPutRequest(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private PermissionPutRequest() {
    permissionLevel_ = 0;
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new PermissionPutRequest();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_PermissionPutRequest_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_PermissionPutRequest_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.PermissionPutRequest.class, org.cyverse.de.protobufs.PermissionPutRequest.Builder.class);
  }

  public static final int PERMISSION_LEVEL_FIELD_NUMBER = 1;
  private int permissionLevel_ = 0;
  /**
   * <code>.groups.PermissionLevel permission_level = 1 [json_name = "permission_level"];</code>
   * @return The enum numeric value on the wire for permissionLevel.
   */
  @java.lang.Override public int getPermissionLevelValue() {
    return permissionLevel_;
  }
  /**
   * <code>.groups.PermissionLevel permission_level = 1 [json_name = "permission_level"];</code>
   * @return The permissionLevel.
   */
  @java.lang.Override public org.cyverse.de.protobufs.PermissionLevel getPermissionLevel() {
    org.cyverse.de.protobufs.PermissionLevel result = org.cyverse.de.protobufs.PermissionLevel.forNumber(permissionLevel_);
    return result == null ? org.cyverse.de.protobufs.PermissionLevel.UNRECOGNIZED : result;
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
    if (permissionLevel_ != org.cyverse.de.protobufs.PermissionLevel.read.getNumber()) {
      output.writeEnum(1, permissionLevel_);
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (permissionLevel_ != org.cyverse.de.protobufs.PermissionLevel.read.getNumber()) {
      size += com.google.protobuf.CodedOutputStream
        .computeEnumSize(1, permissionLevel_);
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
    if (!(obj instanceof org.cyverse.de.protobufs.PermissionPutRequest)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.PermissionPutRequest other = (org.cyverse.de.protobufs.PermissionPutRequest) obj;

    if (permissionLevel_ != other.permissionLevel_) return false;
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
    hash = (37 * hash) + PERMISSION_LEVEL_FIELD_NUMBER;
    hash = (53 * hash) + permissionLevel_;
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.PermissionPutRequest parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.PermissionPutRequest prototype) {
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
   * Specifies the permission level to assign.
   * </pre>
   *
   * Protobuf type {@code groups.PermissionPutRequest}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:groups.PermissionPutRequest)
      org.cyverse.de.protobufs.PermissionPutRequestOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_PermissionPutRequest_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_PermissionPutRequest_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.PermissionPutRequest.class, org.cyverse.de.protobufs.PermissionPutRequest.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.PermissionPutRequest.newBuilder()
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
      permissionLevel_ = 0;
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_PermissionPutRequest_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.PermissionPutRequest getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.PermissionPutRequest.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.PermissionPutRequest build() {
      org.cyverse.de.protobufs.PermissionPutRequest result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.PermissionPutRequest buildPartial() {
      org.cyverse.de.protobufs.PermissionPutRequest result = new org.cyverse.de.protobufs.PermissionPutRequest(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.PermissionPutRequest result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.permissionLevel_ = permissionLevel_;
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
      if (other instanceof org.cyverse.de.protobufs.PermissionPutRequest) {
        return mergeFrom((org.cyverse.de.protobufs.PermissionPutRequest)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.PermissionPutRequest other) {
      if (other == org.cyverse.de.protobufs.PermissionPutRequest.getDefaultInstance()) return this;
      if (other.permissionLevel_ != 0) {
        setPermissionLevelValue(other.getPermissionLevelValue());
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
              permissionLevel_ = input.readEnum();
              bitField0_ |= 0x00000001;
              break;
            } // case 8
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

    private int permissionLevel_ = 0;
    /**
     * <code>.groups.PermissionLevel permission_level = 1 [json_name = "permission_level"];</code>
     * @return The enum numeric value on the wire for permissionLevel.
     */
    @java.lang.Override public int getPermissionLevelValue() {
      return permissionLevel_;
    }
    /**
     * <code>.groups.PermissionLevel permission_level = 1 [json_name = "permission_level"];</code>
     * @param value The enum numeric value on the wire for permissionLevel to set.
     * @return This builder for chaining.
     */
    public Builder setPermissionLevelValue(int value) {
      permissionLevel_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <code>.groups.PermissionLevel permission_level = 1 [json_name = "permission_level"];</code>
     * @return The permissionLevel.
     */
    @java.lang.Override
    public org.cyverse.de.protobufs.PermissionLevel getPermissionLevel() {
      org.cyverse.de.protobufs.PermissionLevel result = org.cyverse.de.protobufs.PermissionLevel.forNumber(permissionLevel_);
      return result == null ? org.cyverse.de.protobufs.PermissionLevel.UNRECOGNIZED : result;
    }
    /**
     * <code>.groups.PermissionLevel permission_level = 1 [json_name = "permission_level"];</code>
     * @param value The permissionLevel to set.
     * @return This builder for chaining.
     */
    public Builder setPermissionLevel(org.cyverse.de.protobufs.PermissionLevel value) {
      if (value == null) {
        throw new NullPointerException();
      }
      bitField0_ |= 0x00000001;
      permissionLevel_ = value.getNumber();
      onChanged();
      return this;
    }
    /**
     * <code>.groups.PermissionLevel permission_level = 1 [json_name = "permission_level"];</code>
     * @return This builder for chaining.
     */
    public Builder clearPermissionLevel() {
      bitField0_ = (bitField0_ & ~0x00000001);
      permissionLevel_ = 0;
      onChanged();
      return this;
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


    // @@protoc_insertion_point(builder_scope:groups.PermissionPutRequest)
  }

  // @@protoc_insertion_point(class_scope:groups.PermissionPutRequest)
  private static final org.cyverse.de.protobufs.PermissionPutRequest DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.PermissionPutRequest();
  }

  public static org.cyverse.de.protobufs.PermissionPutRequest getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<PermissionPutRequest>
      PARSER = new com.google.protobuf.AbstractParser<PermissionPutRequest>() {
    @java.lang.Override
    public PermissionPutRequest parsePartialFrom(
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

  public static com.google.protobuf.Parser<PermissionPutRequest> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<PermissionPutRequest> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.PermissionPutRequest getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

