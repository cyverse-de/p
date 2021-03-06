// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: analysis_container.proto

package org.cyverse.de.protobufs;

public interface InteractiveAppsOrBuilder extends
    // @@protoc_insertion_point(interface_extends:InteractiveApps)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>string proxy_image = 1;</code>
   * @return The proxyImage.
   */
  java.lang.String getProxyImage();
  /**
   * <code>string proxy_image = 1;</code>
   * @return The bytes for proxyImage.
   */
  com.google.protobuf.ByteString
      getProxyImageBytes();

  /**
   * <code>string proxy_name = 2;</code>
   * @return The proxyName.
   */
  java.lang.String getProxyName();
  /**
   * <code>string proxy_name = 2;</code>
   * @return The bytes for proxyName.
   */
  com.google.protobuf.ByteString
      getProxyNameBytes();

  /**
   * <code>string frontend_url = 3;</code>
   * @return The frontendUrl.
   */
  java.lang.String getFrontendUrl();
  /**
   * <code>string frontend_url = 3;</code>
   * @return The bytes for frontendUrl.
   */
  com.google.protobuf.ByteString
      getFrontendUrlBytes();

  /**
   * <code>string cas_url = 4;</code>
   * @return The casUrl.
   */
  java.lang.String getCasUrl();
  /**
   * <code>string cas_url = 4;</code>
   * @return The bytes for casUrl.
   */
  com.google.protobuf.ByteString
      getCasUrlBytes();

  /**
   * <code>string cas_validate = 5;</code>
   * @return The casValidate.
   */
  java.lang.String getCasValidate();
  /**
   * <code>string cas_validate = 5;</code>
   * @return The bytes for casValidate.
   */
  com.google.protobuf.ByteString
      getCasValidateBytes();

  /**
   * <code>string ssl_cert_path = 6;</code>
   * @return The sslCertPath.
   */
  java.lang.String getSslCertPath();
  /**
   * <code>string ssl_cert_path = 6;</code>
   * @return The bytes for sslCertPath.
   */
  com.google.protobuf.ByteString
      getSslCertPathBytes();

  /**
   * <code>string ssl_key_path = 7;</code>
   * @return The sslKeyPath.
   */
  java.lang.String getSslKeyPath();
  /**
   * <code>string ssl_key_path = 7;</code>
   * @return The bytes for sslKeyPath.
   */
  com.google.protobuf.ByteString
      getSslKeyPathBytes();

  /**
   * <code>string websocket_path = 8;</code>
   * @return The websocketPath.
   */
  java.lang.String getWebsocketPath();
  /**
   * <code>string websocket_path = 8;</code>
   * @return The bytes for websocketPath.
   */
  com.google.protobuf.ByteString
      getWebsocketPathBytes();

  /**
   * <code>string websocket_port = 9;</code>
   * @return The websocketPort.
   */
  java.lang.String getWebsocketPort();
  /**
   * <code>string websocket_port = 9;</code>
   * @return The bytes for websocketPort.
   */
  com.google.protobuf.ByteString
      getWebsocketPortBytes();

  /**
   * <code>string websocket_proto = 10;</code>
   * @return The websocketProto.
   */
  java.lang.String getWebsocketProto();
  /**
   * <code>string websocket_proto = 10;</code>
   * @return The bytes for websocketProto.
   */
  com.google.protobuf.ByteString
      getWebsocketProtoBytes();

  /**
   * <code>string backend_url = 11;</code>
   * @return The backendUrl.
   */
  java.lang.String getBackendUrl();
  /**
   * <code>string backend_url = 11;</code>
   * @return The bytes for backendUrl.
   */
  com.google.protobuf.ByteString
      getBackendUrlBytes();

  /**
   * <code>.Header header = 12;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <code>.Header header = 12;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <code>.Header header = 12;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();
}
