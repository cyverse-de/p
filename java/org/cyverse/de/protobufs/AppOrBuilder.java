// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: apps.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface AppOrBuilder extends
    // @@protoc_insertion_point(interface_extends:apps.App)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The UUID for the app.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The id.
   */
  java.lang.String getId();
  /**
   * <pre>
   * The UUID for the app.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The bytes for id.
   */
  com.google.protobuf.ByteString
      getIdBytes();

  /**
   * <pre>
   * The name of the app.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The name.
   */
  java.lang.String getName();
  /**
   * <pre>
   * The name of the app.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The bytes for name.
   */
  com.google.protobuf.ByteString
      getNameBytes();

  /**
   * <pre>
   * The description of the app.
   * </pre>
   *
   * <code>string description = 3;</code>
   * @return The description.
   */
  java.lang.String getDescription();
  /**
   * <pre>
   * The description of the app.
   * </pre>
   *
   * <code>string description = 3;</code>
   * @return The bytes for description.
   */
  com.google.protobuf.ByteString
      getDescriptionBytes();

  /**
   * <pre>
   * The wiki URL of the app.
   * </pre>
   *
   * <code>string wiki_url = 4 [json_name = "wiki_url"];</code>
   * @return The wikiUrl.
   */
  java.lang.String getWikiUrl();
  /**
   * <pre>
   * The wiki URL of the app.
   * </pre>
   *
   * <code>string wiki_url = 4 [json_name = "wiki_url"];</code>
   * @return The bytes for wikiUrl.
   */
  com.google.protobuf.ByteString
      getWikiUrlBytes();
}
