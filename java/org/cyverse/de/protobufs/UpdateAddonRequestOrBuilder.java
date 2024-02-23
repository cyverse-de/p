// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_addons.proto

// Protobuf Java Version: 3.25.3
package org.cyverse.de.protobufs;

public interface UpdateAddonRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.UpdateAddonRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * Contains telemetry information.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <pre>
   * Contains telemetry information.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   * @return The header.
   */
  org.cyverse.de.protobufs.Header getHeader();
  /**
   * <pre>
   * Contains telemetry information.
   * </pre>
   *
   * <code>.header.Header header = 1;</code>
   */
  org.cyverse.de.protobufs.HeaderOrBuilder getHeaderOrBuilder();

  /**
   * <pre>
   * The values to set in the update.
   * </pre>
   *
   * <code>.qms.Addon addon = 2;</code>
   * @return Whether the addon field is set.
   */
  boolean hasAddon();
  /**
   * <pre>
   * The values to set in the update.
   * </pre>
   *
   * <code>.qms.Addon addon = 2;</code>
   * @return The addon.
   */
  org.cyverse.de.protobufs.Addon getAddon();
  /**
   * <pre>
   * The values to set in the update.
   * </pre>
   *
   * <code>.qms.Addon addon = 2;</code>
   */
  org.cyverse.de.protobufs.AddonOrBuilder getAddonOrBuilder();

  /**
   * <pre>
   * Whether to update the name of the addon.
   * </pre>
   *
   * <code>bool update_name = 3;</code>
   * @return The updateName.
   */
  boolean getUpdateName();

  /**
   * <pre>
   * Whether to update the description of the addon.
   * </pre>
   *
   * <code>bool update_description = 4;</code>
   * @return The updateDescription.
   */
  boolean getUpdateDescription();

  /**
   * <pre>
   * Whether to update the resource type of the addon.
   * </pre>
   *
   * <code>bool update_resource_type = 5;</code>
   * @return The updateResourceType.
   */
  boolean getUpdateResourceType();

  /**
   * <pre>
   * Whether to update the default amount of the addon.
   * </pre>
   *
   * <code>bool update_default_amount = 6;</code>
   * @return The updateDefaultAmount.
   */
  boolean getUpdateDefaultAmount();

  /**
   * <pre>
   * Whether to update the default paid field of the addon.
   * </pre>
   *
   * <code>bool update_default_paid = 7;</code>
   * @return The updateDefaultPaid.
   */
  boolean getUpdateDefaultPaid();
}
