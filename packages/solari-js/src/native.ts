export type { VippsButtonProps } from "./components/VippsButton";

export { VippsButton as VippsButtonNative } from "./components/VippsButton.native";

export {
  createNativeClient,
  createNativeClientNoRedirect,
  create_native_client,
  create_native_client_no_redirect,
  type CreateNativeClientOptions,
  type VippsPaymentClient,
  type PaymentSnapshot,
  type VippsPaymentFlowResult,
  VIPPS_COLORS,
} from "./services/vippsPaymentService";
