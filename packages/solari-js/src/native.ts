export type { VippsButtonProps } from "./components/VippsButton";
export * from "./types";

export { VippsButton as VippsButtonNative } from "./components/VippsButton.native";

export {
  createNativeClient,
  createNativeClientNoRedirect,
  create_native_client,
  create_native_client_no_redirect,
  VIPPS_COLORS,
} from "./services/solariPaymentService";
