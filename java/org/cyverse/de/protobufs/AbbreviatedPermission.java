// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: groups.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

/**
 * <pre>
 * *
 * Abbreviated information about permissions granted to a user.
 * </pre>
 *
 * Protobuf type {@code groups.AbbreviatedPermission}
 */
public final class AbbreviatedPermission extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:groups.AbbreviatedPermission)
    AbbreviatedPermissionOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 29,
      /* patch= */ 0,
      /* suffix= */ "",
      AbbreviatedPermission.class.getName());
  }
  // Use AbbreviatedPermission.newBuilder() to construct.
  private AbbreviatedPermission(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private AbbreviatedPermission() {
    permissionId_ = "";
    resourceName_ = "";
    resourceType_ = "";
    permissionLevel_ = 0;
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_AbbreviatedPermission_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_AbbreviatedPermission_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.AbbreviatedPermission.class, org.cyverse.de.protobufs.AbbreviatedPermission.Builder.class);
  }

  public static final int PERMISSION_ID_FIELD_NUMBER = 1;
  @SuppressWarnings("serial")
  private volatile java.lang.Object permissionId_ = "";
  /**
   * <pre>
   * The permission identifier.
   * </pre>
   *
   * <code>string permission_id = 1 [json_name = "permission_id"];</code>
   * @return The permissionId.
   */
  @java.lang.Override
  public java.lang.String getPermissionId() {
    java.lang.Object ref = permissionId_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      permissionId_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The permission identifier.
   * </pre>
   *
   * <code>string permission_id = 1 [json_name = "permission_id"];</code>
   * @return The bytes for permissionId.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getPermissionIdBytes() {
    java.lang.Object ref = permissionId_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      permissionId_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int RESOURCE_NAME_FIELD_NUMBER = 2;
  @SuppressWarnings("serial")
  private volatile java.lang.Object resourceName_ = "";
  /**
   * <pre>
   * The name of the resource.
   * </pre>
   *
   * <code>string resource_name = 2 [json_name = "resource_name"];</code>
   * @return The resourceName.
   */
  @java.lang.Override
  public java.lang.String getResourceName() {
    java.lang.Object ref = resourceName_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      resourceName_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The name of the resource.
   * </pre>
   *
   * <code>string resource_name = 2 [json_name = "resource_name"];</code>
   * @return The bytes for resourceName.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getResourceNameBytes() {
    java.lang.Object ref = resourceName_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      resourceName_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int RESOURCE_TYPE_FIELD_NUMBER = 3;
  @SuppressWarnings("serial")
  private volatile java.lang.Object resourceType_ = "";
  /**
   * <pre>
   * The type of the resource.
   * </pre>
   *
   * <code>string resource_type = 3 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  @java.lang.Override
  public java.lang.String getResourceType() {
    java.lang.Object ref = resourceType_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      resourceType_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The type of the resource.
   * </pre>
   *
   * <code>string resource_type = 3 [json_name = "resource_type"];</code>
   * @return The bytes for resourceType.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getResourceTypeBytes() {
    java.lang.Object ref = resourceType_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      resourceType_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int PERMISSION_LEVEL_FIELD_NUMBER = 4;
  private int permissionLevel_ = 0;
  /**
   * <pre>
   * The permission level.
   * </pre>
   *
   * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
   * @return The enum numeric value on the wire for permissionLevel.
   */
  @java.lang.Override public int getPermissionLevelValue() {
    return permissionLevel_;
  }
  /**
   * <pre>
   * The permission level.
   * </pre>
   *
   * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
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
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(permissionId_)) {
      com.google.protobuf.GeneratedMessage.writeString(output, 1, permissionId_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(resourceName_)) {
      com.google.protobuf.GeneratedMessage.writeString(output, 2, resourceName_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(resourceType_)) {
      com.google.protobuf.GeneratedMessage.writeString(output, 3, resourceType_);
    }
    if (permissionLevel_ != org.cyverse.de.protobufs.PermissionLevel.read.getNumber()) {
      output.writeEnum(4, permissionLevel_);
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(permissionId_)) {
      size += com.google.protobuf.GeneratedMessage.computeStringSize(1, permissionId_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(resourceName_)) {
      size += com.google.protobuf.GeneratedMessage.computeStringSize(2, resourceName_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(resourceType_)) {
      size += com.google.protobuf.GeneratedMessage.computeStringSize(3, resourceType_);
    }
    if (permissionLevel_ != org.cyverse.de.protobufs.PermissionLevel.read.getNumber()) {
      size += com.google.protobuf.CodedOutputStream
        .computeEnumSize(4, permissionLevel_);
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
    if (!(obj instanceof org.cyverse.de.protobufs.AbbreviatedPermission)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.AbbreviatedPermission other = (org.cyverse.de.protobufs.AbbreviatedPermission) obj;

    if (!getPermissionId()
        .equals(other.getPermissionId())) return false;
    if (!getResourceName()
        .equals(other.getResourceName())) return false;
    if (!getResourceType()
        .equals(other.getResourceType())) return false;
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
    hash = (37 * hash) + PERMISSION_ID_FIELD_NUMBER;
    hash = (53 * hash) + getPermissionId().hashCode();
    hash = (37 * hash) + RESOURCE_NAME_FIELD_NUMBER;
    hash = (53 * hash) + getResourceName().hashCode();
    hash = (37 * hash) + RESOURCE_TYPE_FIELD_NUMBER;
    hash = (53 * hash) + getResourceType().hashCode();
    hash = (37 * hash) + PERMISSION_LEVEL_FIELD_NUMBER;
    hash = (53 * hash) + permissionLevel_;
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static org.cyverse.de.protobufs.AbbreviatedPermission parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static org.cyverse.de.protobufs.AbbreviatedPermission parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.AbbreviatedPermission parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.AbbreviatedPermission prototype) {
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
   * Abbreviated information about permissions granted to a user.
   * </pre>
   *
   * Protobuf type {@code groups.AbbreviatedPermission}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:groups.AbbreviatedPermission)
      org.cyverse.de.protobufs.AbbreviatedPermissionOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_AbbreviatedPermission_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_AbbreviatedPermission_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.AbbreviatedPermission.class, org.cyverse.de.protobufs.AbbreviatedPermission.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.AbbreviatedPermission.newBuilder()
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
      permissionId_ = "";
      resourceName_ = "";
      resourceType_ = "";
      permissionLevel_ = 0;
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_AbbreviatedPermission_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.AbbreviatedPermission getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.AbbreviatedPermission.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.AbbreviatedPermission build() {
      org.cyverse.de.protobufs.AbbreviatedPermission result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.AbbreviatedPermission buildPartial() {
      org.cyverse.de.protobufs.AbbreviatedPermission result = new org.cyverse.de.protobufs.AbbreviatedPermission(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.AbbreviatedPermission result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.permissionId_ = permissionId_;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.resourceName_ = resourceName_;
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.resourceType_ = resourceType_;
      }
      if (((from_bitField0_ & 0x00000008) != 0)) {
        result.permissionLevel_ = permissionLevel_;
      }
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.AbbreviatedPermission) {
        return mergeFrom((org.cyverse.de.protobufs.AbbreviatedPermission)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.AbbreviatedPermission other) {
      if (other == org.cyverse.de.protobufs.AbbreviatedPermission.getDefaultInstance()) return this;
      if (!other.getPermissionId().isEmpty()) {
        permissionId_ = other.permissionId_;
        bitField0_ |= 0x00000001;
        onChanged();
      }
      if (!other.getResourceName().isEmpty()) {
        resourceName_ = other.resourceName_;
        bitField0_ |= 0x00000002;
        onChanged();
      }
      if (!other.getResourceType().isEmpty()) {
        resourceType_ = other.resourceType_;
        bitField0_ |= 0x00000004;
        onChanged();
      }
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
            case 10: {
              permissionId_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000001;
              break;
            } // case 10
            case 18: {
              resourceName_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 26: {
              resourceType_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000004;
              break;
            } // case 26
            case 32: {
              permissionLevel_ = input.readEnum();
              bitField0_ |= 0x00000008;
              break;
            } // case 32
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

    private java.lang.Object permissionId_ = "";
    /**
     * <pre>
     * The permission identifier.
     * </pre>
     *
     * <code>string permission_id = 1 [json_name = "permission_id"];</code>
     * @return The permissionId.
     */
    public java.lang.String getPermissionId() {
      java.lang.Object ref = permissionId_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        permissionId_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The permission identifier.
     * </pre>
     *
     * <code>string permission_id = 1 [json_name = "permission_id"];</code>
     * @return The bytes for permissionId.
     */
    public com.google.protobuf.ByteString
        getPermissionIdBytes() {
      java.lang.Object ref = permissionId_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        permissionId_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The permission identifier.
     * </pre>
     *
     * <code>string permission_id = 1 [json_name = "permission_id"];</code>
     * @param value The permissionId to set.
     * @return This builder for chaining.
     */
    public Builder setPermissionId(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      permissionId_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The permission identifier.
     * </pre>
     *
     * <code>string permission_id = 1 [json_name = "permission_id"];</code>
     * @return This builder for chaining.
     */
    public Builder clearPermissionId() {
      permissionId_ = getDefaultInstance().getPermissionId();
      bitField0_ = (bitField0_ & ~0x00000001);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The permission identifier.
     * </pre>
     *
     * <code>string permission_id = 1 [json_name = "permission_id"];</code>
     * @param value The bytes for permissionId to set.
     * @return This builder for chaining.
     */
    public Builder setPermissionIdBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      permissionId_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }

    private java.lang.Object resourceName_ = "";
    /**
     * <pre>
     * The name of the resource.
     * </pre>
     *
     * <code>string resource_name = 2 [json_name = "resource_name"];</code>
     * @return The resourceName.
     */
    public java.lang.String getResourceName() {
      java.lang.Object ref = resourceName_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        resourceName_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The name of the resource.
     * </pre>
     *
     * <code>string resource_name = 2 [json_name = "resource_name"];</code>
     * @return The bytes for resourceName.
     */
    public com.google.protobuf.ByteString
        getResourceNameBytes() {
      java.lang.Object ref = resourceName_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        resourceName_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The name of the resource.
     * </pre>
     *
     * <code>string resource_name = 2 [json_name = "resource_name"];</code>
     * @param value The resourceName to set.
     * @return This builder for chaining.
     */
    public Builder setResourceName(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      resourceName_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The name of the resource.
     * </pre>
     *
     * <code>string resource_name = 2 [json_name = "resource_name"];</code>
     * @return This builder for chaining.
     */
    public Builder clearResourceName() {
      resourceName_ = getDefaultInstance().getResourceName();
      bitField0_ = (bitField0_ & ~0x00000002);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The name of the resource.
     * </pre>
     *
     * <code>string resource_name = 2 [json_name = "resource_name"];</code>
     * @param value The bytes for resourceName to set.
     * @return This builder for chaining.
     */
    public Builder setResourceNameBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      resourceName_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }

    private java.lang.Object resourceType_ = "";
    /**
     * <pre>
     * The type of the resource.
     * </pre>
     *
     * <code>string resource_type = 3 [json_name = "resource_type"];</code>
     * @return The resourceType.
     */
    public java.lang.String getResourceType() {
      java.lang.Object ref = resourceType_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        resourceType_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The type of the resource.
     * </pre>
     *
     * <code>string resource_type = 3 [json_name = "resource_type"];</code>
     * @return The bytes for resourceType.
     */
    public com.google.protobuf.ByteString
        getResourceTypeBytes() {
      java.lang.Object ref = resourceType_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        resourceType_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The type of the resource.
     * </pre>
     *
     * <code>string resource_type = 3 [json_name = "resource_type"];</code>
     * @param value The resourceType to set.
     * @return This builder for chaining.
     */
    public Builder setResourceType(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      resourceType_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The type of the resource.
     * </pre>
     *
     * <code>string resource_type = 3 [json_name = "resource_type"];</code>
     * @return This builder for chaining.
     */
    public Builder clearResourceType() {
      resourceType_ = getDefaultInstance().getResourceType();
      bitField0_ = (bitField0_ & ~0x00000004);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The type of the resource.
     * </pre>
     *
     * <code>string resource_type = 3 [json_name = "resource_type"];</code>
     * @param value The bytes for resourceType to set.
     * @return This builder for chaining.
     */
    public Builder setResourceTypeBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      resourceType_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }

    private int permissionLevel_ = 0;
    /**
     * <pre>
     * The permission level.
     * </pre>
     *
     * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
     * @return The enum numeric value on the wire for permissionLevel.
     */
    @java.lang.Override public int getPermissionLevelValue() {
      return permissionLevel_;
    }
    /**
     * <pre>
     * The permission level.
     * </pre>
     *
     * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
     * @param value The enum numeric value on the wire for permissionLevel to set.
     * @return This builder for chaining.
     */
    public Builder setPermissionLevelValue(int value) {
      permissionLevel_ = value;
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The permission level.
     * </pre>
     *
     * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
     * @return The permissionLevel.
     */
    @java.lang.Override
    public org.cyverse.de.protobufs.PermissionLevel getPermissionLevel() {
      org.cyverse.de.protobufs.PermissionLevel result = org.cyverse.de.protobufs.PermissionLevel.forNumber(permissionLevel_);
      return result == null ? org.cyverse.de.protobufs.PermissionLevel.UNRECOGNIZED : result;
    }
    /**
     * <pre>
     * The permission level.
     * </pre>
     *
     * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
     * @param value The permissionLevel to set.
     * @return This builder for chaining.
     */
    public Builder setPermissionLevel(org.cyverse.de.protobufs.PermissionLevel value) {
      if (value == null) {
        throw new NullPointerException();
      }
      bitField0_ |= 0x00000008;
      permissionLevel_ = value.getNumber();
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The permission level.
     * </pre>
     *
     * <code>.groups.PermissionLevel permission_level = 4 [json_name = "permission_level"];</code>
     * @return This builder for chaining.
     */
    public Builder clearPermissionLevel() {
      bitField0_ = (bitField0_ & ~0x00000008);
      permissionLevel_ = 0;
      onChanged();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:groups.AbbreviatedPermission)
  }

  // @@protoc_insertion_point(class_scope:groups.AbbreviatedPermission)
  private static final org.cyverse.de.protobufs.AbbreviatedPermission DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.AbbreviatedPermission();
  }

  public static org.cyverse.de.protobufs.AbbreviatedPermission getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<AbbreviatedPermission>
      PARSER = new com.google.protobuf.AbstractParser<AbbreviatedPermission>() {
    @java.lang.Override
    public AbbreviatedPermission parsePartialFrom(
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

  public static com.google.protobuf.Parser<AbbreviatedPermission> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<AbbreviatedPermission> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.AbbreviatedPermission getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

