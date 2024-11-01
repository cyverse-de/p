// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_addons.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

/**
 * <pre>
 * *
 * Represents an add-on that can be applied to a subscription to provide a user
 * with a change in a resource quota.
 * </pre>
 *
 * Protobuf type {@code qms.Addon}
 */
public final class Addon extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:qms.Addon)
    AddonOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 28,
      /* patch= */ 3,
      /* suffix= */ "",
      Addon.class.getName());
  }
  // Use Addon.newBuilder() to construct.
  private Addon(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private Addon() {
    uuid_ = "";
    name_ = "";
    description_ = "";
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_Addon_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_Addon_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.Addon.class, org.cyverse.de.protobufs.Addon.Builder.class);
  }

  private int bitField0_;
  public static final int UUID_FIELD_NUMBER = 1;
  @SuppressWarnings("serial")
  private volatile java.lang.Object uuid_ = "";
  /**
   * <pre>
   * The unique identifier.
   * </pre>
   *
   * <code>string uuid = 1;</code>
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
   * The unique identifier.
   * </pre>
   *
   * <code>string uuid = 1;</code>
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

  public static final int NAME_FIELD_NUMBER = 2;
  @SuppressWarnings("serial")
  private volatile java.lang.Object name_ = "";
  /**
   * <pre>
   * The name of the add-on.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The name.
   */
  @java.lang.Override
  public java.lang.String getName() {
    java.lang.Object ref = name_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      name_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The name of the add-on.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The bytes for name.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getNameBytes() {
    java.lang.Object ref = name_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      name_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int DESCRIPTION_FIELD_NUMBER = 3;
  @SuppressWarnings("serial")
  private volatile java.lang.Object description_ = "";
  /**
   * <pre>
   * The description of the add-on.
   * </pre>
   *
   * <code>string description = 3;</code>
   * @return The description.
   */
  @java.lang.Override
  public java.lang.String getDescription() {
    java.lang.Object ref = description_;
    if (ref instanceof java.lang.String) {
      return (java.lang.String) ref;
    } else {
      com.google.protobuf.ByteString bs = 
          (com.google.protobuf.ByteString) ref;
      java.lang.String s = bs.toStringUtf8();
      description_ = s;
      return s;
    }
  }
  /**
   * <pre>
   * The description of the add-on.
   * </pre>
   *
   * <code>string description = 3;</code>
   * @return The bytes for description.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getDescriptionBytes() {
    java.lang.Object ref = description_;
    if (ref instanceof java.lang.String) {
      com.google.protobuf.ByteString b = 
          com.google.protobuf.ByteString.copyFromUtf8(
              (java.lang.String) ref);
      description_ = b;
      return b;
    } else {
      return (com.google.protobuf.ByteString) ref;
    }
  }

  public static final int RESOURCE_TYPE_FIELD_NUMBER = 4;
  private org.cyverse.de.protobufs.ResourceType resourceType_;
  /**
   * <pre>
   * The resource type of the add-on.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   * @return Whether the resourceType field is set.
   */
  @java.lang.Override
  public boolean hasResourceType() {
    return ((bitField0_ & 0x00000001) != 0);
  }
  /**
   * <pre>
   * The resource type of the add-on.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   * @return The resourceType.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceType getResourceType() {
    return resourceType_ == null ? org.cyverse.de.protobufs.ResourceType.getDefaultInstance() : resourceType_;
  }
  /**
   * <pre>
   * The resource type of the add-on.
   * </pre>
   *
   * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypeOrBuilder() {
    return resourceType_ == null ? org.cyverse.de.protobufs.ResourceType.getDefaultInstance() : resourceType_;
  }

  public static final int DEFAULT_AMOUNT_FIELD_NUMBER = 5;
  private double defaultAmount_ = 0D;
  /**
   * <pre>
   * How much of the resource type is added to the quota by the add-on.
   * </pre>
   *
   * <code>double default_amount = 5 [json_name = "default_amount"];</code>
   * @return The defaultAmount.
   */
  @java.lang.Override
  public double getDefaultAmount() {
    return defaultAmount_;
  }

  public static final int DEFAULT_PAID_FIELD_NUMBER = 6;
  private boolean defaultPaid_ = false;
  /**
   * <pre>
   * Whether a user must pay for the add-on. Not whether the user has paid.
   * </pre>
   *
   * <code>bool default_paid = 6 [json_name = "default_paid"];</code>
   * @return The defaultPaid.
   */
  @java.lang.Override
  public boolean getDefaultPaid() {
    return defaultPaid_;
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
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(uuid_)) {
      com.google.protobuf.GeneratedMessage.writeString(output, 1, uuid_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(name_)) {
      com.google.protobuf.GeneratedMessage.writeString(output, 2, name_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(description_)) {
      com.google.protobuf.GeneratedMessage.writeString(output, 3, description_);
    }
    if (((bitField0_ & 0x00000001) != 0)) {
      output.writeMessage(4, getResourceType());
    }
    if (java.lang.Double.doubleToRawLongBits(defaultAmount_) != 0) {
      output.writeDouble(5, defaultAmount_);
    }
    if (defaultPaid_ != false) {
      output.writeBool(6, defaultPaid_);
    }
    getUnknownFields().writeTo(output);
  }

  @java.lang.Override
  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(uuid_)) {
      size += com.google.protobuf.GeneratedMessage.computeStringSize(1, uuid_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(name_)) {
      size += com.google.protobuf.GeneratedMessage.computeStringSize(2, name_);
    }
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(description_)) {
      size += com.google.protobuf.GeneratedMessage.computeStringSize(3, description_);
    }
    if (((bitField0_ & 0x00000001) != 0)) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(4, getResourceType());
    }
    if (java.lang.Double.doubleToRawLongBits(defaultAmount_) != 0) {
      size += com.google.protobuf.CodedOutputStream
        .computeDoubleSize(5, defaultAmount_);
    }
    if (defaultPaid_ != false) {
      size += com.google.protobuf.CodedOutputStream
        .computeBoolSize(6, defaultPaid_);
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
    if (!(obj instanceof org.cyverse.de.protobufs.Addon)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.Addon other = (org.cyverse.de.protobufs.Addon) obj;

    if (!getUuid()
        .equals(other.getUuid())) return false;
    if (!getName()
        .equals(other.getName())) return false;
    if (!getDescription()
        .equals(other.getDescription())) return false;
    if (hasResourceType() != other.hasResourceType()) return false;
    if (hasResourceType()) {
      if (!getResourceType()
          .equals(other.getResourceType())) return false;
    }
    if (java.lang.Double.doubleToLongBits(getDefaultAmount())
        != java.lang.Double.doubleToLongBits(
            other.getDefaultAmount())) return false;
    if (getDefaultPaid()
        != other.getDefaultPaid()) return false;
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
    hash = (37 * hash) + UUID_FIELD_NUMBER;
    hash = (53 * hash) + getUuid().hashCode();
    hash = (37 * hash) + NAME_FIELD_NUMBER;
    hash = (53 * hash) + getName().hashCode();
    hash = (37 * hash) + DESCRIPTION_FIELD_NUMBER;
    hash = (53 * hash) + getDescription().hashCode();
    if (hasResourceType()) {
      hash = (37 * hash) + RESOURCE_TYPE_FIELD_NUMBER;
      hash = (53 * hash) + getResourceType().hashCode();
    }
    hash = (37 * hash) + DEFAULT_AMOUNT_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashLong(
        java.lang.Double.doubleToLongBits(getDefaultAmount()));
    hash = (37 * hash) + DEFAULT_PAID_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashBoolean(
        getDefaultPaid());
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.Addon parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static org.cyverse.de.protobufs.Addon parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static org.cyverse.de.protobufs.Addon parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.Addon parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.Addon prototype) {
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
   * Represents an add-on that can be applied to a subscription to provide a user
   * with a change in a resource quota.
   * </pre>
   *
   * Protobuf type {@code qms.Addon}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:qms.Addon)
      org.cyverse.de.protobufs.AddonOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_Addon_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_Addon_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.Addon.class, org.cyverse.de.protobufs.Addon.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.Addon.newBuilder()
    private Builder() {
      maybeForceBuilderInitialization();
    }

    private Builder(
        com.google.protobuf.GeneratedMessage.BuilderParent parent) {
      super(parent);
      maybeForceBuilderInitialization();
    }
    private void maybeForceBuilderInitialization() {
      if (com.google.protobuf.GeneratedMessage
              .alwaysUseFieldBuilders) {
        getResourceTypeFieldBuilder();
      }
    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      bitField0_ = 0;
      uuid_ = "";
      name_ = "";
      description_ = "";
      resourceType_ = null;
      if (resourceTypeBuilder_ != null) {
        resourceTypeBuilder_.dispose();
        resourceTypeBuilder_ = null;
      }
      defaultAmount_ = 0D;
      defaultPaid_ = false;
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_Addon_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Addon getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.Addon.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Addon build() {
      org.cyverse.de.protobufs.Addon result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Addon buildPartial() {
      org.cyverse.de.protobufs.Addon result = new org.cyverse.de.protobufs.Addon(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.Addon result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.uuid_ = uuid_;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.name_ = name_;
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.description_ = description_;
      }
      int to_bitField0_ = 0;
      if (((from_bitField0_ & 0x00000008) != 0)) {
        result.resourceType_ = resourceTypeBuilder_ == null
            ? resourceType_
            : resourceTypeBuilder_.build();
        to_bitField0_ |= 0x00000001;
      }
      if (((from_bitField0_ & 0x00000010) != 0)) {
        result.defaultAmount_ = defaultAmount_;
      }
      if (((from_bitField0_ & 0x00000020) != 0)) {
        result.defaultPaid_ = defaultPaid_;
      }
      result.bitField0_ |= to_bitField0_;
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.Addon) {
        return mergeFrom((org.cyverse.de.protobufs.Addon)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.Addon other) {
      if (other == org.cyverse.de.protobufs.Addon.getDefaultInstance()) return this;
      if (!other.getUuid().isEmpty()) {
        uuid_ = other.uuid_;
        bitField0_ |= 0x00000001;
        onChanged();
      }
      if (!other.getName().isEmpty()) {
        name_ = other.name_;
        bitField0_ |= 0x00000002;
        onChanged();
      }
      if (!other.getDescription().isEmpty()) {
        description_ = other.description_;
        bitField0_ |= 0x00000004;
        onChanged();
      }
      if (other.hasResourceType()) {
        mergeResourceType(other.getResourceType());
      }
      if (other.getDefaultAmount() != 0D) {
        setDefaultAmount(other.getDefaultAmount());
      }
      if (other.getDefaultPaid() != false) {
        setDefaultPaid(other.getDefaultPaid());
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
              uuid_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000001;
              break;
            } // case 10
            case 18: {
              name_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 26: {
              description_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000004;
              break;
            } // case 26
            case 34: {
              input.readMessage(
                  getResourceTypeFieldBuilder().getBuilder(),
                  extensionRegistry);
              bitField0_ |= 0x00000008;
              break;
            } // case 34
            case 41: {
              defaultAmount_ = input.readDouble();
              bitField0_ |= 0x00000010;
              break;
            } // case 41
            case 48: {
              defaultPaid_ = input.readBool();
              bitField0_ |= 0x00000020;
              break;
            } // case 48
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

    private java.lang.Object uuid_ = "";
    /**
     * <pre>
     * The unique identifier.
     * </pre>
     *
     * <code>string uuid = 1;</code>
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
     * The unique identifier.
     * </pre>
     *
     * <code>string uuid = 1;</code>
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
     * The unique identifier.
     * </pre>
     *
     * <code>string uuid = 1;</code>
     * @param value The uuid to set.
     * @return This builder for chaining.
     */
    public Builder setUuid(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      uuid_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The unique identifier.
     * </pre>
     *
     * <code>string uuid = 1;</code>
     * @return This builder for chaining.
     */
    public Builder clearUuid() {
      uuid_ = getDefaultInstance().getUuid();
      bitField0_ = (bitField0_ & ~0x00000001);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The unique identifier.
     * </pre>
     *
     * <code>string uuid = 1;</code>
     * @param value The bytes for uuid to set.
     * @return This builder for chaining.
     */
    public Builder setUuidBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      uuid_ = value;
      bitField0_ |= 0x00000001;
      onChanged();
      return this;
    }

    private java.lang.Object name_ = "";
    /**
     * <pre>
     * The name of the add-on.
     * </pre>
     *
     * <code>string name = 2;</code>
     * @return The name.
     */
    public java.lang.String getName() {
      java.lang.Object ref = name_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        name_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The name of the add-on.
     * </pre>
     *
     * <code>string name = 2;</code>
     * @return The bytes for name.
     */
    public com.google.protobuf.ByteString
        getNameBytes() {
      java.lang.Object ref = name_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        name_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The name of the add-on.
     * </pre>
     *
     * <code>string name = 2;</code>
     * @param value The name to set.
     * @return This builder for chaining.
     */
    public Builder setName(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      name_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The name of the add-on.
     * </pre>
     *
     * <code>string name = 2;</code>
     * @return This builder for chaining.
     */
    public Builder clearName() {
      name_ = getDefaultInstance().getName();
      bitField0_ = (bitField0_ & ~0x00000002);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The name of the add-on.
     * </pre>
     *
     * <code>string name = 2;</code>
     * @param value The bytes for name to set.
     * @return This builder for chaining.
     */
    public Builder setNameBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      name_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }

    private java.lang.Object description_ = "";
    /**
     * <pre>
     * The description of the add-on.
     * </pre>
     *
     * <code>string description = 3;</code>
     * @return The description.
     */
    public java.lang.String getDescription() {
      java.lang.Object ref = description_;
      if (!(ref instanceof java.lang.String)) {
        com.google.protobuf.ByteString bs =
            (com.google.protobuf.ByteString) ref;
        java.lang.String s = bs.toStringUtf8();
        description_ = s;
        return s;
      } else {
        return (java.lang.String) ref;
      }
    }
    /**
     * <pre>
     * The description of the add-on.
     * </pre>
     *
     * <code>string description = 3;</code>
     * @return The bytes for description.
     */
    public com.google.protobuf.ByteString
        getDescriptionBytes() {
      java.lang.Object ref = description_;
      if (ref instanceof String) {
        com.google.protobuf.ByteString b = 
            com.google.protobuf.ByteString.copyFromUtf8(
                (java.lang.String) ref);
        description_ = b;
        return b;
      } else {
        return (com.google.protobuf.ByteString) ref;
      }
    }
    /**
     * <pre>
     * The description of the add-on.
     * </pre>
     *
     * <code>string description = 3;</code>
     * @param value The description to set.
     * @return This builder for chaining.
     */
    public Builder setDescription(
        java.lang.String value) {
      if (value == null) { throw new NullPointerException(); }
      description_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The description of the add-on.
     * </pre>
     *
     * <code>string description = 3;</code>
     * @return This builder for chaining.
     */
    public Builder clearDescription() {
      description_ = getDefaultInstance().getDescription();
      bitField0_ = (bitField0_ & ~0x00000004);
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The description of the add-on.
     * </pre>
     *
     * <code>string description = 3;</code>
     * @param value The bytes for description to set.
     * @return This builder for chaining.
     */
    public Builder setDescriptionBytes(
        com.google.protobuf.ByteString value) {
      if (value == null) { throw new NullPointerException(); }
      checkByteStringIsUtf8(value);
      description_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }

    private org.cyverse.de.protobufs.ResourceType resourceType_;
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.ResourceType, org.cyverse.de.protobufs.ResourceType.Builder, org.cyverse.de.protobufs.ResourceTypeOrBuilder> resourceTypeBuilder_;
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     * @return Whether the resourceType field is set.
     */
    public boolean hasResourceType() {
      return ((bitField0_ & 0x00000008) != 0);
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     * @return The resourceType.
     */
    public org.cyverse.de.protobufs.ResourceType getResourceType() {
      if (resourceTypeBuilder_ == null) {
        return resourceType_ == null ? org.cyverse.de.protobufs.ResourceType.getDefaultInstance() : resourceType_;
      } else {
        return resourceTypeBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     */
    public Builder setResourceType(org.cyverse.de.protobufs.ResourceType value) {
      if (resourceTypeBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        resourceType_ = value;
      } else {
        resourceTypeBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     */
    public Builder setResourceType(
        org.cyverse.de.protobufs.ResourceType.Builder builderForValue) {
      if (resourceTypeBuilder_ == null) {
        resourceType_ = builderForValue.build();
      } else {
        resourceTypeBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     */
    public Builder mergeResourceType(org.cyverse.de.protobufs.ResourceType value) {
      if (resourceTypeBuilder_ == null) {
        if (((bitField0_ & 0x00000008) != 0) &&
          resourceType_ != null &&
          resourceType_ != org.cyverse.de.protobufs.ResourceType.getDefaultInstance()) {
          getResourceTypeBuilder().mergeFrom(value);
        } else {
          resourceType_ = value;
        }
      } else {
        resourceTypeBuilder_.mergeFrom(value);
      }
      if (resourceType_ != null) {
        bitField0_ |= 0x00000008;
        onChanged();
      }
      return this;
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     */
    public Builder clearResourceType() {
      bitField0_ = (bitField0_ & ~0x00000008);
      resourceType_ = null;
      if (resourceTypeBuilder_ != null) {
        resourceTypeBuilder_.dispose();
        resourceTypeBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     */
    public org.cyverse.de.protobufs.ResourceType.Builder getResourceTypeBuilder() {
      bitField0_ |= 0x00000008;
      onChanged();
      return getResourceTypeFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     */
    public org.cyverse.de.protobufs.ResourceTypeOrBuilder getResourceTypeOrBuilder() {
      if (resourceTypeBuilder_ != null) {
        return resourceTypeBuilder_.getMessageOrBuilder();
      } else {
        return resourceType_ == null ?
            org.cyverse.de.protobufs.ResourceType.getDefaultInstance() : resourceType_;
      }
    }
    /**
     * <pre>
     * The resource type of the add-on.
     * </pre>
     *
     * <code>.qms.ResourceType resource_type = 4 [json_name = "resource_type"];</code>
     */
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.ResourceType, org.cyverse.de.protobufs.ResourceType.Builder, org.cyverse.de.protobufs.ResourceTypeOrBuilder> 
        getResourceTypeFieldBuilder() {
      if (resourceTypeBuilder_ == null) {
        resourceTypeBuilder_ = new com.google.protobuf.SingleFieldBuilder<
            org.cyverse.de.protobufs.ResourceType, org.cyverse.de.protobufs.ResourceType.Builder, org.cyverse.de.protobufs.ResourceTypeOrBuilder>(
                getResourceType(),
                getParentForChildren(),
                isClean());
        resourceType_ = null;
      }
      return resourceTypeBuilder_;
    }

    private double defaultAmount_ ;
    /**
     * <pre>
     * How much of the resource type is added to the quota by the add-on.
     * </pre>
     *
     * <code>double default_amount = 5 [json_name = "default_amount"];</code>
     * @return The defaultAmount.
     */
    @java.lang.Override
    public double getDefaultAmount() {
      return defaultAmount_;
    }
    /**
     * <pre>
     * How much of the resource type is added to the quota by the add-on.
     * </pre>
     *
     * <code>double default_amount = 5 [json_name = "default_amount"];</code>
     * @param value The defaultAmount to set.
     * @return This builder for chaining.
     */
    public Builder setDefaultAmount(double value) {

      defaultAmount_ = value;
      bitField0_ |= 0x00000010;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * How much of the resource type is added to the quota by the add-on.
     * </pre>
     *
     * <code>double default_amount = 5 [json_name = "default_amount"];</code>
     * @return This builder for chaining.
     */
    public Builder clearDefaultAmount() {
      bitField0_ = (bitField0_ & ~0x00000010);
      defaultAmount_ = 0D;
      onChanged();
      return this;
    }

    private boolean defaultPaid_ ;
    /**
     * <pre>
     * Whether a user must pay for the add-on. Not whether the user has paid.
     * </pre>
     *
     * <code>bool default_paid = 6 [json_name = "default_paid"];</code>
     * @return The defaultPaid.
     */
    @java.lang.Override
    public boolean getDefaultPaid() {
      return defaultPaid_;
    }
    /**
     * <pre>
     * Whether a user must pay for the add-on. Not whether the user has paid.
     * </pre>
     *
     * <code>bool default_paid = 6 [json_name = "default_paid"];</code>
     * @param value The defaultPaid to set.
     * @return This builder for chaining.
     */
    public Builder setDefaultPaid(boolean value) {

      defaultPaid_ = value;
      bitField0_ |= 0x00000020;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Whether a user must pay for the add-on. Not whether the user has paid.
     * </pre>
     *
     * <code>bool default_paid = 6 [json_name = "default_paid"];</code>
     * @return This builder for chaining.
     */
    public Builder clearDefaultPaid() {
      bitField0_ = (bitField0_ & ~0x00000020);
      defaultPaid_ = false;
      onChanged();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:qms.Addon)
  }

  // @@protoc_insertion_point(class_scope:qms.Addon)
  private static final org.cyverse.de.protobufs.Addon DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.Addon();
  }

  public static org.cyverse.de.protobufs.Addon getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<Addon>
      PARSER = new com.google.protobuf.AbstractParser<Addon>() {
    @java.lang.Override
    public Addon parsePartialFrom(
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

  public static com.google.protobuf.Parser<Addon> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<Addon> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.Addon getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

