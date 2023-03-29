// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: apps.proto

package org.cyverse.de.protobufs;

/**
 * <pre>
 **
 * Information about when something was integrated.
 * </pre>
 *
 * Protobuf type {@code apps.IntegrationData}
 */
public final class IntegrationData extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:apps.IntegrationData)
    IntegrationDataOrBuilder {
private static final long serialVersionUID = 0L;
  // Use IntegrationData.newBuilder() to construct.
  private IntegrationData(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private IntegrationData() {
    id_ = "";
    integratorName_ = "";
    integratorEmail_ = "";
  }

  @java.lang.Override
  @SuppressWarnings({"unused"})
  protected java.lang.Object newInstance(
      UnusedPrivateParameter unused) {
    return new IntegrationData();
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return this.unknownFields;
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.AppsProtobufs.internal_static_apps_IntegrationData_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.AppsProtobufs.internal_static_apps_IntegrationData_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.IntegrationData.class, org.cyverse.de.protobufs.IntegrationData.Builder.class);
  }

  public static final int ID_FIELD_NUMBER = 1;
  @SuppressWarnings("serial")
  private volatile java.lang.Object id_ = "";
  /**
   * <pre>
   * The UUID of the integration data.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The id.
   */
  @java.lang.Override
  public java.lang.String getId() {
    java.lang.Object ref = id_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      id_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The UUID of the integration data.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The bytes for id.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getIdBytes() {
    java.lang.Object ref = id_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      id_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int INTEGRATOR_NAME_FIELD_NUMBER = 2;
  @SuppressWarnings("serial")
  private volatile java.lang.Object integratorName_ = "";
  /**
   * <pre>
   * The name of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
   * @return The integratorName.
   */
  @java.lang.Override
  public java.lang.String getIntegratorName() {
    java.lang.Object ref = integratorName_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      integratorName_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The name of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
   * @return The bytes for integratorName.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getIntegratorNameBytes() {
    java.lang.Object ref = integratorName_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      integratorName_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int INTEGRATOR_EMAIL_FIELD_NUMBER = 3;
  @SuppressWarnings("serial")
  private volatile java.lang.Object integratorEmail_ = "";
  /**
   * <pre>
   * The email of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
   * @return The integratorEmail.
   */
  @java.lang.Override
  public java.lang.String getIntegratorEmail() {
    java.lang.Object ref = integratorEmail_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      integratorEmail_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The email of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
   * @return The bytes for integratorEmail.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getIntegratorEmailBytes() {
    java.lang.Object ref = integratorEmail_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      integratorEmail_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int USER_FIELD_NUMBER = 4;
  private org.cyverse.de.protobufs.User user_;
  /**
   * <pre>
   * The user information of the integrator.
   * </pre>
   *
   * <code>.user.User user = 4;</code>
   * @return Whether the user field is set.
   */
  @java.lang.Override
  public boolean hasUser() {
    return user_ != null;
  }
  /**
   * <pre>
   * The user information of the integrator.
   * </pre>
   *
   * <code>.user.User user = 4;</code>
   * @return The user.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.User getUser() {
    return user_ == null ? org.cyverse.de.protobufs.User.getDefaultInstance() : user_;
  }
  /**
   * <pre>
   * The user information of the integrator.
   * </pre>
   *
   * <code>.user.User user = 4;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.UserOrBuilder getUserOrBuilder() {
    return user_ == null ? org.cyverse.de.protobufs.User.getDefaultInstance() : user_;
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
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(id_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 1, id_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(integratorName_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 2, integratorName_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(integratorEmail_)) {
      com.google.protobuf.GeneratedMessageV3.writeString(output, 3, integratorEmail_);
    }
    if (user_ != null) {
      output.writeMessage(4, getUser());
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(id_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(1, id_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(integratorName_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(2, integratorName_);
    }
    if (!com.google.protobuf.GeneratedMessageV3.isStringEmpty(integratorEmail_)) {
      size += com.google.protobuf.GeneratedMessageV3.computeStringSize(3, integratorEmail_);
    }
    if (user_ != null) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(4, getUser());
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
    if (!(obj instanceof org.cyverse.de.protobufs.IntegrationData)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.IntegrationData other = (org.cyverse.de.protobufs.IntegrationData) obj;

    if (!getId()
        .equals(other.getId())) return false;
    if (!getIntegratorName()
        .equals(other.getIntegratorName())) return false;
    if (!getIntegratorEmail()
        .equals(other.getIntegratorEmail())) return false;
    if (hasUser() != other.hasUser()) return false;
    if (hasUser()) {
      if (!getUser()
          .equals(other.getUser())) return false;
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
    hash = (37 * hash) + ID_FIELD_NUMBER;
    hash = (53 * hash) + getId().hashCode();
    hash = (37 * hash) + INTEGRATOR_NAME_FIELD_NUMBER;
    hash = (53 * hash) + getIntegratorName().hashCode();
    hash = (37 * hash) + INTEGRATOR_EMAIL_FIELD_NUMBER;
    hash = (53 * hash) + getIntegratorEmail().hashCode();
    if (hasUser()) {
      hash = (37 * hash) + USER_FIELD_NUMBER;
      hash = (53 * hash) + getUser().hashCode();
    }
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.IntegrationData parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.IntegrationData prototype) {
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
   * Information about when something was integrated.
   * </pre>
   *
   * Protobuf type {@code apps.IntegrationData}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:apps.IntegrationData)
      org.cyverse.de.protobufs.IntegrationDataOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.AppsProtobufs.internal_static_apps_IntegrationData_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.AppsProtobufs.internal_static_apps_IntegrationData_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.IntegrationData.class, org.cyverse.de.protobufs.IntegrationData.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.IntegrationData.newBuilder()
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
      id_ = "";
      integratorName_ = "";
      integratorEmail_ = "";
      user_ = null;
      if (userBuilder_ != null) {
        userBuilder_.dispose();
        userBuilder_ = null;
      }
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.AppsProtobufs.internal_static_apps_IntegrationData_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.IntegrationData getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.IntegrationData.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.IntegrationData build() {
      org.cyverse.de.protobufs.IntegrationData result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.IntegrationData buildPartial() {
      org.cyverse.de.protobufs.IntegrationData result = new org.cyverse.de.protobufs.IntegrationData(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.IntegrationData result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.id_ = id_;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.integratorName_ = integratorName_;
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.integratorEmail_ = integratorEmail_;
      }
      if (((from_bitField0_ & 0x00000008) != 0)) {
        result.user_ = userBuilder_ == null
            ? user_
            : userBuilder_.build();
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
      if (other instanceof org.cyverse.de.protobufs.IntegrationData) {
        return mergeFrom((org.cyverse.de.protobufs.IntegrationData)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.IntegrationData other) {
      if (other == org.cyverse.de.protobufs.IntegrationData.getDefaultInstance()) return this;
      if (!other.getId().isEmpty()) {
        id_ = other.id_;
        bitField0_ |= 0x00000001;
        onChanged();
      }
      if (!other.getIntegratorName().isEmpty()) {
        integratorName_ = other.integratorName_;
        bitField0_ |= 0x00000002;
        onChanged();
      }
      if (!other.getIntegratorEmail().isEmpty()) {
        integratorEmail_ = other.integratorEmail_;
        bitField0_ |= 0x00000004;
        onChanged();
      }
      if (other.hasUser()) {
        mergeUser(other.getUser());
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
              id_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000001;
              break;
            } // case 10
            case 18: {
              integratorName_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 26: {
              integratorEmail_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000004;
              break;
            } // case 26
            case 34: {
              input.readMessage(
                  getUserFieldBuilder().getBuilder(),
                  extensionRegistry);
              bitField0_ |= 0x00000008;
              break;
            } // case 34
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

    private java.lang.Object id_ = "";
    /**
     * <pre>
     * The UUID of the integration data.
     * </pre>
     *
     * <code>string id = 1;</code>
     * @return The id.
     */
    public java.lang.String getId() {
      java.lang.Object ref = id_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        id_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The UUID of the integration data.
     * </pre>
     *
     * <code>string id = 1;</code>
     * @return The bytes for id.
     */
    public com.google.protobuf.ByteString
        getIdBytes() {
      java.lang.Object ref = id_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        id_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The UUID of the integration data.
     * </pre>
     *
     * <code>string id = 1;</code>
     * @param value The id to set.
     * @return This builder for chaining.
     */
    public Builder setId(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      id_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The UUID of the integration data.
     * </pre>
     *
     * <code>string id = 1;</code>
     * @return This builder for chaining.
     */
    public Builder clearId() {
      id_ = getDefaultInstance().getId();
      bitField0_ = (bitField0_ & ~0x00000001);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The UUID of the integration data.
     * </pre>
     *
     * <code>string id = 1;</code>
     * @param value The bytes for id to set.
     * @return This builder for chaining.
     */
    public Builder setIdBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      id_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }

    private java.lang.Object integratorName_ = "";
    /**
     * <pre>
     * The name of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
     * @return The integratorName.
     */
    public java.lang.String getIntegratorName() {
      java.lang.Object ref = integratorName_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        integratorName_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The name of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
     * @return The bytes for integratorName.
     */
    public com.google.protobuf.ByteString
        getIntegratorNameBytes() {
      java.lang.Object ref = integratorName_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        integratorName_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The name of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
     * @param value The integratorName to set.
     * @return This builder for chaining.
     */
    public Builder setIntegratorName(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      integratorName_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The name of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
     * @return This builder for chaining.
     */
    public Builder clearIntegratorName() {
      integratorName_ = getDefaultInstance().getIntegratorName();
      bitField0_ = (bitField0_ & ~0x00000002);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The name of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
     * @param value The bytes for integratorName to set.
     * @return This builder for chaining.
     */
    public Builder setIntegratorNameBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      integratorName_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }

    private java.lang.Object integratorEmail_ = "";
    /**
     * <pre>
     * The email of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
     * @return The integratorEmail.
     */
    public java.lang.String getIntegratorEmail() {
      java.lang.Object ref = integratorEmail_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        integratorEmail_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The email of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
     * @return The bytes for integratorEmail.
     */
    public com.google.protobuf.ByteString
        getIntegratorEmailBytes() {
      java.lang.Object ref = integratorEmail_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        integratorEmail_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The email of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
     * @param value The integratorEmail to set.
     * @return This builder for chaining.
     */
    public Builder setIntegratorEmail(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      integratorEmail_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The email of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
     * @return This builder for chaining.
     */
    public Builder clearIntegratorEmail() {
      integratorEmail_ = getDefaultInstance().getIntegratorEmail();
      bitField0_ = (bitField0_ & ~0x00000004);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The email of the person that integrated stuff.
     * </pre>
     *
     * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
     * @param value The bytes for integratorEmail to set.
     * @return This builder for chaining.
     */
    public Builder setIntegratorEmailBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      integratorEmail_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }

    private org.cyverse.de.protobufs.User user_;
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.User, org.cyverse.de.protobufs.User.Builder, org.cyverse.de.protobufs.UserOrBuilder> userBuilder_;
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     * @return Whether the user field is set.
     */
    public boolean hasUser() {
      return ((bitField0_ & 0x00000008) != 0);
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     * @return The user.
     */
    public org.cyverse.de.protobufs.User getUser() {
      if (userBuilder_ == null) {
        return user_ == null ? org.cyverse.de.protobufs.User.getDefaultInstance() : user_;
      } else {
        return userBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     */
    public Builder setUser(org.cyverse.de.protobufs.User value) {
      if (userBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        user_ = value;
      } else {
        userBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     */
    public Builder setUser(
        org.cyverse.de.protobufs.User.Builder builderForValue) {
      if (userBuilder_ == null) {
        user_ = builderForValue.build();
      } else {
        userBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     */
    public Builder mergeUser(org.cyverse.de.protobufs.User value) {
      if (userBuilder_ == null) {
        if (((bitField0_ & 0x00000008) != 0) &&
          user_ != null &&
          user_ != org.cyverse.de.protobufs.User.getDefaultInstance()) {
          getUserBuilder().mergeFrom(value);
        } else {
          user_ = value;
        }
      } else {
        userBuilder_.mergeFrom(value);
      }
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     */
    public Builder clearUser() {
      bitField0_ = (bitField0_ & ~0x00000008);
      user_ = null;
      if (userBuilder_ != null) {
        userBuilder_.dispose();
        userBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     */
    public org.cyverse.de.protobufs.User.Builder getUserBuilder() {
      bitField0_ |= 0x00000008;
      onChanged();
      return getUserFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     */
    public org.cyverse.de.protobufs.UserOrBuilder getUserOrBuilder() {
      if (userBuilder_ != null) {
        return userBuilder_.getMessageOrBuilder();
      } else {
        return user_ == null ?
            org.cyverse.de.protobufs.User.getDefaultInstance() : user_;
      }
    }
    /**
     * <pre>
     * The user information of the integrator.
     * </pre>
     *
     * <code>.user.User user = 4;</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        org.cyverse.de.protobufs.User, org.cyverse.de.protobufs.User.Builder, org.cyverse.de.protobufs.UserOrBuilder> 
        getUserFieldBuilder() {
      if (userBuilder_ == null) {
        userBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            org.cyverse.de.protobufs.User, org.cyverse.de.protobufs.User.Builder, org.cyverse.de.protobufs.UserOrBuilder>(
                getUser(),
                getParentForChildren(),
                isClean());
        user_ = null;
      }
      return userBuilder_;
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


    // @@protoc_insertion_point(builder_scope:apps.IntegrationData)
  }

  // @@protoc_insertion_point(class_scope:apps.IntegrationData)
  private static final org.cyverse.de.protobufs.IntegrationData DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.IntegrationData();
  }

  public static org.cyverse.de.protobufs.IntegrationData getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<IntegrationData>
      PARSER = new com.google.protobuf.AbstractParser<IntegrationData>() {
    @java.lang.Override
    public IntegrationData parsePartialFrom(
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

  public static com.google.protobuf.Parser<IntegrationData> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<IntegrationData> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.IntegrationData getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

