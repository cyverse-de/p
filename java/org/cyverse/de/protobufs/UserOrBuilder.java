// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: user.proto

package org.cyverse.de.protobufs;

public interface UserOrBuilder extends
    // @@protoc_insertion_point(interface_extends:User)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>string uuid = 1;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <code>string uuid = 1;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <code>string username = 2;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <code>string username = 2;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();

  /**
   * <code>.User.Preferences preferences = 3;</code>
   * @return Whether the preferences field is set.
   */
  boolean hasPreferences();
  /**
   * <code>.User.Preferences preferences = 3;</code>
   * @return The preferences.
   */
  org.cyverse.de.protobufs.User.Preferences getPreferences();
  /**
   * <code>.User.Preferences preferences = 3;</code>
   */
  org.cyverse.de.protobufs.User.PreferencesOrBuilder getPreferencesOrBuilder();

  /**
   * <code>repeated .User.Login logins = 4;</code>
   */
  java.util.List<org.cyverse.de.protobufs.User.Login> 
      getLoginsList();
  /**
   * <code>repeated .User.Login logins = 4;</code>
   */
  org.cyverse.de.protobufs.User.Login getLogins(int index);
  /**
   * <code>repeated .User.Login logins = 4;</code>
   */
  int getLoginsCount();
  /**
   * <code>repeated .User.Login logins = 4;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.User.LoginOrBuilder> 
      getLoginsOrBuilderList();
  /**
   * <code>repeated .User.Login logins = 4;</code>
   */
  org.cyverse.de.protobufs.User.LoginOrBuilder getLoginsOrBuilder(
      int index);

  /**
   * <code>uint32 login_count = 7;</code>
   * @return The loginCount.
   */
  int getLoginCount();

  /**
   * <code>.User.SavedSearches saved_searches = 8;</code>
   * @return Whether the savedSearches field is set.
   */
  boolean hasSavedSearches();
  /**
   * <code>.User.SavedSearches saved_searches = 8;</code>
   * @return The savedSearches.
   */
  org.cyverse.de.protobufs.User.SavedSearches getSavedSearches();
  /**
   * <code>.User.SavedSearches saved_searches = 8;</code>
   */
  org.cyverse.de.protobufs.User.SavedSearchesOrBuilder getSavedSearchesOrBuilder();

  /**
   * <code>.Header header = 9;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <code>.Header header = 9;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <code>.Header header = 9;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <code>.ServiceError error = 10;</code>
   * @return Whether the error field is set.
   */
  boolean hasError();
  /**
   * <code>.ServiceError error = 10;</code>
   * @return The error.
   */
  org.cyverse.de.protobufs.ServiceError getError();
  /**
   * <code>.ServiceError error = 10;</code>
   */
  org.cyverse.de.protobufs.ServiceErrorOrBuilder getErrorOrBuilder();
}
