export interface VippsButtonProps {
    onClick: () => void | Promise<void>;
}
export declare const VIPPS_BUTTON_SCRIPT_URL = "https://checkout.vipps.no/checkout-button/v1/vipps-checkout-button.js";
export declare const VIPPS_BUTTON_SVG_PATH = "./pay-with-vipps.svg";
export declare const DEFAULT_VIPPS_BUTTON_PROPS: {
    brand: string;
    variant: string;
    language: string;
    rounded: boolean;
    verb: string;
    stretched: boolean;
    branded: boolean;
};
export declare function propsToAttributes(): Record<string, string>;
export declare function loadVippsButtonScript(): Promise<void>;
export declare function getVippsButtonSvgUrl(): string;
//# sourceMappingURL=VippsButton.d.ts.map