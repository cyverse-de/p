// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: analysis_container.proto

package org.cyverse.de.protobufs;

public interface ContainerOrBuilder extends
    // @@protoc_insertion_point(interface_extends:Container)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>string id = 1;</code>
   * @return The id.
   */
  java.lang.String getId();
  /**
   * <code>string id = 1;</code>
   * @return The bytes for id.
   */
  com.google.protobuf.ByteString
      getIdBytes();

  /**
   * <code>repeated .Container.Volume volumes = 2 [json_name = "container_volumes"];</code>
   */
  java.util.List<org.cyverse.de.protobufs.Container.Volume> 
      getVolumesList();
  /**
   * <code>repeated .Container.Volume volumes = 2 [json_name = "container_volumes"];</code>
   */
  org.cyverse.de.protobufs.Container.Volume getVolumes(int index);
  /**
   * <code>repeated .Container.Volume volumes = 2 [json_name = "container_volumes"];</code>
   */
  int getVolumesCount();
  /**
   * <code>repeated .Container.Volume volumes = 2 [json_name = "container_volumes"];</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.Container.VolumeOrBuilder> 
      getVolumesOrBuilderList();
  /**
   * <code>repeated .Container.Volume volumes = 2 [json_name = "container_volumes"];</code>
   */
  org.cyverse.de.protobufs.Container.VolumeOrBuilder getVolumesOrBuilder(
      int index);

  /**
   * <code>repeated .Container.Device devices = 3 [json_name = "container_devices"];</code>
   */
  java.util.List<org.cyverse.de.protobufs.Container.Device> 
      getDevicesList();
  /**
   * <code>repeated .Container.Device devices = 3 [json_name = "container_devices"];</code>
   */
  org.cyverse.de.protobufs.Container.Device getDevices(int index);
  /**
   * <code>repeated .Container.Device devices = 3 [json_name = "container_devices"];</code>
   */
  int getDevicesCount();
  /**
   * <code>repeated .Container.Device devices = 3 [json_name = "container_devices"];</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.Container.DeviceOrBuilder> 
      getDevicesOrBuilderList();
  /**
   * <code>repeated .Container.Device devices = 3 [json_name = "container_devices"];</code>
   */
  org.cyverse.de.protobufs.Container.DeviceOrBuilder getDevicesOrBuilder(
      int index);

  /**
   * <code>repeated .Container.VolumesFrom volumes_from = 4 [json_name = "container_volumes_from"];</code>
   */
  java.util.List<org.cyverse.de.protobufs.Container.VolumesFrom> 
      getVolumesFromList();
  /**
   * <code>repeated .Container.VolumesFrom volumes_from = 4 [json_name = "container_volumes_from"];</code>
   */
  org.cyverse.de.protobufs.Container.VolumesFrom getVolumesFrom(int index);
  /**
   * <code>repeated .Container.VolumesFrom volumes_from = 4 [json_name = "container_volumes_from"];</code>
   */
  int getVolumesFromCount();
  /**
   * <code>repeated .Container.VolumesFrom volumes_from = 4 [json_name = "container_volumes_from"];</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.Container.VolumesFromOrBuilder> 
      getVolumesFromOrBuilderList();
  /**
   * <code>repeated .Container.VolumesFrom volumes_from = 4 [json_name = "container_volumes_from"];</code>
   */
  org.cyverse.de.protobufs.Container.VolumesFromOrBuilder getVolumesFromOrBuilder(
      int index);

  /**
   * <code>string name = 5;</code>
   * @return The name.
   */
  java.lang.String getName();
  /**
   * <code>string name = 5;</code>
   * @return The bytes for name.
   */
  com.google.protobuf.ByteString
      getNameBytes();

  /**
   * <code>string network_mode = 6 [json_name = "network_mode"];</code>
   * @return The networkMode.
   */
  java.lang.String getNetworkMode();
  /**
   * <code>string network_mode = 6 [json_name = "network_mode"];</code>
   * @return The bytes for networkMode.
   */
  com.google.protobuf.ByteString
      getNetworkModeBytes();

  /**
   * <code>int64 cpu_shares = 7 [json_name = "cpu_shares"];</code>
   * @return The cpuShares.
   */
  long getCpuShares();

  /**
   * <code>.InteractiveApps interactive_apps = 8 [json_name = "interactive_apps"];</code>
   * @return Whether the interactiveApps field is set.
   */
  boolean hasInteractiveApps();
  /**
   * <code>.InteractiveApps interactive_apps = 8 [json_name = "interactive_apps"];</code>
   * @return The interactiveApps.
   */
  org.cyverse.de.protobufs.InteractiveApps getInteractiveApps();
  /**
   * <code>.InteractiveApps interactive_apps = 8 [json_name = "interactive_apps"];</code>
   */
  org.cyverse.de.protobufs.InteractiveAppsOrBuilder getInteractiveAppsOrBuilder();

  /**
   * <code>int64 memory_limit = 9 [json_name = "memory_limit"];</code>
   * @return The memoryLimit.
   */
  long getMemoryLimit();

  /**
   * <code>int64 min_memory_limit = 10 [json_name = "min_memory_limit"];</code>
   * @return The minMemoryLimit.
   */
  long getMinMemoryLimit();

  /**
   * <code>float max_cpu_cores = 11 [json_name = "max_cpu_cores"];</code>
   * @return The maxCpuCores.
   */
  float getMaxCpuCores();

  /**
   * <code>float min_cpu_cores = 12 [json_name = "min_cpu_cores"];</code>
   * @return The minCpuCores.
   */
  float getMinCpuCores();

  /**
   * <code>int64 min_disk_space = 13 [json_name = "min_disk_space"];</code>
   * @return The minDiskSpace.
   */
  long getMinDiskSpace();

  /**
   * <code>int64 pids_limit = 14 [json_name = "pids_limit"];</code>
   * @return The pidsLimit.
   */
  long getPidsLimit();

  /**
   * <code>.Container.Image image = 15;</code>
   * @return Whether the image field is set.
   */
  boolean hasImage();
  /**
   * <code>.Container.Image image = 15;</code>
   * @return The image.
   */
  org.cyverse.de.protobufs.Container.Image getImage();
  /**
   * <code>.Container.Image image = 15;</code>
   */
  org.cyverse.de.protobufs.Container.ImageOrBuilder getImageOrBuilder();

  /**
   * <code>string entry_point = 16 [json_name = "entrypoint"];</code>
   * @return The entryPoint.
   */
  java.lang.String getEntryPoint();
  /**
   * <code>string entry_point = 16 [json_name = "entrypoint"];</code>
   * @return The bytes for entryPoint.
   */
  com.google.protobuf.ByteString
      getEntryPointBytes();

  /**
   * <code>string working_dir = 17 [json_name = "working_directory"];</code>
   * @return The workingDir.
   */
  java.lang.String getWorkingDir();
  /**
   * <code>string working_dir = 17 [json_name = "working_directory"];</code>
   * @return The bytes for workingDir.
   */
  com.google.protobuf.ByteString
      getWorkingDirBytes();

  /**
   * <code>repeated .Container.Port ports = 18;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Container.Port> 
      getPortsList();
  /**
   * <code>repeated .Container.Port ports = 18;</code>
   */
  org.cyverse.de.protobufs.Container.Port getPorts(int index);
  /**
   * <code>repeated .Container.Port ports = 18;</code>
   */
  int getPortsCount();
  /**
   * <code>repeated .Container.Port ports = 18;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.Container.PortOrBuilder> 
      getPortsOrBuilderList();
  /**
   * <code>repeated .Container.Port ports = 18;</code>
   */
  org.cyverse.de.protobufs.Container.PortOrBuilder getPortsOrBuilder(
      int index);

  /**
   * <code>bool skip_tmp_mount = 19 [json_name = "skip_tmp_mount"];</code>
   * @return The skipTmpMount.
   */
  boolean getSkipTmpMount();

  /**
   * <code>int32 uid = 20 [json_name = "uid"];</code>
   * @return The uid.
   */
  int getUid();

  /**
   * <code>.Header header = 21;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <code>.Header header = 21;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <code>.Header header = 21;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();
}
