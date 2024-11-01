// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: containers.proto
// Protobuf Java Version: 4.28.3

package org.cyverse.de.protobufs;

public interface ImageOrBuilder extends
    // @@protoc_insertion_point(interface_extends:containers.Image)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The UUID of the container image.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The id.
   */
  java.lang.String getId();
  /**
   * <pre>
   * The UUID of the container image.
   * </pre>
   *
   * <code>string id = 1;</code>
   * @return The bytes for id.
   */
  com.google.protobuf.ByteString
      getIdBytes();

  /**
   * <pre>
   * The name of the container image.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The name.
   */
  java.lang.String getName();
  /**
   * <pre>
   * The name of the container image.
   * </pre>
   *
   * <code>string name = 2;</code>
   * @return The bytes for name.
   */
  com.google.protobuf.ByteString
      getNameBytes();

  /**
   * <pre>
   * The container image's tag.
   * </pre>
   *
   * <code>string tag = 3;</code>
   * @return The tag.
   */
  java.lang.String getTag();
  /**
   * <pre>
   * The container image's tag.
   * </pre>
   *
   * <code>string tag = 3;</code>
   * @return The bytes for tag.
   */
  com.google.protobuf.ByteString
      getTagBytes();

  /**
   * <pre>
   * The URL for the image.
   * </pre>
   *
   * <code>string url = 5;</code>
   * @return The url.
   */
  java.lang.String getUrl();
  /**
   * <pre>
   * The URL for the image.
   * </pre>
   *
   * <code>string url = 5;</code>
   * @return The bytes for url.
   */
  com.google.protobuf.ByteString
      getUrlBytes();

  /**
   * <pre>
   * The path to the image in OSG. Might be blank.
   * </pre>
   *
   * <code>string osg_image_path = 6 [json_name = "osg_image_path"];</code>
   * @return The osgImagePath.
   */
  java.lang.String getOsgImagePath();
  /**
   * <pre>
   * The path to the image in OSG. Might be blank.
   * </pre>
   *
   * <code>string osg_image_path = 6 [json_name = "osg_image_path"];</code>
   * @return The bytes for osgImagePath.
   */
  com.google.protobuf.ByteString
      getOsgImagePathBytes();
}
