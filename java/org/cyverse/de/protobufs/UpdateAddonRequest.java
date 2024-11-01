// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_addons.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

/**
 * <pre>
 * *
 * A request to update an add-on. The boolean fields are needed because Go (and
 * probably other languages) needs a way to tell when to set a field to an empty
 * string, since that's the zero value for a string.
 * </pre>
 *
 * Protobuf type {@code qms.UpdateAddonRequest}
 */
public final class UpdateAddonRequest extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:qms.UpdateAddonRequest)
    UpdateAddonRequestOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 28,
      /* patch= */ 3,
      /* suffix= */ "",
      UpdateAddonRequest.class.getName());
  }
  // Use UpdateAddonRequest.newBuilder() to construct.
  private UpdateAddonRequest(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private UpdateAddonRequest() {
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_UpdateAddonRequest_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_UpdateAddonRequest_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.UpdateAddonRequest.class, org.cyverse.de.protobufs.UpdateAddonRequest.Builder.class);
  }

  private int bitField0_;
  public static final int HEADER_FIELD_NUMBER = 1;
  private org.cyverse.de.protobufs.Header header_;
  /**
   * <pre>
   * Contains telemetry information.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  @java.lang.Override
  public boolean hasHeader() {
    return ((bitField0_ & 0x00000001) != 0);
  }
  /**
   * <pre>
   * Contains telemetry information.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.Header getHeader() {
    return header_ == null ? org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
  }
  /**
   * <pre>
   * Contains telemetry information.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder() {
    return header_ == null ? org.cyverse.de.protobufs.Header.getDefaultInstance() : header_;
  }

  public static final int ADDON_FIELD_NUMBER = 2;
  private org.cyverse.de.protobufs.Addon addon_;
  /**
   * <pre>
   * The values to set in the update.
   * </pre>
   *
   * <code>.qms.Addon addon = 2;</code>
   * @return Whether the addon field is set.
   */
  @java.lang.Override
  public boolean hasAddon() {
    return ((bitField0_ & 0x00000002) != 0);
  }
  /**
   * <pre>
   * The values to set in the update.
   * </pre>
   *
   * <code>.qms.Addon addon = 2;</code>
   * @return The addon.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.Addon getAddon() {
    return addon_ == null ? org.cyverse.de.protobufs.Addon.getDefaultInstance() : addon_;
  }
  /**
   * <pre>
   * The values to set in the update.
   * </pre>
   *
   * <code>.qms.Addon addon = 2;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.AddonOrBuilder getAddonOrBuilder() {
    return addon_ == null ? org.cyverse.de.protobufs.Addon.getDefaultInstance() : addon_;
  }

  public static final int UPDATE_NAME_FIELD_NUMBER = 3;
  private boolean updateName_ = false;
  /**
   * <pre>
   * Whether to update the name of the addon.
   * </pre>
   *
   * <code>bool update_name = 3;</code>
   * @return The updateName.
   */
  @java.lang.Override
  public boolean getUpdateName() {
    return updateName_;
  }

  public static final int UPDATE_DESCRIPTION_FIELD_NUMBER = 4;
  private boolean updateDescription_ = false;
  /**
   * <pre>
   * Whether to update the description of the addon.
   * </pre>
   *
   * <code>bool update_description = 4;</code>
   * @return The updateDescription.
   */
  @java.lang.Override
  public boolean getUpdateDescription() {
    return updateDescription_;
  }

  public static final int UPDATE_RESOURCE_TYPE_FIELD_NUMBER = 5;
  private boolean updateResourceType_ = false;
  /**
   * <pre>
   * Whether to update the resource type of the addon.
   * </pre>
   *
   * <code>bool update_resource_type = 5;</code>
   * @return The updateResourceType.
   */
  @java.lang.Override
  public boolean getUpdateResourceType() {
    return updateResourceType_;
  }

  public static final int UPDATE_DEFAULT_AMOUNT_FIELD_NUMBER = 6;
  private boolean updateDefaultAmount_ = false;
  /**
   * <pre>
   * Whether to update the default amount of the addon.
   * </pre>
   *
   * <code>bool update_default_amount = 6;</code>
   * @return The updateDefaultAmount.
   */
  @java.lang.Override
  public boolean getUpdateDefaultAmount() {
    return updateDefaultAmount_;
  }

  public static final int UPDATE_DEFAULT_PAID_FIELD_NUMBER = 7;
  private boolean updateDefaultPaid_ = false;
  /**
   * <pre>
   * Whether to update the default paid field of the addon.
   * </pre>
   *
   * <code>bool update_default_paid = 7;</code>
   * @return The updateDefaultPaid.
   */
  @java.lang.Override
  public boolean getUpdateDefaultPaid() {
    return updateDefaultPaid_;
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
      output.writeMessage(1, getHeader());
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      output.writeMessage(2, getAddon());
    }
    if (updateName_ != false) {
      output.writeBool(3, updateName_);
    }
    if (updateDescription_ != false) {
      output.writeBool(4, updateDescription_);
    }
    if (updateResourceType_ != false) {
      output.writeBool(5, updateResourceType_);
    }
    if (updateDefaultAmount_ != false) {
      output.writeBool(6, updateDefaultAmount_);
    }
    if (updateDefaultPaid_ != false) {
      output.writeBool(7, updateDefaultPaid_);
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
        .computeMessageSize(1, getHeader());
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(2, getAddon());
    }
    if (updateName_ != false) {
      size += com.google.protobuf.CodedOutputStream
        .computeBoolSize(3, updateName_);
    }
    if (updateDescription_ != false) {
      size += com.google.protobuf.CodedOutputStream
        .computeBoolSize(4, updateDescription_);
    }
    if (updateResourceType_ != false) {
      size += com.google.protobuf.CodedOutputStream
        .computeBoolSize(5, updateResourceType_);
    }
    if (updateDefaultAmount_ != false) {
      size += com.google.protobuf.CodedOutputStream
        .computeBoolSize(6, updateDefaultAmount_);
    }
    if (updateDefaultPaid_ != false) {
      size += com.google.protobuf.CodedOutputStream
        .computeBoolSize(7, updateDefaultPaid_);
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
    if (!(obj instanceof org.cyverse.de.protobufs.UpdateAddonRequest)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.UpdateAddonRequest other = (org.cyverse.de.protobufs.UpdateAddonRequest) obj;

    if (hasHeader() != other.hasHeader()) return false;
    if (hasHeader()) {
      if (!getHeader()
          .equals(other.getHeader())) return false;
    }
    if (hasAddon() != other.hasAddon()) return false;
    if (hasAddon()) {
      if (!getAddon()
          .equals(other.getAddon())) return false;
    }
    if (getUpdateName()
        != other.getUpdateName()) return false;
    if (getUpdateDescription()
        != other.getUpdateDescription()) return false;
    if (getUpdateResourceType()
        != other.getUpdateResourceType()) return false;
    if (getUpdateDefaultAmount()
        != other.getUpdateDefaultAmount()) return false;
    if (getUpdateDefaultPaid()
        != other.getUpdateDefaultPaid()) return false;
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
    if (hasAddon()) {
      hash = (37 * hash) + ADDON_FIELD_NUMBER;
      hash = (53 * hash) + getAddon().hashCode();
    }
    hash = (37 * hash) + UPDATE_NAME_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashBoolean(
        getUpdateName());
    hash = (37 * hash) + UPDATE_DESCRIPTION_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashBoolean(
        getUpdateDescription());
    hash = (37 * hash) + UPDATE_RESOURCE_TYPE_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashBoolean(
        getUpdateResourceType());
    hash = (37 * hash) + UPDATE_DEFAULT_AMOUNT_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashBoolean(
        getUpdateDefaultAmount());
    hash = (37 * hash) + UPDATE_DEFAULT_PAID_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashBoolean(
        getUpdateDefaultPaid());
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static org.cyverse.de.protobufs.UpdateAddonRequest parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static org.cyverse.de.protobufs.UpdateAddonRequest parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.UpdateAddonRequest parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.UpdateAddonRequest prototype) {
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
   * A request to update an add-on. The boolean fields are needed because Go (and
   * probably other languages) needs a way to tell when to set a field to an empty
   * string, since that's the zero value for a string.
   * </pre>
   *
   * Protobuf type {@code qms.UpdateAddonRequest}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:qms.UpdateAddonRequest)
      org.cyverse.de.protobufs.UpdateAddonRequestOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_UpdateAddonRequest_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_UpdateAddonRequest_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.UpdateAddonRequest.class, org.cyverse.de.protobufs.UpdateAddonRequest.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.UpdateAddonRequest.newBuilder()
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
        getHeaderFieldBuilder();
        getAddonFieldBuilder();
      }
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
      addon_ = null;
      if (addonBuilder_ != null) {
        addonBuilder_.dispose();
        addonBuilder_ = null;
      }
      updateName_ = false;
      updateDescription_ = false;
      updateResourceType_ = false;
      updateDefaultAmount_ = false;
      updateDefaultPaid_ = false;
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_UpdateAddonRequest_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UpdateAddonRequest getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.UpdateAddonRequest.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UpdateAddonRequest build() {
      org.cyverse.de.protobufs.UpdateAddonRequest result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.UpdateAddonRequest buildPartial() {
      org.cyverse.de.protobufs.UpdateAddonRequest result = new org.cyverse.de.protobufs.UpdateAddonRequest(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.UpdateAddonRequest result) {
      int from_bitField0_ = bitField0_;
      int to_bitField0_ = 0;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.header_ = headerBuilder_ == null
            ? header_
            : headerBuilder_.build();
        to_bitField0_ |= 0x00000001;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.addon_ = addonBuilder_ == null
            ? addon_
            : addonBuilder_.build();
        to_bitField0_ |= 0x00000002;
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.updateName_ = updateName_;
      }
      if (((from_bitField0_ & 0x00000008) != 0)) {
        result.updateDescription_ = updateDescription_;
      }
      if (((from_bitField0_ & 0x00000010) != 0)) {
        result.updateResourceType_ = updateResourceType_;
      }
      if (((from_bitField0_ & 0x00000020) != 0)) {
        result.updateDefaultAmount_ = updateDefaultAmount_;
      }
      if (((from_bitField0_ & 0x00000040) != 0)) {
        result.updateDefaultPaid_ = updateDefaultPaid_;
      }
      result.bitField0_ |= to_bitField0_;
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.UpdateAddonRequest) {
        return mergeFrom((org.cyverse.de.protobufs.UpdateAddonRequest)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.UpdateAddonRequest other) {
      if (other == org.cyverse.de.protobufs.UpdateAddonRequest.getDefaultInstance()) return this;
      if (other.hasHeader()) {
        mergeHeader(other.getHeader());
      }
      if (other.hasAddon()) {
        mergeAddon(other.getAddon());
      }
      if (other.getUpdateName() != false) {
        setUpdateName(other.getUpdateName());
      }
      if (other.getUpdateDescription() != false) {
        setUpdateDescription(other.getUpdateDescription());
      }
      if (other.getUpdateResourceType() != false) {
        setUpdateResourceType(other.getUpdateResourceType());
      }
      if (other.getUpdateDefaultAmount() != false) {
        setUpdateDefaultAmount(other.getUpdateDefaultAmount());
      }
      if (other.getUpdateDefaultPaid() != false) {
        setUpdateDefaultPaid(other.getUpdateDefaultPaid());
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
                  getAddonFieldBuilder().getBuilder(),
                  extensionRegistry);
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 24: {
              updateName_ = input.readBool();
              bitField0_ |= 0x00000004;
              break;
            } // case 24
            case 32: {
              updateDescription_ = input.readBool();
              bitField0_ |= 0x00000008;
              break;
            } // case 32
            case 40: {
              updateResourceType_ = input.readBool();
              bitField0_ |= 0x00000010;
              break;
            } // case 40
            case 48: {
              updateDefaultAmount_ = input.readBool();
              bitField0_ |= 0x00000020;
              break;
            } // case 48
            case 56: {
              updateDefaultPaid_ = input.readBool();
              bitField0_ |= 0x00000040;
              break;
            } // case 56
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
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder> headerBuilder_;
    /**
     * <pre>
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     * @return Whether the header field is set.
     */
    public boolean hasHeader() {
      return ((bitField0_ & 0x00000001) != 0);
    }
    /**
     * <pre>
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
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
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
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
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
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
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
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
      if (header_ != null) {
        bitField0_ |= 0x00000001;
        onChanged();
      }
      return this;
    }
    /**
     * <pre>
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
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
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    public org.cyverse.de.protobufs.Header.Builder getHeaderBuilder() {
      bitField0_ |= 0x00000001;
      onChanged();
      return getHeaderFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
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
     * Contains telemetry information.
     * </pre>
     *
     * <code>.header.Header header = 1;</code>
     */
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder> 
        getHeaderFieldBuilder() {
      if (headerBuilder_ == null) {
        headerBuilder_ = new com.google.protobuf.SingleFieldBuilder<
            org.cyverse.de.protobufs.Header, org.cyverse.de.protobufs.Header.Builder, org.cyverse.de.protobufs.HeaderOrBuilder>(
                getHeader(),
                getParentForChildren(),
                isClean());
        header_ = null;
      }
      return headerBuilder_;
    }

    private org.cyverse.de.protobufs.Addon addon_;
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.Addon, org.cyverse.de.protobufs.Addon.Builder, org.cyverse.de.protobufs.AddonOrBuilder> addonBuilder_;
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     * @return Whether the addon field is set.
     */
    public boolean hasAddon() {
      return ((bitField0_ & 0x00000002) != 0);
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     * @return The addon.
     */
    public org.cyverse.de.protobufs.Addon getAddon() {
      if (addonBuilder_ == null) {
        return addon_ == null ? org.cyverse.de.protobufs.Addon.getDefaultInstance() : addon_;
      } else {
        return addonBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     */
    public Builder setAddon(org.cyverse.de.protobufs.Addon value) {
      if (addonBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        addon_ = value;
      } else {
        addonBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     */
    public Builder setAddon(
        org.cyverse.de.protobufs.Addon.Builder builderForValue) {
      if (addonBuilder_ == null) {
        addon_ = builderForValue.build();
      } else {
        addonBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     */
    public Builder mergeAddon(org.cyverse.de.protobufs.Addon value) {
      if (addonBuilder_ == null) {
        if (((bitField0_ & 0x00000002) != 0) &&
          addon_ != null &&
          addon_ != org.cyverse.de.protobufs.Addon.getDefaultInstance()) {
          getAddonBuilder().mergeFrom(value);
        } else {
          addon_ = value;
        }
      } else {
        addonBuilder_.mergeFrom(value);
      }
      if (addon_ != null) {
        bitField0_ |= 0x00000002;
        onChanged();
      }
      return this;
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     */
    public Builder clearAddon() {
      bitField0_ = (bitField0_ & ~0x00000002);
      addon_ = null;
      if (addonBuilder_ != null) {
        addonBuilder_.dispose();
        addonBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     */
    public org.cyverse.de.protobufs.Addon.Builder getAddonBuilder() {
      bitField0_ |= 0x00000002;
      onChanged();
      return getAddonFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     */
    public org.cyverse.de.protobufs.AddonOrBuilder getAddonOrBuilder() {
      if (addonBuilder_ != null) {
        return addonBuilder_.getMessageOrBuilder();
      } else {
        return addon_ == null ?
            org.cyverse.de.protobufs.Addon.getDefaultInstance() : addon_;
      }
    }
    /**
     * <pre>
     * The values to set in the update.
     * </pre>
     *
     * <code>.qms.Addon addon = 2;</code>
     */
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.Addon, org.cyverse.de.protobufs.Addon.Builder, org.cyverse.de.protobufs.AddonOrBuilder> 
        getAddonFieldBuilder() {
      if (addonBuilder_ == null) {
        addonBuilder_ = new com.google.protobuf.SingleFieldBuilder<
            org.cyverse.de.protobufs.Addon, org.cyverse.de.protobufs.Addon.Builder, org.cyverse.de.protobufs.AddonOrBuilder>(
                getAddon(),
                getParentForChildren(),
                isClean());
        addon_ = null;
      }
      return addonBuilder_;
    }

    private boolean updateName_ ;
    /**
     * <pre>
     * Whether to update the name of the addon.
     * </pre>
     *
     * <code>bool update_name = 3;</code>
     * @return The updateName.
     */
    @java.lang.Override
    public boolean getUpdateName() {
      return updateName_;
    }
    /**
     * <pre>
     * Whether to update the name of the addon.
     * </pre>
     *
     * <code>bool update_name = 3;</code>
     * @param value The updateName to set.
     * @return This builder for chaining.
     */
    public Builder setUpdateName(boolean value) {

      updateName_ = value;
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Whether to update the name of the addon.
     * </pre>
     *
     * <code>bool update_name = 3;</code>
     * @return This builder for chaining.
     */
    public Builder clearUpdateName() {
      bitField0_ = (bitField0_ & ~0x00000004);
      updateName_ = false;
      onChanged();
      return this;
    }

    private boolean updateDescription_ ;
    /**
     * <pre>
     * Whether to update the description of the addon.
     * </pre>
     *
     * <code>bool update_description = 4;</code>
     * @return The updateDescription.
     */
    @java.lang.Override
    public boolean getUpdateDescription() {
      return updateDescription_;
    }
    /**
     * <pre>
     * Whether to update the description of the addon.
     * </pre>
     *
     * <code>bool update_description = 4;</code>
     * @param value The updateDescription to set.
     * @return This builder for chaining.
     */
    public Builder setUpdateDescription(boolean value) {

      updateDescription_ = value;
      bitField0_ |= 0x00000008;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Whether to update the description of the addon.
     * </pre>
     *
     * <code>bool update_description = 4;</code>
     * @return This builder for chaining.
     */
    public Builder clearUpdateDescription() {
      bitField0_ = (bitField0_ & ~0x00000008);
      updateDescription_ = false;
      onChanged();
      return this;
    }

    private boolean updateResourceType_ ;
    /**
     * <pre>
     * Whether to update the resource type of the addon.
     * </pre>
     *
     * <code>bool update_resource_type = 5;</code>
     * @return The updateResourceType.
     */
    @java.lang.Override
    public boolean getUpdateResourceType() {
      return updateResourceType_;
    }
    /**
     * <pre>
     * Whether to update the resource type of the addon.
     * </pre>
     *
     * <code>bool update_resource_type = 5;</code>
     * @param value The updateResourceType to set.
     * @return This builder for chaining.
     */
    public Builder setUpdateResourceType(boolean value) {

      updateResourceType_ = value;
      bitField0_ |= 0x00000010;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Whether to update the resource type of the addon.
     * </pre>
     *
     * <code>bool update_resource_type = 5;</code>
     * @return This builder for chaining.
     */
    public Builder clearUpdateResourceType() {
      bitField0_ = (bitField0_ & ~0x00000010);
      updateResourceType_ = false;
      onChanged();
      return this;
    }

    private boolean updateDefaultAmount_ ;
    /**
     * <pre>
     * Whether to update the default amount of the addon.
     * </pre>
     *
     * <code>bool update_default_amount = 6;</code>
     * @return The updateDefaultAmount.
     */
    @java.lang.Override
    public boolean getUpdateDefaultAmount() {
      return updateDefaultAmount_;
    }
    /**
     * <pre>
     * Whether to update the default amount of the addon.
     * </pre>
     *
     * <code>bool update_default_amount = 6;</code>
     * @param value The updateDefaultAmount to set.
     * @return This builder for chaining.
     */
    public Builder setUpdateDefaultAmount(boolean value) {

      updateDefaultAmount_ = value;
      bitField0_ |= 0x00000020;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Whether to update the default amount of the addon.
     * </pre>
     *
     * <code>bool update_default_amount = 6;</code>
     * @return This builder for chaining.
     */
    public Builder clearUpdateDefaultAmount() {
      bitField0_ = (bitField0_ & ~0x00000020);
      updateDefaultAmount_ = false;
      onChanged();
      return this;
    }

    private boolean updateDefaultPaid_ ;
    /**
     * <pre>
     * Whether to update the default paid field of the addon.
     * </pre>
     *
     * <code>bool update_default_paid = 7;</code>
     * @return The updateDefaultPaid.
     */
    @java.lang.Override
    public boolean getUpdateDefaultPaid() {
      return updateDefaultPaid_;
    }
    /**
     * <pre>
     * Whether to update the default paid field of the addon.
     * </pre>
     *
     * <code>bool update_default_paid = 7;</code>
     * @param value The updateDefaultPaid to set.
     * @return This builder for chaining.
     */
    public Builder setUpdateDefaultPaid(boolean value) {

      updateDefaultPaid_ = value;
      bitField0_ |= 0x00000040;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * Whether to update the default paid field of the addon.
     * </pre>
     *
     * <code>bool update_default_paid = 7;</code>
     * @return This builder for chaining.
     */
    public Builder clearUpdateDefaultPaid() {
      bitField0_ = (bitField0_ & ~0x00000040);
      updateDefaultPaid_ = false;
      onChanged();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:qms.UpdateAddonRequest)
  }

  // @@protoc_insertion_point(class_scope:qms.UpdateAddonRequest)
  private static final org.cyverse.de.protobufs.UpdateAddonRequest DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.UpdateAddonRequest();
  }

  public static org.cyverse.de.protobufs.UpdateAddonRequest getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<UpdateAddonRequest>
      PARSER = new com.google.protobuf.AbstractParser<UpdateAddonRequest>() {
    @java.lang.Override
    public UpdateAddonRequest parsePartialFrom(
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

  public static com.google.protobuf.Parser<UpdateAddonRequest> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<UpdateAddonRequest> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.UpdateAddonRequest getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

