// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: groups.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

/**
 * <pre>
 * *
 * Information about permissions granted to a user.
 * </pre>
 *
 * Protobuf type {@code groups.Permission}
 */
public final class Permission extends
    com.google.protobuf.GeneratedMessage implements
    // @@protoc_insertion_point(message_implements:groups.Permission)
    PermissionOrBuilder {
private static final long serialVersionUID = 0L;
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 28,
      /* patch= */ 3,
      /* suffix= */ "",
      Permission.class.getName());
  }
  // Use Permission.newBuilder() to construct.
  private Permission(com.google.protobuf.GeneratedMessage.Builder<?> builder) {
    super(builder);
  }
  private Permission() {
    id_ = "";
    permissionLevel_ = 0;
  }

  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_Permission_descriptor;
  }

  @java.lang.Override
  protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_Permission_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            org.cyverse.de.protobufs.Permission.class, org.cyverse.de.protobufs.Permission.Builder.class);
  }

  private int bitField0_;
  public static final int ID_FIELD_NUMBER = 1;
  @SuppressWarnings("serial")
  private volatile java.lang.Object id_ = "";
  /**
   * <pre>
   * The permission id.
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
   * The permission id.
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

  public static final int SUBJECT_FIELD_NUMBER = 2;
  private org.cyverse.de.protobufs.SubjectOut subject_;
  /**
   * <pre>
   * The outgoing subject.
   * </pre>
   *
   * <code>.groups.SubjectOut subject = 2;</code>
   * @return Whether the subject field is set.
   */
  @java.lang.Override
  public boolean hasSubject() {
    return ((bitField0_ & 0x00000001) != 0);
  }
  /**
   * <pre>
   * The outgoing subject.
   * </pre>
   *
   * <code>.groups.SubjectOut subject = 2;</code>
   * @return The subject.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.SubjectOut getSubject() {
    return subject_ == null ? org.cyverse.de.protobufs.SubjectOut.getDefaultInstance() : subject_;
  }
  /**
   * <pre>
   * The outgoing subject.
   * </pre>
   *
   * <code>.groups.SubjectOut subject = 2;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.SubjectOutOrBuilder getSubjectOrBuilder() {
    return subject_ == null ? org.cyverse.de.protobufs.SubjectOut.getDefaultInstance() : subject_;
  }

  public static final int RESOURCE_FIELD_NUMBER = 3;
  private org.cyverse.de.protobufs.ResourceOut resource_;
  /**
   * <pre>
   * The outgoing resource.
   * </pre>
   *
   * <code>.groups.ResourceOut resource = 3;</code>
   * @return Whether the resource field is set.
   */
  @java.lang.Override
  public boolean hasResource() {
    return ((bitField0_ & 0x00000002) != 0);
  }
  /**
   * <pre>
   * The outgoing resource.
   * </pre>
   *
   * <code>.groups.ResourceOut resource = 3;</code>
   * @return The resource.
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceOut getResource() {
    return resource_ == null ? org.cyverse.de.protobufs.ResourceOut.getDefaultInstance() : resource_;
  }
  /**
   * <pre>
   * The outgoing resource.
   * </pre>
   *
   * <code>.groups.ResourceOut resource = 3;</code>
   */
  @java.lang.Override
  public org.cyverse.de.protobufs.ResourceOutOrBuilder getResourceOrBuilder() {
    return resource_ == null ? org.cyverse.de.protobufs.ResourceOut.getDefaultInstance() : resource_;
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
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(id_)) {
      com.google.protobuf.GeneratedMessage.writeString(output, 1, id_);
    }
    if (((bitField0_ & 0x00000001) != 0)) {
      output.writeMessage(2, getSubject());
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      output.writeMessage(3, getResource());
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
    if (!com.google.protobuf.GeneratedMessage.isStringEmpty(id_)) {
      size += com.google.protobuf.GeneratedMessage.computeStringSize(1, id_);
    }
    if (((bitField0_ & 0x00000001) != 0)) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(2, getSubject());
    }
    if (((bitField0_ & 0x00000002) != 0)) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(3, getResource());
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
    if (!(obj instanceof org.cyverse.de.protobufs.Permission)) {
      return super.equals(obj);
    }
    org.cyverse.de.protobufs.Permission other = (org.cyverse.de.protobufs.Permission) obj;

    if (!getId()
        .equals(other.getId())) return false;
    if (hasSubject() != other.hasSubject()) return false;
    if (hasSubject()) {
      if (!getSubject()
          .equals(other.getSubject())) return false;
    }
    if (hasResource() != other.hasResource()) return false;
    if (hasResource()) {
      if (!getResource()
          .equals(other.getResource())) return false;
    }
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
    hash = (37 * hash) + ID_FIELD_NUMBER;
    hash = (53 * hash) + getId().hashCode();
    if (hasSubject()) {
      hash = (37 * hash) + SUBJECT_FIELD_NUMBER;
      hash = (53 * hash) + getSubject().hashCode();
    }
    if (hasResource()) {
      hash = (37 * hash) + RESOURCE_FIELD_NUMBER;
      hash = (53 * hash) + getResource().hashCode();
    }
    hash = (37 * hash) + PERMISSION_LEVEL_FIELD_NUMBER;
    hash = (53 * hash) + permissionLevel_;
    hash = (29 * hash) + getUnknownFields().hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static org.cyverse.de.protobufs.Permission parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public static org.cyverse.de.protobufs.Permission parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input);
  }

  public static org.cyverse.de.protobufs.Permission parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessage
        .parseWithIOException(PARSER, input);
  }
  public static org.cyverse.de.protobufs.Permission parseFrom(
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
  public static Builder newBuilder(org.cyverse.de.protobufs.Permission prototype) {
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
   * Information about permissions granted to a user.
   * </pre>
   *
   * Protobuf type {@code groups.Permission}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessage.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:groups.Permission)
      org.cyverse.de.protobufs.PermissionOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_Permission_descriptor;
    }

    @java.lang.Override
    protected com.google.protobuf.GeneratedMessage.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_Permission_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              org.cyverse.de.protobufs.Permission.class, org.cyverse.de.protobufs.Permission.Builder.class);
    }

    // Construct using org.cyverse.de.protobufs.Permission.newBuilder()
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
        getSubjectFieldBuilder();
        getResourceFieldBuilder();
      }
    }
    @java.lang.Override
    public Builder clear() {
      super.clear();
      bitField0_ = 0;
      id_ = "";
      subject_ = null;
      if (subjectBuilder_ != null) {
        subjectBuilder_.dispose();
        subjectBuilder_ = null;
      }
      resource_ = null;
      if (resourceBuilder_ != null) {
        resourceBuilder_.dispose();
        resourceBuilder_ = null;
      }
      permissionLevel_ = 0;
      return this;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return org.cyverse.de.protobufs.GroupsProtobufs.internal_static_groups_Permission_descriptor;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Permission getDefaultInstanceForType() {
      return org.cyverse.de.protobufs.Permission.getDefaultInstance();
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Permission build() {
      org.cyverse.de.protobufs.Permission result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    @java.lang.Override
    public org.cyverse.de.protobufs.Permission buildPartial() {
      org.cyverse.de.protobufs.Permission result = new org.cyverse.de.protobufs.Permission(this);
      if (bitField0_ != 0) { buildPartial0(result); }
      onBuilt();
      return result;
    }

    private void buildPartial0(org.cyverse.de.protobufs.Permission result) {
      int from_bitField0_ = bitField0_;
      if (((from_bitField0_ & 0x00000001) != 0)) {
        result.id_ = id_;
      }
      int to_bitField0_ = 0;
      if (((from_bitField0_ & 0x00000002) != 0)) {
        result.subject_ = subjectBuilder_ == null
            ? subject_
            : subjectBuilder_.build();
        to_bitField0_ |= 0x00000001;
      }
      if (((from_bitField0_ & 0x00000004) != 0)) {
        result.resource_ = resourceBuilder_ == null
            ? resource_
            : resourceBuilder_.build();
        to_bitField0_ |= 0x00000002;
      }
      if (((from_bitField0_ & 0x00000008) != 0)) {
        result.permissionLevel_ = permissionLevel_;
      }
      result.bitField0_ |= to_bitField0_;
    }

    @java.lang.Override
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof org.cyverse.de.protobufs.Permission) {
        return mergeFrom((org.cyverse.de.protobufs.Permission)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(org.cyverse.de.protobufs.Permission other) {
      if (other == org.cyverse.de.protobufs.Permission.getDefaultInstance()) return this;
      if (!other.getId().isEmpty()) {
        id_ = other.id_;
        bitField0_ |= 0x00000001;
        onChanged();
      }
      if (other.hasSubject()) {
        mergeSubject(other.getSubject());
      }
      if (other.hasResource()) {
        mergeResource(other.getResource());
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
              id_ = input.readStringRequireUtf8();
              bitField0_ |= 0x00000001;
              break;
            } // case 10
            case 18: {
              input.readMessage(
                  getSubjectFieldBuilder().getBuilder(),
                  extensionRegistry);
              bitField0_ |= 0x00000002;
              break;
            } // case 18
            case 26: {
              input.readMessage(
                  getResourceFieldBuilder().getBuilder(),
                  extensionRegistry);
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

    private java.lang.Object id_ = "";
    /**
     * <pre>
     * The permission id.
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
     * The permission id.
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
     * The permission id.
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
     * The permission id.
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
     * The permission id.
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

    private org.cyverse.de.protobufs.SubjectOut subject_;
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.SubjectOut, org.cyverse.de.protobufs.SubjectOut.Builder, org.cyverse.de.protobufs.SubjectOutOrBuilder> subjectBuilder_;
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     * @return Whether the subject field is set.
     */
    public boolean hasSubject() {
      return ((bitField0_ & 0x00000002) != 0);
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     * @return The subject.
     */
    public org.cyverse.de.protobufs.SubjectOut getSubject() {
      if (subjectBuilder_ == null) {
        return subject_ == null ? org.cyverse.de.protobufs.SubjectOut.getDefaultInstance() : subject_;
      } else {
        return subjectBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     */
    public Builder setSubject(org.cyverse.de.protobufs.SubjectOut value) {
      if (subjectBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        subject_ = value;
      } else {
        subjectBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     */
    public Builder setSubject(
        org.cyverse.de.protobufs.SubjectOut.Builder builderForValue) {
      if (subjectBuilder_ == null) {
        subject_ = builderForValue.build();
      } else {
        subjectBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000002;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     */
    public Builder mergeSubject(org.cyverse.de.protobufs.SubjectOut value) {
      if (subjectBuilder_ == null) {
        if (((bitField0_ & 0x00000002) != 0) &&
          subject_ != null &&
          subject_ != org.cyverse.de.protobufs.SubjectOut.getDefaultInstance()) {
          getSubjectBuilder().mergeFrom(value);
        } else {
          subject_ = value;
        }
      } else {
        subjectBuilder_.mergeFrom(value);
      }
      if (subject_ != null) {
        bitField0_ |= 0x00000002;
        onChanged();
      }
      return this;
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     */
    public Builder clearSubject() {
      bitField0_ = (bitField0_ & ~0x00000002);
      subject_ = null;
      if (subjectBuilder_ != null) {
        subjectBuilder_.dispose();
        subjectBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     */
    public org.cyverse.de.protobufs.SubjectOut.Builder getSubjectBuilder() {
      bitField0_ |= 0x00000002;
      onChanged();
      return getSubjectFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     */
    public org.cyverse.de.protobufs.SubjectOutOrBuilder getSubjectOrBuilder() {
      if (subjectBuilder_ != null) {
        return subjectBuilder_.getMessageOrBuilder();
      } else {
        return subject_ == null ?
            org.cyverse.de.protobufs.SubjectOut.getDefaultInstance() : subject_;
      }
    }
    /**
     * <pre>
     * The outgoing subject.
     * </pre>
     *
     * <code>.groups.SubjectOut subject = 2;</code>
     */
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.SubjectOut, org.cyverse.de.protobufs.SubjectOut.Builder, org.cyverse.de.protobufs.SubjectOutOrBuilder> 
        getSubjectFieldBuilder() {
      if (subjectBuilder_ == null) {
        subjectBuilder_ = new com.google.protobuf.SingleFieldBuilder<
            org.cyverse.de.protobufs.SubjectOut, org.cyverse.de.protobufs.SubjectOut.Builder, org.cyverse.de.protobufs.SubjectOutOrBuilder>(
                getSubject(),
                getParentForChildren(),
                isClean());
        subject_ = null;
      }
      return subjectBuilder_;
    }

    private org.cyverse.de.protobufs.ResourceOut resource_;
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.ResourceOut, org.cyverse.de.protobufs.ResourceOut.Builder, org.cyverse.de.protobufs.ResourceOutOrBuilder> resourceBuilder_;
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     * @return Whether the resource field is set.
     */
    public boolean hasResource() {
      return ((bitField0_ & 0x00000004) != 0);
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     * @return The resource.
     */
    public org.cyverse.de.protobufs.ResourceOut getResource() {
      if (resourceBuilder_ == null) {
        return resource_ == null ? org.cyverse.de.protobufs.ResourceOut.getDefaultInstance() : resource_;
      } else {
        return resourceBuilder_.getMessage();
      }
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     */
    public Builder setResource(org.cyverse.de.protobufs.ResourceOut value) {
      if (resourceBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        resource_ = value;
      } else {
        resourceBuilder_.setMessage(value);
      }
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     */
    public Builder setResource(
        org.cyverse.de.protobufs.ResourceOut.Builder builderForValue) {
      if (resourceBuilder_ == null) {
        resource_ = builderForValue.build();
      } else {
        resourceBuilder_.setMessage(builderForValue.build());
      }
      bitField0_ |= 0x00000004;
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     */
    public Builder mergeResource(org.cyverse.de.protobufs.ResourceOut value) {
      if (resourceBuilder_ == null) {
        if (((bitField0_ & 0x00000004) != 0) &&
          resource_ != null &&
          resource_ != org.cyverse.de.protobufs.ResourceOut.getDefaultInstance()) {
          getResourceBuilder().mergeFrom(value);
        } else {
          resource_ = value;
        }
      } else {
        resourceBuilder_.mergeFrom(value);
      }
      if (resource_ != null) {
        bitField0_ |= 0x00000004;
        onChanged();
      }
      return this;
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     */
    public Builder clearResource() {
      bitField0_ = (bitField0_ & ~0x00000004);
      resource_ = null;
      if (resourceBuilder_ != null) {
        resourceBuilder_.dispose();
        resourceBuilder_ = null;
      }
      onChanged();
      return this;
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     */
    public org.cyverse.de.protobufs.ResourceOut.Builder getResourceBuilder() {
      bitField0_ |= 0x00000004;
      onChanged();
      return getResourceFieldBuilder().getBuilder();
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     */
    public org.cyverse.de.protobufs.ResourceOutOrBuilder getResourceOrBuilder() {
      if (resourceBuilder_ != null) {
        return resourceBuilder_.getMessageOrBuilder();
      } else {
        return resource_ == null ?
            org.cyverse.de.protobufs.ResourceOut.getDefaultInstance() : resource_;
      }
    }
    /**
     * <pre>
     * The outgoing resource.
     * </pre>
     *
     * <code>.groups.ResourceOut resource = 3;</code>
     */
    private com.google.protobuf.SingleFieldBuilder<
        org.cyverse.de.protobufs.ResourceOut, org.cyverse.de.protobufs.ResourceOut.Builder, org.cyverse.de.protobufs.ResourceOutOrBuilder> 
        getResourceFieldBuilder() {
      if (resourceBuilder_ == null) {
        resourceBuilder_ = new com.google.protobuf.SingleFieldBuilder<
            org.cyverse.de.protobufs.ResourceOut, org.cyverse.de.protobufs.ResourceOut.Builder, org.cyverse.de.protobufs.ResourceOutOrBuilder>(
                getResource(),
                getParentForChildren(),
                isClean());
        resource_ = null;
      }
      return resourceBuilder_;
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

    // @@protoc_insertion_point(builder_scope:groups.Permission)
  }

  // @@protoc_insertion_point(class_scope:groups.Permission)
  private static final org.cyverse.de.protobufs.Permission DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new org.cyverse.de.protobufs.Permission();
  }

  public static org.cyverse.de.protobufs.Permission getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<Permission>
      PARSER = new com.google.protobuf.AbstractParser<Permission>() {
    @java.lang.Override
    public Permission parsePartialFrom(
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

  public static com.google.protobuf.Parser<Permission> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<Permission> getParserForType() {
    return PARSER;
  }

  @java.lang.Override
  public org.cyverse.de.protobufs.Permission getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

