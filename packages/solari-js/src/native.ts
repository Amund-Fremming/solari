export type { VippsButtonProps } from "./components/VippsButton";

export { VippsButton as VippsButtonNative } from "./components/VippsButton.native";

export {
  createNativeClient,
  create_native_client,
  type CreateNativeClientOptions,
  type VippsPaymentClient,
  type PaymentSnapshot,
  type VippsPaymentFlowResult,
  VIPPS_COLORS,
} from "./services/vippsPaymentService";
