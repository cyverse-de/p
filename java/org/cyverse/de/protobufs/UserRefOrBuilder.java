// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: user.proto

package org.cyverse.de.protobufs;

public interface UserRefOrBuilder extends
    // @@protoc_insertion_point(interface_extends:user.UserRef)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The username of the user in the database. Must be unique.
   * It's more likely for a service to have this, which is why
   * it's listed first. Writing services to use the username
   * can skip a lookup of the UUID.
   * </pre>
   *
   * <code>optional string username = 1;</code>
   * @return Whether the username field is set.
   */
  boolean hasUsername();
  /**
   * <pre>
   * The username of the user in the database. Must be unique.
   * It's more likely for a service to have this, which is why
   * it's listed first. Writing services to use the username
   * can skip a lookup of the UUID.
   * </pre>
   *
   * <code>optional string username = 1;</code>
   * @return The username.
   */
  java.lang.String getUsername();
  /**
   * <pre>
   * The username of the user in the database. Must be unique.
   * It's more likely for a service to have this, which is why
   * it's listed first. Writing services to use the username
   * can skip a lookup of the UUID.
   * </pre>
   *
   * <code>optional string username = 1;</code>
   * @return The bytes for username.
   */
  com.google.protobuf.ByteString
      getUsernameBytes();

  /**
   * <pre>
   * The UUID of the user in the database. A service can have
   * this, but it's more likely for it to have the username.
   * </pre>
   *
   * <code>optional string uuid = 2;</code>
   * @return Whether the uuid field is set.
   */
  boolean hasUuid();
  /**
   * <pre>
   * The UUID of the user in the database. A service can have
   * this, but it's more likely for it to have the username.
   * </pre>
   *
   * <code>optional string uuid = 2;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <pre>
   * The UUID of the user in the database. A service can have
   * this, but it's more likely for it to have the username.
   * </pre>
   *
   * <code>optional string uuid = 2;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();
}