// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: user_requests.proto
// Protobuf Java Version: 4.31.1

package org.cyverse.de.protobufs;

@com.google.protobuf.Generated
public final class UserRequestsProtobufs {
  private UserRequestsProtobufs() {}
  static {
    com.google.protobuf.RuntimeVersion.validateProtobufGencodeVersion(
      com.google.protobuf.RuntimeVersion.RuntimeDomain.PUBLIC,
      /* major= */ 4,
      /* minor= */ 31,
      /* patch= */ 1,
      /* suffix= */ "",
      UserRequestsProtobufs.class.getName());
  }
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_user_requests_UserLookupRequest_descriptor;
  static final 
    com.google.protobuf.GeneratedMessage.FieldAccessorTable
      internal_static_user_requests_UserLookupRequest_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\023user_requests.proto\022\ruser_requests\032\014he" +
      "ader.proto\"\205\002\n\021UserLookupRequest\022\022\n\010user" +
      "name\030\001 \001(\tH\000\022\021\n\007user_id\030\002 \001(\tH\000\022\025\n\013analy" +
      "sis_id\030\003 \001(\tH\000\022\026\n\016include_logins\030\005 \001(\010\022\033" +
      "\n\023include_preferences\030\006 \001(\010\022\036\n\026include_s" +
      "aved_searches\030\007 \001(\010\022\023\n\013login_limit\030\010 \001(\r" +
      "\022\024\n\014login_offset\030\t \001(\r\022\036\n\006header\030\n \001(\0132\016" +
      ".header.HeaderB\014\n\nlookup_idsJ\004\010\004\020\005BT\n\030or" +
      "g.cyverse.de.protobufsB\025UserRequestsProt" +
      "obufsP\001Z\037github.com/cyverse-de/p/go/user" +
      "b\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor(),
        });
    internal_static_user_requests_UserLookupRequest_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_user_requests_UserLookupRequest_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessage.FieldAccessorTable(
        internal_static_user_requests_UserLookupRequest_descriptor,
        new java.lang.String[] { "Username", "UserId", "AnalysisId", "IncludeLogins", "IncludePreferences", "IncludeSavedSearches", "LoginLimit", "LoginOffset", "Header", "LookupIds", });
    descriptor.resolveAllFeaturesImmutable();
    org.cyverse.de.protobufs.HeaderProtobufs.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
