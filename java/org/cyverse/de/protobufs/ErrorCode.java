// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: svcerror.proto

package org.cyverse.de.protobufs;

/**
 * <pre>
 **
 * The types of errors that can be retuned by message handlers.
 * </pre>
 *
 * Protobuf enum {@code svcerror.ErrorCode}
 */
public enum ErrorCode
    implements com.google.protobuf.ProtocolMessageEnum {
  /**
   * <pre>
   * Default value for the error code. Don't set the error code to this. Use Unspecified if tempted.
   * </pre>
   *
   * <code>UNSET = 0;</code>
   */
  UNSET(0),
  /**
   * <pre>
   * An error occurred, but the kind wasn't specified or included in the list.
   * </pre>
   *
   * <code>UNSPECIFIED = 1;</code>
   */
  UNSPECIFIED(1),
  /**
   * <pre>
   * Internal error.
   * </pre>
   *
   * <code>INTERNAL = 2;</code>
   */
  INTERNAL(2),
  /**
   * <pre>
   * The requested resource wasn't found.
   * </pre>
   *
   * <code>NOT_FOUND = 3;</code>
   */
  NOT_FOUND(3),
  /**
   * <pre>
   * The request was bad/wrong is some way.
   * </pre>
   *
   * <code>BAD_REQUEST = 4;</code>
   */
  BAD_REQUEST(4),
  /**
   * <pre>
   * A failure to marshal a response.
   * </pre>
   *
   * <code>MARSHAL_FAILURE = 5;</code>
   */
  MARSHAL_FAILURE(5),
  /**
   * <pre>
   * A failure to unmarshal a request.
   * </pre>
   *
   * <code>UNMARSHAL_FAILURE = 6;</code>
   */
  UNMARSHAL_FAILURE(6),
  /**
   * <pre>
   * A parameter is missing.
   * </pre>
   *
   * <code>PARAMETER_MISSING = 7;</code>
   */
  PARAMETER_MISSING(7),
  /**
   * <pre>
   *&#47; A parameter is invalid.
   * </pre>
   *
   * <code>PARAMETER_INVALID = 8;</code>
   */
  PARAMETER_INVALID(8),
  UNRECOGNIZED(-1),
  ;

  /**
   * <pre>
   * Default value for the error code. Don't set the error code to this. Use Unspecified if tempted.
   * </pre>
   *
   * <code>UNSET = 0;</code>
   */
  public static final int UNSET_VALUE = 0;
  /**
   * <pre>
   * An error occurred, but the kind wasn't specified or included in the list.
   * </pre>
   *
   * <code>UNSPECIFIED = 1;</code>
   */
  public static final int UNSPECIFIED_VALUE = 1;
  /**
   * <pre>
   * Internal error.
   * </pre>
   *
   * <code>INTERNAL = 2;</code>
   */
  public static final int INTERNAL_VALUE = 2;
  /**
   * <pre>
   * The requested resource wasn't found.
   * </pre>
   *
   * <code>NOT_FOUND = 3;</code>
   */
  public static final int NOT_FOUND_VALUE = 3;
  /**
   * <pre>
   * The request was bad/wrong is some way.
   * </pre>
   *
   * <code>BAD_REQUEST = 4;</code>
   */
  public static final int BAD_REQUEST_VALUE = 4;
  /**
   * <pre>
   * A failure to marshal a response.
   * </pre>
   *
   * <code>MARSHAL_FAILURE = 5;</code>
   */
  public static final int MARSHAL_FAILURE_VALUE = 5;
  /**
   * <pre>
   * A failure to unmarshal a request.
   * </pre>
   *
   * <code>UNMARSHAL_FAILURE = 6;</code>
   */
  public static final int UNMARSHAL_FAILURE_VALUE = 6;
  /**
   * <pre>
   * A parameter is missing.
   * </pre>
   *
   * <code>PARAMETER_MISSING = 7;</code>
   */
  public static final int PARAMETER_MISSING_VALUE = 7;
  /**
   * <pre>
   *&#47; A parameter is invalid.
   * </pre>
   *
   * <code>PARAMETER_INVALID = 8;</code>
   */
  public static final int PARAMETER_INVALID_VALUE = 8;


  public final int getNumber() {
    if (this == UNRECOGNIZED) {
      throw new java.lang.IllegalArgumentException(
          "Can't get the number of an unknown enum value.");
    }
    return value;
  }

  /**
   * @param value The numeric wire value of the corresponding enum entry.
   * @return The enum associated with the given numeric wire value.
   * @deprecated Use {@link #forNumber(int)} instead.
   */
  @java.lang.Deprecated
  public static ErrorCode valueOf(int value) {
    return forNumber(value);
  }

  /**
   * @param value The numeric wire value of the corresponding enum entry.
   * @return The enum associated with the given numeric wire value.
   */
  public static ErrorCode forNumber(int value) {
    switch (value) {
      case 0: return UNSET;
      case 1: return UNSPECIFIED;
      case 2: return INTERNAL;
      case 3: return NOT_FOUND;
      case 4: return BAD_REQUEST;
      case 5: return MARSHAL_FAILURE;
      case 6: return UNMARSHAL_FAILURE;
      case 7: return PARAMETER_MISSING;
      case 8: return PARAMETER_INVALID;
      default: return null;
    }
  }

  public static com.google.protobuf.Internal.EnumLiteMap<ErrorCode>
      internalGetValueMap() {
    return internalValueMap;
  }
  private static final com.google.protobuf.Internal.EnumLiteMap<
      ErrorCode> internalValueMap =
        new com.google.protobuf.Internal.EnumLiteMap<ErrorCode>() {
          public ErrorCode findValueByNumber(int number) {
            return ErrorCode.forNumber(number);
          }
        };

  public final com.google.protobuf.Descriptors.EnumValueDescriptor
      getValueDescriptor() {
    if (this == UNRECOGNIZED) {
      throw new java.lang.IllegalStateException(
          "Can't get the descriptor of an unrecognized enum value.");
    }
    return getDescriptor().getValues().get(ordinal());
  }
  public final com.google.protobuf.Descriptors.EnumDescriptor
      getDescriptorForType() {
    return getDescriptor();
  }
  public static final com.google.protobuf.Descriptors.EnumDescriptor
      getDescriptor() {
    return org.cyverse.de.protobufs.ServiceErrorProtobufs.getDescriptor().getEnumTypes().get(0);
  }

  private static final ErrorCode[] VALUES = values();

  public static ErrorCode valueOf(
      com.google.protobuf.Descriptors.EnumValueDescriptor desc) {
    if (desc.getType() != getDescriptor()) {
      throw new java.lang.IllegalArgumentException(
        "EnumValueDescriptor is not for this type.");
    }
    if (desc.getIndex() == -1) {
      return UNRECOGNIZED;
    }
    return VALUES[desc.getIndex()];
  }

  private final int value;

  private ErrorCode(int value) {
    this.value = value;
  }

  // @@protoc_insertion_point(enum_scope:svcerror.ErrorCode)
}

