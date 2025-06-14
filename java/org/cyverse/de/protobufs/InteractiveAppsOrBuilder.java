// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: containers.proto
// Protobuf Java Version: 4.31.1

package org.cyverse.de.protobufs;

@com.google.protobuf.Generated
public interface InteractiveAppsOrBuilder extends
    // @@protoc_insertion_point(interface_extends:containers.InteractiveApps)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The image containing the proxy application that requests to a VICE app flow through.
   * </pre>
   *
   * <code>string proxy_image = 1;</code>
   * @return The proxyImage.
   */
  java.lang.String getProxyImage();
  /**
   * <pre>
   * The image containing the proxy application that requests to a VICE app flow through.
   * </pre>
   *
   * <code>string proxy_image = 1;</code>
   * @return The bytes for proxyImage.
   */
  com.google.protobuf.ByteString
      getProxyImageBytes();

  /**
   * <pre>
   * The name of the proxy container.
   * </pre>
   *
   * <code>string proxy_name = 2;</code>
   * @return The proxyName.
   */
  java.lang.String getProxyName();
  /**
   * <pre>
   * The name of the proxy container.
   * </pre>
   *
   * <code>string proxy_name = 2;</code>
   * @return The bytes for proxyName.
   */
  com.google.protobuf.ByteString
      getProxyNameBytes();

  /**
   * <pre>
   * The URL the proxy will redirect users to when necessary.
   * </pre>
   *
   * <code>string frontend_url = 3;</code>
   * @return The frontendUrl.
   */
  java.lang.String getFrontendUrl();
  /**
   * <pre>
   * The URL the proxy will redirect users to when necessary.
   * </pre>
   *
   * <code>string frontend_url = 3;</code>
   * @return The bytes for frontendUrl.
   */
  com.google.protobuf.ByteString
      getFrontendUrlBytes();

  /**
   * <pre>
   * The URL to the CAS instance needed for authentication.
   * </pre>
   *
   * <code>string cas_url = 4;</code>
   * @return The casUrl.
   */
  java.lang.String getCasUrl();
  /**
   * <pre>
   * The URL to the CAS instance needed for authentication.
   * </pre>
   *
   * <code>string cas_url = 4;</code>
   * @return The bytes for casUrl.
   */
  com.google.protobuf.ByteString
      getCasUrlBytes();

  /**
   * <pre>
   * The URL to the CAS authentication validation.
   * </pre>
   *
   * <code>string cas_validate = 5;</code>
   * @return The casValidate.
   */
  java.lang.String getCasValidate();
  /**
   * <pre>
   * The URL to the CAS authentication validation.
   * </pre>
   *
   * <code>string cas_validate = 5;</code>
   * @return The bytes for casValidate.
   */
  com.google.protobuf.ByteString
      getCasValidateBytes();

  /**
   * <pre>
   * The path to the SSL cert in the container.
   * </pre>
   *
   * <code>string ssl_cert_path = 6;</code>
   * @return The sslCertPath.
   */
  java.lang.String getSslCertPath();
  /**
   * <pre>
   * The path to the SSL cert in the container.
   * </pre>
   *
   * <code>string ssl_cert_path = 6;</code>
   * @return The bytes for sslCertPath.
   */
  com.google.protobuf.ByteString
      getSslCertPathBytes();

  /**
   * <pre>
   * The path to the SSL key in the container.
   * </pre>
   *
   * <code>string ssl_key_path = 7;</code>
   * @return The sslKeyPath.
   */
  java.lang.String getSslKeyPath();
  /**
   * <pre>
   * The path to the SSL key in the container.
   * </pre>
   *
   * <code>string ssl_key_path = 7;</code>
   * @return The bytes for sslKeyPath.
   */
  com.google.protobuf.ByteString
      getSslKeyPathBytes();

  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string websocket_path = 8;</code>
   * @return The websocketPath.
   */
  java.lang.String getWebsocketPath();
  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string websocket_path = 8;</code>
   * @return The bytes for websocketPath.
   */
  com.google.protobuf.ByteString
      getWebsocketPathBytes();

  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string websocket_port = 9;</code>
   * @return The websocketPort.
   */
  java.lang.String getWebsocketPort();
  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string websocket_port = 9;</code>
   * @return The bytes for websocketPort.
   */
  com.google.protobuf.ByteString
      getWebsocketPortBytes();

  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string websocket_proto = 10;</code>
   * @return The websocketProto.
   */
  java.lang.String getWebsocketProto();
  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string websocket_proto = 10;</code>
   * @return The bytes for websocketProto.
   */
  com.google.protobuf.ByteString
      getWebsocketProtoBytes();

  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string backend_url = 11;</code>
   * @return The backendUrl.
   */
  java.lang.String getBackendUrl();
  /**
   * <pre>
   * Unused.
   * </pre>
   *
   * <code>string backend_url = 11;</code>
   * @return The bytes for backendUrl.
   */
  com.google.protobuf.ByteString
      getBackendUrlBytes();
}
