// Export types and utilities
export type { VippsButtonProps } from "./VippsButton";

export {
  VIPPS_BUTTON_SCRIPT_URL,
  VIPPS_BUTTON_SVG_PATH,
  DEFAULT_VIPPS_BUTTON_PROPS,
  propsToAttributes,
  loadVippsButtonScript,
  getVippsButtonSvgUrl,
} from "./VippsButton";

// Web component
export { VippsButton as VippsButtonWeb } from "./VippsButton.web";
