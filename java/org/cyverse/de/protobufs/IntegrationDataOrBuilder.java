// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: apps.proto

package org.cyverse.de.protobufs;

public interface IntegrationDataOrBuilder extends
    // @@protoc_insertion_point(interface_extends:apps.IntegrationData)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The UUID of the integration data.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The id.
   */
  java.lang.String getId();
  /**
   * <pre>
   * The UUID of the integration data.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The bytes for id.
   */
  com.google.protobuf.ByteString
      getIdBytes();

  /**
   * <pre>
   * The name of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
   * @return The integratorName.
   */
  java.lang.String getIntegratorName();
  /**
   * <pre>
   * The name of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_name = 2 [json_name = "integrator_name"];</code>
   * @return The bytes for integratorName.
   */
  com.google.protobuf.ByteString
      getIntegratorNameBytes();

  /**
   * <pre>
   * The email of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
   * @return The integratorEmail.
   */
  java.lang.String getIntegratorEmail();
  /**
   * <pre>
   * The email of the person that integrated stuff.
   * </pre>
   *
   * <code>string integrator_email = 3 [json_name = "integrator_email"];</code>
   * @return The bytes for integratorEmail.
   */
  com.google.protobuf.ByteString
      getIntegratorEmailBytes();

  /**
   * <pre>
   * The user information of the integrator.
   * </pre>
   *
   * <code>.user.User user = 4;</code>
   * @return Whether the user field is set.
   */
  boolean hasUser();
  /**
   * <pre>
   * The user information of the integrator.
   * </pre>
   *
   * <code>.user.User user = 4;</code>
   * @return The user.
   */
  org.cyverse.de.protobufs.User getUser();
  /**
   * <pre>
   * The user information of the integrator.
   * </pre>
   *
   * <code>.user.User user = 4;</code>
   */
  org.cyverse.de.protobufs.UserOrBuilder getUserOrBuilder();
}