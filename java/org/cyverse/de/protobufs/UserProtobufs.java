// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: user.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public final class UserProtobufs {
  private UserProtobufs() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_user_Preferences_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_user_Preferences_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_user_Login_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_user_Login_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_user_SavedSearches_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_user_SavedSearches_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_user_User_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_user_User_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\nuser.proto\022\004user\032\037google/protobuf/time" +
      "stamp.proto\"0\n\013Preferences\022\014\n\004uuid\030\001 \001(\t" +
      "\022\023\n\013preferences\030\002 \001(\t\"\236\001\n\005Login\022\014\n\004uuid\030" +
      "\001 \001(\t\022\022\n\nip_address\030\002 \001(\t\022\022\n\nuser_agent\030" +
      "\003 \001(\t\022.\n\nlogin_time\030\004 \001(\0132\032.google.proto" +
      "buf.Timestamp\022/\n\013logout_time\030\005 \001(\0132\032.goo" +
      "gle.protobuf.Timestamp\"5\n\rSavedSearches\022" +
      "\014\n\004uuid\030\001 \001(\t\022\026\n\016saved_searches\030\002 \001(\t\"&\n" +
      "\004User\022\014\n\004uuid\030\001 \001(\t\022\020\n\010username\030\002 \001(\tBL\n" +
      "\030org.cyverse.de.protobufsB\rUserProtobufs" +
      "P\001Z\037github.com/cyverse-de/p/go/userb\006pro" +
      "to3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          com.google.protobuf.TimestampProto.getDescriptor(),
        });
    internal_static_user_Preferences_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_user_Preferences_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_user_Preferences_descriptor,
        new java.lang.String[] { "Uuid", "Preferences", });
    internal_static_user_Login_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_user_Login_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_user_Login_descriptor,
        new java.lang.String[] { "Uuid", "IpAddress", "UserAgent", "LoginTime", "LogoutTime", });
    internal_static_user_SavedSearches_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_user_SavedSearches_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_user_SavedSearches_descriptor,
        new java.lang.String[] { "Uuid", "SavedSearches", });
    internal_static_user_User_descriptor =
      getDescriptor().getMessageTypes().get(3);
    internal_static_user_User_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_user_User_descriptor,
        new java.lang.String[] { "Uuid", "Username", });
    com.google.protobuf.TimestampProto.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
