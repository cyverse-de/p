// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: user.proto

package org.cyverse.de.protobufs;

/**
 * <pre>
 * How a user can be referred to. Typically only one of them is
 * set. Can be used in Request messages
 * </pre>
 *
 * Protobuf type {@code user.UserRef}
 */
public final class UserRef extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:user.UserRef)
    UserRefOrBuilder {
private static final long serialVersionUID = 0L;
  // Use UserRef.newBuilder() to construct.
  private UserRef(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private UserRef() {
    username_ = "";
    uuid_ = "";
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new UserRef();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.UserProtobufs.internal_static_user_UserRef_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.UserProtobufs.internal_static_user_UserRef_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.UserRef.class, org.cyverse.de.protobufs.UserRef.Builder.class);
  }

  private int bitField0_;
  public static final int USERNAME_FIELD_NUMBER = 1;
  private volatile java.lang.Object username_;
  /**
   * <pre>
   * The username of the user in the database. Must be unique.
   * It's more likely for a service to have this, which is why
   * it's listed first. Writing services to use the username
   * can skip a lookup of the UUID.
   * </pre>
   *
   * <code>optional string username = 1;</code>
   * @return Whether the username field is set.
   */
  @java.lang.Override
  public boolean hasUsername() {
    return ((bitField0_ & 0x00000001) != 0);
  }
  /**
   * <pre>
   * The username of the user in the database. Must be unique.
   * It's more likely for a service to have this, which is why
   * it's listed first. Writing services to use the username
   * can skip a lookup of the UUID.
   * </pre>
   *
   * <code>optional string username = 1;</code>
   * @return The username.
   */
  @java.lang.Override
  public java.lang.String getUsername() {
    java.lang.Object ref = username_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      username_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The username of the user in the database. Must be unique.
   * It's more likely for a service to have this, which is why
   * it's listed first. Writing services to use the username
   * can skip a lookup of the UUID.
   * </pre>
   *
   * <code>optional string username = 1;</code>
   * @return The bytes for username.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getUsernameBytes() {
    java.lang.Object ref = username_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      username_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int UUID_FIELD_NUMBER = 2;
  private volatile java.lang.Object uuid_;
  /**
   * <pre>
   * The UUID of the user in the database. A service can have
   * this, but it's more likely for it to have the username.
   * </pre>
   *
   * <code>optional string uuid = 2;</code>
   * @return Whether the uuid field is set.
   */
  @java.lang.Override
  public boolean hasUuid() {
    return ((bitField0_ & 0x00000002) != 0);
  }
  /**
   * <pre>
   * The UUID of the user in the database. A service can have
   * this, but it's more likely for it to have the username.
   * </pre>
   *
   * <code>optional string uuid = 2;</code>
   * @return The uuid.
   */
  @java.lang.Override
  public java.lang.String getUuid() {
    java.lang.Object ref = uuid_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      uuid_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The UUID of the user in the database. A service can have
   * this, but it's more likely for it to have the username.
   * </pre>
   *
   * <code>optional string uuid = 2;</code>
   * @return The bytes for uuid.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getUuidBytes() {
    java.lang.Object ref = uuid_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      uuid_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
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
    if (((bitField0_ & 0x00000001) != 0)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 1, username_);
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 2, uuid_);
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (((bitField0_ & 0x00000001) != 0)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(1, username_);
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(2, uuid_);
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
    if (!(obj instanceof org.cyverse.de.protobufs.UserRef)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.UserRef other = (org.cyverse.de.protobufs.UserRef) obj;

    if (hasUsername() != other.hasUsername()) return false;
    if (hasUsername()) {
      if (!getUsername()
          .equals(other.getUsername())) return false;
    }
    if (hasUuid() != other.hasUuid()) return false;
    if (hasUuid()) {
      if (!getUuid()
          .equals(other.getUuid())) return false;
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
    if (hasUsername()) {
      hash = (37 * hash) + USERNAME_FIELD_NUMBER;
      hash = (53 * hash) + getUsername().hashCode();
    }
    if (hasUuid()) {
      hash = (37 * hash) + UUID_FIELD_NUMBER;
      hash = (53 * hash) + getUuid().hashCode();
    }
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.UserRef parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserRef parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UserRef parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UserRef parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.UserRef prototype) {
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
   * How a user can be referred to. Typically only one of them is
   * set. Can be used in Request messages
   * </pre>
   *
   * Protobuf type {@code user.UserRef}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:user.UserRef)
      org.cyverse.de.protobufs.UserRefOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.UserProtobufs.internal_static_user_UserRef_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.UserProtobufs.internal_static_user_UserRef_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.UserRef.class, org.cyverse.de.protobufs.UserRef.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.UserRef.newBuilder()
    private Builder() {

    }

    private Builder(
        com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
      super(parent);

    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      username_ = "";
      bitField0_ = (bitField0_ & ~0x00000001);
      uuid_ = "";
      bitField0_ = (bitField0_ & ~0x00000002);
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.UserProtobufs.internal_static_user_UserRef_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UserRef getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.UserRef.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UserRef build() {
      org.cyverse.de.protobufs.UserRef result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UserRef buildPartial() {
      org.cyverse.de.protobufs.UserRef result = new org.cyverse.de.protobufs.UserRef(this);
      int from_bitField0_ = bitField0_;
      int to_bitField0_ = 0;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        to_bitField0_ |= 0x00000001;
      }
      result.username_ = username_;
      if (((from_bitField0_ & 0x00000002) != 0)) {
        to_bitField0_ |= 0x00000002;
      }
      result.uuid_ = uuid_;
      result.bitField0_ = to_bitField0_;
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
      if (other instanceof org.cyverse.de.protobufs.UserRef) {
        return mergeFrom((org.cyverse.de.protobufs.UserRef)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.UserRef other) {
      if (other == org.cyverse.de.protobufs.UserRef.getDefaultInstance()) return this;
      if (other.hasUsername()) {
        bitField0_ |= 0x00000001;
        username_ = other.username_;
        onChanged();
      }
      if (other.hasUuid()) {
        bitField0_ |= 0x00000002;
        uuid_ = other.uuid_;
        onChanged();
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
              username_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000001;
              break;
            } // case 10
            case 18: {
              uuid_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000002;
              break;
            } // case 18
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

    private java.lang.Object username_ = "";
    /**
     * <pre>
     * The username of the user in the database. Must be unique.
     * It's more likely for a service to have this, which is why
     * it's listed first. Writing services to use the username
     * can skip a lookup of the UUID.
     * </pre>
     *
     * <code>optional string username = 1;</code>
     * @return Whether the username field is set.
     */
    public boolean hasUsername() {
      return ((bitField0_ & 0x00000001) != 0);
    }
    /**
     * <pre>
     * The username of the user in the database. Must be unique.
     * It's more likely for a service to have this, which is why
     * it's listed first. Writing services to use the username
     * can skip a lookup of the UUID.
     * </pre>
     *
     * <code>optional string username = 1;</code>
     * @return The username.
     */
    public java.lang.String getUsername() {
      java.lang.Object ref = username_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        username_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The username of the user in the database. Must be unique.
     * It's more likely for a service to have this, which is why
     * it's listed first. Writing services to use the username
     * can skip a lookup of the UUID.
     * </pre>
     *
     * <code>optional string username = 1;</code>
     * @return The bytes for username.
     */
    public com.google.protobuf.ByteString
        getUsernameBytes() {
      java.lang.Object ref = username_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        username_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The username of the user in the database. Must be unique.
     * It's more likely for a service to have this, which is why
     * it's listed first. Writing services to use the username
     * can skip a lookup of the UUID.
     * </pre>
     *
     * <code>optional string username = 1;</code>
     * @param value The username to set.
     * @return This builder for chaining.
     */
    public Builder setUsername(
        java.lang.String value) {
      if (value == null) {
    throw new NullPointerException();
  }
  bitField0_ |= 0x00000001;
      username_ = value;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The username of the user in the database. Must be unique.
     * It's more likely for a service to have this, which is why
     * it's listed first. Writing services to use the username
     * can skip a lookup of the UUID.
     * </pre>
     *
     * <code>optional string username = 1;</code>
     * @return This builder for chaining.
     */
    public Builder clearUsername() {
      bitField0_ = (bitField0_ & ~0x00000001);
      username_ = getDefaultInstance().getUsername();
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The username of the user in the database. Must be unique.
     * It's more likely for a service to have this, which is why
     * it's listed first. Writing services to use the username
     * can skip a lookup of the UUID.
     * </pre>
     *
     * <code>optional string username = 1;</code>
     * @param value The bytes for username to set.
     * @return This builder for chaining.
     */
    public Builder setUsernameBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) {
    throw new NullPointerException();
  }
  checkByteStringIsUtf8(value);
      bitField0_ |= 0x00000001;
      username_ = value;
      onChanged();
      return this;
    }

    private java.lang.Object uuid_ = "";
    /**
     * <pre>
     * The UUID of the user in the database. A service can have
     * this, but it's more likely for it to have the username.
     * </pre>
     *
     * <code>optional string uuid = 2;</code>
     * @return Whether the uuid field is set.
     */
    public boolean hasUuid() {
      return ((bitField0_ & 0x00000002) != 0);
    }
    /**
     * <pre>
     * The UUID of the user in the database. A service can have
     * this, but it's more likely for it to have the username.
     * </pre>
     *
     * <code>optional string uuid = 2;</code>
     * @return The uuid.
     */
    public java.lang.String getUuid() {
      java.lang.Object ref = uuid_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        uuid_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The UUID of the user in the database. A service can have
     * this, but it's more likely for it to have the username.
     * </pre>
     *
     * <code>optional string uuid = 2;</code>
     * @return The bytes for uuid.
     */
    public com.google.protobuf.ByteString
        getUuidBytes() {
      java.lang.Object ref = uuid_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        uuid_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The UUID of the user in the database. A service can have
     * this, but it's more likely for it to have the username.
     * </pre>
     *
     * <code>optional string uuid = 2;</code>
     * @param value The uuid to set.
     * @return This builder for chaining.
     */
    public Builder setUuid(
        java.lang.String value) {
      if (value == null) {
    throw new NullPointerException();
  }
  bitField0_ |= 0x00000002;
      uuid_ = value;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The UUID of the user in the database. A service can have
     * this, but it's more likely for it to have the username.
     * </pre>
     *
     * <code>optional string uuid = 2;</code>
     * @return This builder for chaining.
     */
    public Builder clearUuid() {
      bitField0_ = (bitField0_ & ~0x00000002);
      uuid_ = getDefaultInstance().getUuid();
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The UUID of the user in the database. A service can have
     * this, but it's more likely for it to have the username.
     * </pre>
     *
     * <code>optional string uuid = 2;</code>
     * @param value The bytes for uuid to set.
     * @return This builder for chaining.
     */
    public Builder setUuidBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) {
    throw new NullPointerException();
  }
  checkByteStringIsUtf8(value);
      bitField0_ |= 0x00000002;
      uuid_ = value;
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


    // @@protoc_insertion_point(builder_scope:user.UserRef)
  }

  // @@protoc_insertion_point(class_scope:user.UserRef)
  private static final org.cyverse.de.protobufs.UserRef DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.UserRef();
  }

  public static org.cyverse.de.protobufs.UserRef getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<UserRef>
      PARSER = new com.google.protobuf.AbstractParser<UserRef>() {
    @java.lang.Override
    public UserRef parsePartialFrom(
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

  public static com.google.protobuf.Parser<UserRef> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<UserRef> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.UserRef getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

