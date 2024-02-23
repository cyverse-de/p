// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: containers.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface DeviceOrBuilder extends
    // @@protoc_insertion_point(interface_extends:containers.Device)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The path to the device on the host.
   * </pre>
   *
   * <code>string host_path = 1 [json_name = "host_path"];</code>
   * @return The hostPath.
   */
  java.lang.String getHostPath();
  /**
   * <pre>
   * The path to the device on the host.
   * </pre>
   *
   * <code>string host_path = 1 [json_name = "host_path"];</code>
   * @return The bytes for hostPath.
   */
  com.google.protobuf.ByteString
      getHostPathBytes();

  /**
   * <pre>
   * The path to the device in the container.
   * </pre>
   *
   * <code>string container_path = 2 [json_name = "container_path"];</code>
   * @return The containerPath.
   */
  java.lang.String getContainerPath();
  /**
   * <pre>
   * The path to the device in the container.
   * </pre>
   *
   * <code>string container_path = 2 [json_name = "container_path"];</code>
   * @return The bytes for containerPath.
   */
  com.google.protobuf.ByteString
      getContainerPathBytes();

  /**
   * <pre>
   * The permissions needed to mount the device.
   * </pre>
   *
   * <code>string cgroup_permissions = 3 [json_name = "cgroup_permissions"];</code>
   * @return The cgroupPermissions.
   */
  java.lang.String getCgroupPermissions();
  /**
   * <pre>
   * The permissions needed to mount the device.
   * </pre>
   *
   * <code>string cgroup_permissions = 3 [json_name = "cgroup_permissions"];</code>
   * @return The bytes for cgroupPermissions.
   */
  com.google.protobuf.ByteString
      getCgroupPermissionsBytes();
}
