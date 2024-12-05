// Generated by the protocol buffer compiler.  DO NOT EDIT!
// NO CHECKED-IN PROTOBUF GENCODE
// source: qms_subscriptions.proto
// Protobuf Java Version: 4.29.0

package org.cyverse.de.protobufs;

public interface SubscriptionOrBuilder extends
    // @@protoc_insertion_point(interface_extends:qms.Subscription)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The unique identifier
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The uuid.
   */
  java.lang.String getUuid();
  /**
   * <pre>
   * The unique identifier
   * </pre>
   *
   * <code>string uuid = 1;</code>
   * @return The bytes for uuid.
   */
  com.google.protobuf.ByteString
      getUuidBytes();

  /**
   * <pre>
   * The date the subscription activates.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_start_date = 2 [json_name = "effective_start_date"];</code>
   * @return Whether the effectiveStartDate field is set.
   */
  boolean hasEffectiveStartDate();
  /**
   * <pre>
   * The date the subscription activates.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_start_date = 2 [json_name = "effective_start_date"];</code>
   * @return The effectiveStartDate.
   */
  com.google.protobuf.Timestamp getEffectiveStartDate();
  /**
   * <pre>
   * The date the subscription activates.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_start_date = 2 [json_name = "effective_start_date"];</code>
   */
  com.google.protobuf.TimestampOrBuilder getEffectiveStartDateOrBuilder();

  /**
   * <pre>
   * The date the subscription deactivates/expires.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_end_date = 3 [json_name = "effective_end_date"];</code>
   * @return Whether the effectiveEndDate field is set.
   */
  boolean hasEffectiveEndDate();
  /**
   * <pre>
   * The date the subscription deactivates/expires.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_end_date = 3 [json_name = "effective_end_date"];</code>
   * @return The effectiveEndDate.
   */
  com.google.protobuf.Timestamp getEffectiveEndDate();
  /**
   * <pre>
   * The date the subscription deactivates/expires.
   * </pre>
   *
   * <code>.google.protobuf.Timestamp effective_end_date = 3 [json_name = "effective_end_date"];</code>
   */
  com.google.protobuf.TimestampOrBuilder getEffectiveEndDateOrBuilder();

  /**
   * <pre>
   * The user in the QMS system that the subscription is for.
   * </pre>
   *
   * <code>.qms.QMSUser user = 4;</code>
   * @return Whether the user field is set.
   */
  boolean hasUser();
  /**
   * <pre>
   * The user in the QMS system that the subscription is for.
   * </pre>
   *
   * <code>.qms.QMSUser user = 4;</code>
   * @return The user.
   */
  org.cyverse.de.protobufs.QMSUser getUser();
  /**
   * <pre>
   * The user in the QMS system that the subscription is for.
   * </pre>
   *
   * <code>.qms.QMSUser user = 4;</code>
   */
  org.cyverse.de.protobufs.QMSUserOrBuilder getUserOrBuilder();

  /**
   * <pre>
   * The plan the user is subscribed to.
   * </pre>
   *
   * <code>.qms.Plan plan = 5;</code>
   * @return Whether the plan field is set.
   */
  boolean hasPlan();
  /**
   * <pre>
   * The plan the user is subscribed to.
   * </pre>
   *
   * <code>.qms.Plan plan = 5;</code>
   * @return The plan.
   */
  org.cyverse.de.protobufs.Plan getPlan();
  /**
   * <pre>
   * The plan the user is subscribed to.
   * </pre>
   *
   * <code>.qms.Plan plan = 5;</code>
   */
  org.cyverse.de.protobufs.PlanOrBuilder getPlanOrBuilder();

  /**
   * <pre>
   * The list of quotas applied to the subscription. Initially populated
   * by quota defaults, but can be overridden.
   * </pre>
   *
   * <code>repeated .qms.Quota quotas = 6;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Quota> 
      getQuotasList();
  /**
   * <pre>
   * The list of quotas applied to the subscription. Initially populated
   * by quota defaults, but can be overridden.
   * </pre>
   *
   * <code>repeated .qms.Quota quotas = 6;</code>
   */
  org.cyverse.de.protobufs.Quota getQuotas(int index);
  /**
   * <pre>
   * The list of quotas applied to the subscription. Initially populated
   * by quota defaults, but can be overridden.
   * </pre>
   *
   * <code>repeated .qms.Quota quotas = 6;</code>
   */
  int getQuotasCount();
  /**
   * <pre>
   * The list of quotas applied to the subscription. Initially populated
   * by quota defaults, but can be overridden.
   * </pre>
   *
   * <code>repeated .qms.Quota quotas = 6;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.QuotaOrBuilder> 
      getQuotasOrBuilderList();
  /**
   * <pre>
   * The list of quotas applied to the subscription. Initially populated
   * by quota defaults, but can be overridden.
   * </pre>
   *
   * <code>repeated .qms.Quota quotas = 6;</code>
   */
  org.cyverse.de.protobufs.QuotaOrBuilder getQuotasOrBuilder(
      int index);

  /**
   * <pre>
   * The list of resource usages that the user has generated while this plan was active.
   * </pre>
   *
   * <code>repeated .qms.Usage usages = 7;</code>
   */
  java.util.List<org.cyverse.de.protobufs.Usage> 
      getUsagesList();
  /**
   * <pre>
   * The list of resource usages that the user has generated while this plan was active.
   * </pre>
   *
   * <code>repeated .qms.Usage usages = 7;</code>
   */
  org.cyverse.de.protobufs.Usage getUsages(int index);
  /**
   * <pre>
   * The list of resource usages that the user has generated while this plan was active.
   * </pre>
   *
   * <code>repeated .qms.Usage usages = 7;</code>
   */
  int getUsagesCount();
  /**
   * <pre>
   * The list of resource usages that the user has generated while this plan was active.
   * </pre>
   *
   * <code>repeated .qms.Usage usages = 7;</code>
   */
  java.util.List<? extends org.cyverse.de.protobufs.UsageOrBuilder> 
      getUsagesOrBuilderList();
  /**
   * <pre>
   * The list of resource usages that the user has generated while this plan was active.
   * </pre>
   *
   * <code>repeated .qms.Usage usages = 7;</code>
   */
  org.cyverse.de.protobufs.UsageOrBuilder getUsagesOrBuilder(
      int index);

  /**
   * <pre>
   * A flag indicating whether or not the user paid for the subscription.
   * </pre>
   *
   * <code>bool paid = 8;</code>
   * @return The paid.
   */
  boolean getPaid();

  /**
   * <pre>
   * Information about the rate that was active when the subscription was purchased. Note that this rate is recorded
   * whether or not the user paid for the subscription directly.
   * </pre>
   *
   * <code>.qms.PlanRate plan_rate = 9 [json_name = "plan_rate"];</code>
   * @return Whether the planRate field is set.
   */
  boolean hasPlanRate();
  /**
   * <pre>
   * Information about the rate that was active when the subscription was purchased. Note that this rate is recorded
   * whether or not the user paid for the subscription directly.
   * </pre>
   *
   * <code>.qms.PlanRate plan_rate = 9 [json_name = "plan_rate"];</code>
   * @return The planRate.
   */
  org.cyverse.de.protobufs.PlanRate getPlanRate();
  /**
   * <pre>
   * Information about the rate that was active when the subscription was purchased. Note that this rate is recorded
   * whether or not the user paid for the subscription directly.
   * </pre>
   *
   * <code>.qms.PlanRate plan_rate = 9 [json_name = "plan_rate"];</code>
   */
  org.cyverse.de.protobufs.PlanRateOrBuilder getPlanRateOrBuilder();
}
