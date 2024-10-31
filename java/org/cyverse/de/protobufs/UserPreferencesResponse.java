// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: user_requests.proto

package org.cyverse.de.protobufs;

/**
 * <pre>
 * A response containing information about a user's preferences. Contains
 * user information (set according to what was in the request), preferences
 * information (if it's applicable for the request), and any errors
 * encountered while processing the request.
 * </pre>
 *
 * Protobuf type {@code user_requests.UserPreferencesResponse}
 */
public final class UserPreferencesResponse extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:user_requests.UserPreferencesResponse)
    UserPreferencesResponseOrBuilder {
private static final long serialVersionUID = 0L;
  // Use UserPreferencesResponse.newBuilder() to construct.
  private UserPreferencesResponse(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private UserPreferencesResponse() {
    preferences_ = "";
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new UserPreferencesResponse();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.UserRequestsProtobufs.internal_static_user_requests_UserPreferencesResponse_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.UserRequestsProtobufs.internal_static_user_requests_UserPreferencesResponse_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.UserPreferencesResponse.class, org.cyverse.de.protobufs.UserPreferencesResponse.Builder.class);
  }

  private int bitField0_;
  public static final int USER_FIELD_NUMBER = 1;
  private org.cyverse.de.protobufs.UserRef user_;
  /**
   * <code>optional .user.UserRef user = 1;</code>
   * @return Whether the user field is set.
   */
  @java.lang.Override
  public boolean hasUser() {
    return ((bitField0_ & 0x00000001) != 0);
  }
  /**
   * <code>optional .user.UserRef user = 1;</code>
   * @return The user.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.UserRef getUser() {
    return user_ == null ? org.cyverse.de.protobufs.UserRef.getDefaultInstance() : user_;
  }
  /**
   * <code>optional .user.UserRef user = 1;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.UserRefOrBuilder getUserOrBuilder() {
    return user_ == null ? org.cyverse.de.protobufs.UserRef.getDefaultInstance() : user_;
  }

  public static final int PREFERENCES_FIELD_NUMBER = 2;
  private volatile java.lang.Object preferences_;
  /**
   * <code>optional string preferences = 2;</code>
   * @return Whether the preferences field is set.
   */
  @java.lang.Override
  public boolean hasPreferences() {
    return ((bitField0_ & 0x00000002) != 0);
  }
  /**
   * <code>optional string preferences = 2;</code>
   * @return The preferences.
   */
  @java.lang.Override
  public java.lang.String getPreferences() {
    java.lang.Object ref = preferences_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      preferences_ = s;
      return s;
    }
  }
  /**
   * <code>optional string preferences = 2;</code>
   * @return The bytes for preferences.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getPreferencesBytes() {
    java.lang.Object ref = preferences_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      preferences_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int ERROR_FIELD_NUMBER = 3;
  private org.cyverse.de.protobufs.Error error_;
  /**
   * <code>optional .svcerror.Error error = 3;</code>
   * @return Whether the error field is set.
   */
  @java.lang.Override
  public boolean hasError() {
    return ((bitField0_ & 0x00000004) != 0);
  }
  /**
   * <code>optional .svcerror.Error error = 3;</code>
   * @return The error.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.Error getError() {
    return error_ == null ? org.cyverse.de.protobufs.Error.getDefaultInstance() : error_;
  }
  /**
   * <code>optional .svcerror.Error error = 3;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ErrorOrBuilder getErrorOrBuilder() {
    return error_ == null ? org.cyverse.de.protobufs.Error.getDefaultInstance() : error_;
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
      output.writeMessage(1, getUser());
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 2, preferences_);
    }
    if (((bitField0_ & 0x00000004) != 0)) {
      output.writeMessage(3, getError());
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (((bitField0_ & 0x00000001) != 0)) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(1, getUser());
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(2, preferences_);
    }
    if (((bitField0_ & 0x00000004) != 0)) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(3, getError());
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
    if (!(obj instanceof org.cyverse.de.protobufs.UserPreferencesResponse)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.UserPreferencesResponse other = (org.cyverse.de.protobufs.UserPreferencesResponse) obj;

    if (hasUser() != other.hasUser()) return false;
    if (hasUser()) {
      if (!getUser()
          .equals(other.getUser())) return false;
    }
    if (hasPreferences() != other.hasPreferences()) return false;
    if (hasPreferences()) {
      if (!getPreferences()
          .equals(other.getPreferences())) return false;
    }
    if (hasError() != other.hasError()) return false;
    if (hasError()) {
      if (!getError()
          .equals(other.getError())) return false;
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
    if (hasUser()) {
      hash = (37 * hash) + USER_FIELD_NUMBER;
      hash = (53 * hash) + getUser().hashCode();
    }
    if (hasPreferences()) {
      hash = (37 * hash) + PREFERENCES_FIELD_NUMBER;
      hash = (53 * hash) + getPreferences().hashCode();
    }
    if (hasError()) {
      hash = (37 * hash) + ERROR_FIELD_NUMBER;
      hash = (53 * hash) + getError().hashCode();
    }
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UserPreferencesResponse parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.UserPreferencesResponse prototype) {
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
   * A response containing information about a user's preferences. Contains
   * user information (set according to what was in the request), preferences
   * information (if it's applicable for the request), and any errors
   * encountered while processing the request.
   * </pre>
   *
   * Protobuf type {@code user_requests.UserPreferencesResponse}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:user_requests.UserPreferencesResponse)
      org.cyverse.de.protobufs.UserPreferencesResponseOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.UserRequestsProtobufs.internal_static_user_requests_UserPreferencesResponse_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.UserRequestsProtobufs.internal_static_user_requests_UserPreferencesResponse_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.UserPreferencesResponse.class, org.cyverse.de.protobufs.UserPreferencesResponse.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.UserPreferencesResponse.newBuilder()
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
        getUserFieldBuilder();
        getErrorFieldBuilder();
      }
    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      if (userBuilder_ == null) {
        user_ = null;
      } else {
        userBuilder_.clear();
      }
      bitField0_ = (bitField0_ & ~0x00000001);
      preferences_ = "";
      bitField0_ = (bitField0_ & ~0x00000002);
      if (errorBuilder_ == null) {
        error_ = null;
      } else {
        errorBuilder_.clear();
      }
      bitField0_ = (bitField0_ & ~0x00000004);
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.UserRequestsProtobufs.internal_static_user_requests_UserPreferencesResponse_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UserPreferencesResponse getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.UserPreferencesResponse.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UserPreferencesResponse build() {
      org.cyverse.de.protobufs.UserPreferencesResponse result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UserPreferencesResponse buildPartial() {
      org.cyverse.de.protobufs.UserPreferencesResponse result = new org.cyverse.de.protobufs.UserPreferencesResponse(this);
      int from_bitField0_ = bitField0_;
      int to_bitField0_ = 0;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        if (userBuilder_ == null) {
          result.user_ = user_;
        } else {
          result.user_ = userBuilder_.build();
        }
        to_bitField0_ |= 0x00000001;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        to_bitField0_ |= 0x00000002;
      }
      result.preferences_ = preferences_;
      if (((from_bitField0_ & 0x00000004) != 0)) {
        if (errorBuilder_ == null) {
          result.error_ = error_;
        } else {
          result.error_ = errorBuilder_.build();
        }
        to_bitField0_ |= 0x00000004;
      }
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
      if (other instanceof org.cyverse.de.protobufs.UserPreferencesResponse) {
        return mergeFrom((org.cyverse.de.protobufs.UserPreferencesResponse)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.UserPreferencesResponse other) {
      if (other == org.cyverse.de.protobufs.UserPreferencesResponse.getDefaultInstance()) return this;
      if (other.hasUser()) {
        mergeUser(other.getUser());
      }
      if (other.hasPreferences()) {
        bitField0_ |= 0x00000002;
        preferences_ = other.preferences_;
        onChanged();
      }
      if (other.hasError()) {
        mergeError(other.getError());
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
                  getUserFieldBuilder().getBuilder(),
                  extensionRegistry);
              bitField0_ |= 0x00000001;
              break;
            } // case 10
            case 18: {
              preferences_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 26: {
              input.readMessage(
                  getErrorFieldBuilder().getBuilder(),
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

    private org.cyverse.de.protobufs.UserRef user_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.UserRef, org.cyverse.de.protobufs.UserRef.Builder, org.cyverse.de.protobufs.UserRefOrBuilder> userBuilder_;
    /**
     * <code>optional .user.UserRef user = 1;</code>
     * @return Whether the user field is set.
     */
    public boolean hasUser() {
      return ((bitField0_ & 0x00000001) != 0);
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     * @return The user.
     */
    public org.cyverse.de.protobufs.UserRef getUser() {
      if (userBuilder_ == null) {
        return user_ == null ? org.cyverse.de.protobufs.UserRef.getDefaultInstance() : user_;
      } else {
        return userBuilder_.getMessage();
      }
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     */
    public Builder setUser(org.cyverse.de.protobufs.UserRef value) {
      if (userBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        user_ = value;
        onChanged();
      } else {
        userBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000001;
      return this;
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     */
    public Builder setUser(
        org.cyverse.de.protobufs.UserRef.Builder builderForValue) {
      if (userBuilder_ == null) {
        user_ = builderForValue.build();
        onChanged();
      } else {
        userBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000001;
      return this;
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     */
    public Builder mergeUser(org.cyverse.de.protobufs.UserRef value) {
      if (userBuilder_ == null) {
        if (((bitField0_ & 0x00000001) != 0) &&
            user_ != null &&
            user_ != org.cyverse.de.protobufs.UserRef.getDefaultInstance()) {
          user_ =
            org.cyverse.de.protobufs.UserRef.newBuilder(user_).mergeFrom(value).buildPartial();
        } else {
          user_ = value;
        }
        onChanged();
      } else {
        userBuilder_.mergeFrom(value);
      }
      bitField0_ |= 0x00000001;
      return this;
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     */
    public Builder clearUser() {
      if (userBuilder_ == null) {
        user_ = null;
        onChanged();
      } else {
        userBuilder_.clear();
      }
      bitField0_ = (bitField0_ & ~0x00000001);
      return this;
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     */
    public org.cyverse.de.protobufs.UserRef.Builder getUserBuilder() {
      bitField0_ |= 0x00000001;
      onChanged();
      return getUserFieldBuilder().getBuilder();
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     */
    public org.cyverse.de.protobufs.UserRefOrBuilder getUserOrBuilder() {
      if (userBuilder_ != null) {
        return userBuilder_.getMessageOrBuilder();
      } else {
        return user_ == null ?
            org.cyverse.de.protobufs.UserRef.getDefaultInstance() : user_;
      }
    }
    /**
     * <code>optional .user.UserRef user = 1;</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.UserRef, org.cyverse.de.protobufs.UserRef.Builder, org.cyverse.de.protobufs.UserRefOrBuilder> 
        getUserFieldBuilder() {
      if (userBuilder_ == null) {
        userBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            org.cyverse.de.protobufs.UserRef, org.cyverse.de.protobufs.UserRef.Builder, org.cyverse.de.protobufs.UserRefOrBuilder>(
                getUser(),
                getParentForChildren(),
                isClean());
        user_ = null;
      }
      return userBuilder_;
    }

    private java.lang.Object preferences_ = "";
    /**
     * <code>optional string preferences = 2;</code>
     * @return Whether the preferences field is set.
     */
    public boolean hasPreferences() {
      return ((bitField0_ & 0x00000002) != 0);
    }
    /**
     * <code>optional string preferences = 2;</code>
     * @return The preferences.
     */
    public java.lang.String getPreferences() {
      java.lang.Object ref = preferences_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        preferences_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <code>optional string preferences = 2;</code>
     * @return The bytes for preferences.
     */
    public com.google.protobuf.ByteString
        getPreferencesBytes() {
      java.lang.Object ref = preferences_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        preferences_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <code>optional string preferences = 2;</code>
     * @param value The preferences to set.
     * @return This builder for chaining.
     */
    public Builder setPreferences(
        java.lang.String value) {
      if (value == null) {
    throw new NullPointerException();
  }
  bitField0_ |= 0x00000002;
      preferences_ = value;
      onChanged();
      return this;
    }
    /**
     * <code>optional string preferences = 2;</code>
     * @return This builder for chaining.
     */
    public Builder clearPreferences() {
      bitField0_ = (bitField0_ & ~0x00000002);
      preferences_ = getDefaultInstance().getPreferences();
      onChanged();
      return this;
    }
    /**
     * <code>optional string preferences = 2;</code>
     * @param value The bytes for preferences to set.
     * @return This builder for chaining.
     */
    public Builder setPreferencesBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) {
    throw new NullPointerException();
  }
  checkByteStringIsUtf8(value);
      bitField0_ |= 0x00000002;
      preferences_ = value;
      onChanged();
      return this;
    }

    private org.cyverse.de.protobufs.Error error_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Error, org.cyverse.de.protobufs.Error.Builder, org.cyverse.de.protobufs.ErrorOrBuilder> errorBuilder_;
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     * @return Whether the error field is set.
     */
    public boolean hasError() {
      return ((bitField0_ & 0x00000004) != 0);
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     * @return The error.
     */
    public org.cyverse.de.protobufs.Error getError() {
      if (errorBuilder_ == null) {
        return error_ == null ? org.cyverse.de.protobufs.Error.getDefaultInstance() : error_;
      } else {
        return errorBuilder_.getMessage();
      }
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     */
    public Builder setError(org.cyverse.de.protobufs.Error value) {
      if (errorBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        error_ = value;
        onChanged();
      } else {
        errorBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000004;
      return this;
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     */
    public Builder setError(
        org.cyverse.de.protobufs.Error.Builder builderForValue) {
      if (errorBuilder_ == null) {
        error_ = builderForValue.build();
        onChanged();
      } else {
        errorBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000004;
      return this;
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     */
    public Builder mergeError(org.cyverse.de.protobufs.Error value) {
      if (errorBuilder_ == null) {
        if (((bitField0_ & 0x00000004) != 0) &&
            error_ != null &&
            error_ != org.cyverse.de.protobufs.Error.getDefaultInstance()) {
          error_ =
            org.cyverse.de.protobufs.Error.newBuilder(error_).mergeFrom(value).buildPartial();
        } else {
          error_ = value;
        }
        onChanged();
      } else {
        errorBuilder_.mergeFrom(value);
      }
      bitField0_ |= 0x00000004;
      return this;
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     */
    public Builder clearError() {
      if (errorBuilder_ == null) {
        error_ = null;
        onChanged();
      } else {
        errorBuilder_.clear();
      }
      bitField0_ = (bitField0_ & ~0x00000004);
      return this;
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     */
    public org.cyverse.de.protobufs.Error.Builder getErrorBuilder() {
      bitField0_ |= 0x00000004;
      onChanged();
      return getErrorFieldBuilder().getBuilder();
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     */
    public org.cyverse.de.protobufs.ErrorOrBuilder getErrorOrBuilder() {
      if (errorBuilder_ != null) {
        return errorBuilder_.getMessageOrBuilder();
      } else {
        return error_ == null ?
            org.cyverse.de.protobufs.Error.getDefaultInstance() : error_;
      }
    }
    /**
     * <code>optional .svcerror.Error error = 3;</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.Error, org.cyverse.de.protobufs.Error.Builder, org.cyverse.de.protobufs.ErrorOrBuilder> 
        getErrorFieldBuilder() {
      if (errorBuilder_ == null) {
        errorBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            org.cyverse.de.protobufs.Error, org.cyverse.de.protobufs.Error.Builder, org.cyverse.de.protobufs.ErrorOrBuilder>(
                getError(),
                getParentForChildren(),
                isClean());
        error_ = null;
      }
      return errorBuilder_;
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


    // @@protoc_insertion_point(builder_scope:user_requests.UserPreferencesResponse)
  }

  // @@protoc_insertion_point(class_scope:user_requests.UserPreferencesResponse)
  private static final org.cyverse.de.protobufs.UserPreferencesResponse DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.UserPreferencesResponse();
  }

  public static org.cyverse.de.protobufs.UserPreferencesResponse getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<UserPreferencesResponse>
      PARSER = new com.google.protobuf.AbstractParser<UserPreferencesResponse>() {
    @java.lang.Override
    public UserPreferencesResponse parsePartialFrom(
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

  public static com.google.protobuf.Parser<UserPreferencesResponse> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<UserPreferencesResponse> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.UserPreferencesResponse getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

