// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_addons.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

/**
 * <pre>
 * *
 * Represents the rate charged for an addon as the prcie for one year of service.
 * </pre>
 *
 * Protobuf type {@code qms.AddonRate}
 */
public final class AddonRate extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:qms.AddonRate)
    AddonRateOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 29,
      /* patch= */ 0,
      /* suffix= */ "",
      AddonRate.class.getName());
  }
  // Use AddonRate.newBuilder() to construct.
  private AddonRate(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private AddonRate() {
    uuid_ = "";
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_AddonRate_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_AddonRate_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.AddonRate.class, org.cyverse.de.protobufs.AddonRate.Builder.class);
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

  public static final int RATE_FIELD_NUMBER = 2;
  private double rate_ = 0D;
  /**
   * <pre>
   * The rate.
   * </pre>
   *
   * <code>double rate = 2;</code>
   * @return The rate.
   */
  @java.lang.Override
  public double getRate() {
    return rate_;
  }

  public static final int EFFECTIVE_DATE_FIELD_NUMBER = 3;
  private com.google.protobuf.Timestamp effectiveDate_;
  /**
   * <pre>
   * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
   * any given time is always the rate with the most recent effective date that occurs in the past.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
   * @return Whether the effectiveDate field is set.
   */
  @java.lang.Override
  public boolean hasEffectiveDate() {
    return ((bitField0_ & 0x00000001) != 0);
  }
  /**
   * <pre>
   * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
   * any given time is always the rate with the most recent effective date that occurs in the past.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
   * @return The effectiveDate.
   */
  @java.lang.Override
  public com.google.protobuf.Timestamp getEffectiveDate() {
    return effectiveDate_ == null ? com.google.protobuf.Timestamp.getDefaultInstance() : effectiveDate_;
  }
  /**
   * <pre>
   * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
   * any given time is always the rate with the most recent effective date that occurs in the past.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
   */
  @java.lang.Override
  public com.google.protobuf.TimestampOrBuilder getEffectiveDateOrBuilder() {
    return effectiveDate_ == null ? com.google.protobuf.Timestamp.getDefaultInstance() : effectiveDate_;
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
    if (java.lang.Double.doubleToRawLongBits(rate_) != 0) {
      output.writeDouble(2, rate_);
    }
    if (((bitField0_ & 0x00000001) != 0)) {
      output.writeMessage(3, getEffectiveDate());
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
    if (java.lang.Double.doubleToRawLongBits(rate_) != 0) {
      size += com.google.protobuf.CodedOutputStream
        .computeDoubleSize(2, rate_);
    }
    if (((bitField0_ & 0x00000001) != 0)) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(3, getEffectiveDate());
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
    if (!(obj instanceof org.cyverse.de.protobufs.AddonRate)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.AddonRate other = (org.cyverse.de.protobufs.AddonRate) obj;

    if (!getUuid()
        .equals(other.getUuid())) return false;
    if (java.lang.Double.doubleToLongBits(getRate())
        != java.lang.Double.doubleToLongBits(
            other.getRate())) return false;
    if (hasEffectiveDate() != other.hasEffectiveDate()) return false;
    if (hasEffectiveDate()) {
      if (!getEffectiveDate()
          .equals(other.getEffectiveDate())) return false;
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
    hash = (37 * hash) + UUID_FIELD_NUMBER;
    hash = (53 * hash) + getUuid().hashCode();
    hash = (37 * hash) + RATE_FIELD_NUMBER;
    hash = (53 * hash) + com.google.protobuf.Internal.hashLong(
        java.lang.Double.doubleToLongBits(getRate()));
    if (hasEffectiveDate()) {
      hash = (37 * hash) + EFFECTIVE_DATE_FIELD_NUMBER;
      hash = (53 * hash) + getEffectiveDate().hashCode();
    }
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.AddonRate parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static org.cyverse.de.protobufs.AddonRate parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static org.cyverse.de.protobufs.AddonRate parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.AddonRate parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.AddonRate prototype) {
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
   * Represents the rate charged for an addon as the prcie for one year of service.
   * </pre>
   *
   * Protobuf type {@code qms.AddonRate}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:qms.AddonRate)
      org.cyverse.de.protobufs.AddonRateOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_AddonRate_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_AddonRate_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.AddonRate.class, org.cyverse.de.protobufs.AddonRate.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.AddonRate.newBuilder()
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
        getEffectiveDateFieldBuilder();
      }
    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      bitField0_ = 0;
      uuid_ = "";
      rate_ = 0D;
      effectiveDate_ = null;
      if (effectiveDateBuilder_ != null) {
        effectiveDateBuilder_.dispose();
        effectiveDateBuilder_ = null;
      }
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.QMSAddonProtobufs.internal_static_qms_AddonRate_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.AddonRate getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.AddonRate.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.AddonRate build() {
      org.cyverse.de.protobufs.AddonRate result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.AddonRate buildPartial() {
      org.cyverse.de.protobufs.AddonRate result = new org.cyverse.de.protobufs.AddonRate(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.AddonRate result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.uuid_ = uuid_;
      }
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.rate_ = rate_;
      }
      int to_bitField0_ = 0;
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.effectiveDate_ = effectiveDateBuilder_ == null
            ? effectiveDate_
            : effectiveDateBuilder_.build();
        to_bitField0_ |= 0x00000001;
      }
      result.bitField0_ |= to_bitField0_;
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.AddonRate) {
        return mergeFrom((org.cyverse.de.protobufs.AddonRate)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.AddonRate other) {
      if (other == org.cyverse.de.protobufs.AddonRate.getDefaultInstance()) return this;
      if (!other.getUuid().isEmpty()) {
        uuid_ = other.uuid_;
        bitField0_ |= 0x00000001;
        onChanged();
      }
      if (other.getRate() != 0D) {
        setRate(other.getRate());
      }
      if (other.hasEffectiveDate()) {
        mergeEffectiveDate(other.getEffectiveDate());
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
            case 17: {
              rate_ = input.readDouble();
              bitField0_ |= 0x00000002;
              break;
            } // case 17
            case 26: {
              input.readMessage(
                  getEffectiveDateFieldBuilder().getBuilder(),
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

    private double rate_ ;
    /**
     * <pre>
     * The rate.
     * </pre>
     *
     * <code>double rate = 2;</code>
     * @return The rate.
     */
    @java.lang.Override
    public double getRate() {
      return rate_;
    }
    /**
     * <pre>
     * The rate.
     * </pre>
     *
     * <code>double rate = 2;</code>
     * @param value The rate to set.
     * @return This builder for chaining.
     */
    public Builder setRate(double value) {

      rate_ = value;
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The rate.
     * </pre>
     *
     * <code>double rate = 2;</code>
     * @return This builder for chaining.
     */
    public Builder clearRate() {
      bitField0_ = (bitField0_ & ~0x00000002);
      rate_ = 0D;
      onChanged();
      return this;
    }

    private com.google.protobuf.Timestamp effectiveDate_;
    private com.google.protobuf.SingleFieldBuilder<
        com.google.protobuf.Timestamp, com.google.protobuf.Timestamp.Builder, com.google.protobuf.TimestampOrBuilder> effectiveDateBuilder_;
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     * @return Whether the effectiveDate field is set.
     */
    public boolean hasEffectiveDate() {
      return ((bitField0_ & 0x00000004) != 0);
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     * @return The effectiveDate.
     */
    public com.google.protobuf.Timestamp getEffectiveDate() {
      if (effectiveDateBuilder_ == null) {
        return effectiveDate_ == null ? com.google.protobuf.Timestamp.getDefaultInstance() : effectiveDate_;
      } else {
        return effectiveDateBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     */
    public Builder setEffectiveDate(com.google.protobuf.Timestamp value) {
      if (effectiveDateBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        effectiveDate_ = value;
      } else {
        effectiveDateBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     */
    public Builder setEffectiveDate(
        com.google.protobuf.Timestamp.Builder builderForValue) {
      if (effectiveDateBuilder_ == null) {
        effectiveDate_ = builderForValue.build();
      } else {
        effectiveDateBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     */
    public Builder mergeEffectiveDate(com.google.protobuf.Timestamp value) {
      if (effectiveDateBuilder_ == null) {
        if (((bitField0_ & 0x00000004) != 0) &&
          effectiveDate_ != null &&
          effectiveDate_ != com.google.protobuf.Timestamp.getDefaultInstance()) {
          getEffectiveDateBuilder().mergeFrom(value);
        } else {
          effectiveDate_ = value;
        }
      } else {
        effectiveDateBuilder_.mergeFrom(value);
      }
      if (effectiveDate_ != null) {
        bitField0_ |= 0x00000004;
        onChanged();
      }
      return this;
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     */
    public Builder clearEffectiveDate() {
      bitField0_ = (bitField0_ & ~0x00000004);
      effectiveDate_ = null;
      if (effectiveDateBuilder_ != null) {
        effectiveDateBuilder_.dispose();
        effectiveDateBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     */
    public com.google.protobuf.Timestamp.Builder getEffectiveDateBuilder() {
      bitField0_ |= 0x00000004;
      onChanged();
      return getEffectiveDateFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     */
    public com.google.protobuf.TimestampOrBuilder getEffectiveDateOrBuilder() {
      if (effectiveDateBuilder_ != null) {
        return effectiveDateBuilder_.getMessageOrBuilder();
      } else {
        return effectiveDate_ == null ?
            com.google.protobuf.Timestamp.getDefaultInstance() : effectiveDate_;
      }
    }
    /**
     * <pre>
     * The date that the rate becomes effective. There can be multiple rates for an addon; the rate that is effective at
     * any given time is always the rate with the most recent effective date that occurs in the past.
     * </pre>
     *
     * <code>.google.protobuf.Timestamp effective_date = 3 [json_name = "effective_date"];</code>
     */
    private com.google.protobuf.SingleFieldBuilder<
        com.google.protobuf.Timestamp, com.google.protobuf.Timestamp.Builder, com.google.protobuf.TimestampOrBuilder> 
        getEffectiveDateFieldBuilder() {
      if (effectiveDateBuilder_ == null) {
        effectiveDateBuilder_ = new com.google.protobuf.SingleFieldBuilder<
            com.google.protobuf.Timestamp, com.google.protobuf.Timestamp.Builder, com.google.protobuf.TimestampOrBuilder>(
                getEffectiveDate(),
                getParentForChildren(),
                isClean());
        effectiveDate_ = null;
      }
      return effectiveDateBuilder_;
    }

    // @@protoc_insertion_point(builder_scope:qms.AddonRate)
  }

  // @@protoc_insertion_point(class_scope:qms.AddonRate)
  private static final org.cyverse.de.protobufs.AddonRate DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.AddonRate();
  }

  public static org.cyverse.de.protobufs.AddonRate getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<AddonRate>
      PARSER = new com.google.protobuf.AbstractParser<AddonRate>() {
    @java.lang.Override
    public AddonRate parsePartialFrom(
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

  public static com.google.protobuf.Parser<AddonRate> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<AddonRate> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.AddonRate getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

