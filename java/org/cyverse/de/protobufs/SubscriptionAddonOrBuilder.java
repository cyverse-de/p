// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: qms_addons.proto

package org.cyverse.de.protobufs;

public interface SubscriptionAddonOrBuilder extends
    // @@protoc_insertion_point(interface_extends:SubscriptionAddon)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The unique identifier for the add-on
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <pre>
   * The unique identifier for the add-on
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <pre>
   * The add-on used with the subscription. May only contain the add-on's 
   * UUID in some circumstances.
   * </pre>
   *
   * <code>.Addon addon = 2;</code>
   * @return Whether the addon field is set.
   */
  boolean hasAddon();
  /**
   * <pre>
   * The add-on used with the subscription. May only contain the add-on's 
   * UUID in some circumstances.
   * </pre>
   *
   * <code>.Addon addon = 2;</code>
   * @return The addon.
   */
  org.cyverse.de.protobufs.Addon getAddon();
  /**
   * <pre>
   * The add-on used with the subscription. May only contain the add-on's 
   * UUID in some circumstances.
   * </pre>
   *
   * <code>.Addon addon = 2;</code>
   */
  org.cyverse.de.protobufs.AddonOrBuilder getAddonOrBuilder();

  /**
   * <pre>
   * The subscription the add-on was applied to. May only contain the add-on's
   * UUID in some circumstances.
   * </pre>
   *
   * <code>.Subscription subscription = 3;</code>
   * @return Whether the subscription field is set.
   */
  boolean hasSubscription();
  /**
   * <pre>
   * The subscription the add-on was applied to. May only contain the add-on's
   * UUID in some circumstances.
   * </pre>
   *
   * <code>.Subscription subscription = 3;</code>
   * @return The subscription.
   */
  org.cyverse.de.protobufs.Subscription getSubscription();
  /**
   * <pre>
   * The subscription the add-on was applied to. May only contain the add-on's
   * UUID in some circumstances.
   * </pre>
   *
   * <code>.Subscription subscription = 3;</code>
   */
  org.cyverse.de.protobufs.SubscriptionOrBuilder getSubscriptionOrBuilder();

  /**
   * <pre>
   * The amount of the resource applied by the add-on. This should default to
   * the amount contained in the add-on definition, but can be overridden, 
   * which is why it's a separate field here.
   * </pre>
   *
   * <code>float amount = 4;</code>
   * @return The amount.
   */
  float getAmount();

  /**
   * <pre>
   * Whether the subscription add-on costs money. This should default to the 
   * same paid value contained in the add-on definition, but can be overridden,
   * which is whay it's a separate field here.
   * </pre>
   *
   * <code>bool paid = 5;</code>
   * @return The paid.
   */
  boolean getPaid();
}
