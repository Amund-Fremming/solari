export interface VippsButtonProps {
  onClick: () => void | Promise<void>;
}

export const VIPPS_BUTTON_SCRIPT_URL =
  "https://checkout.vipps.no/checkout-button/v1/vipps-checkout-button.js";

export const VIPPS_BUTTON_SVG_PATH = "./pay-with-vipps.svg";

export const DEFAULT_VIPPS_BUTTON_PROPS = {
  brand: "vipps",
  variant: "primary",
  language: "no",
  rounded: true,
  verb: "pay",
  stretched: true,
  branded: true,
};

export function propsToAttributes(): Record<string, string> {
  return {
    type: "button",
    brand: DEFAULT_VIPPS_BUTTON_PROPS.brand,
    variant: DEFAULT_VIPPS_BUTTON_PROPS.variant,
    language: DEFAULT_VIPPS_BUTTON_PROPS.language,
    rounded: String(DEFAULT_VIPPS_BUTTON_PROPS.rounded),
    verb: DEFAULT_VIPPS_BUTTON_PROPS.verb,
    stretched: String(DEFAULT_VIPPS_BUTTON_PROPS.stretched),
    branded: String(DEFAULT_VIPPS_BUTTON_PROPS.branded),
  };
}

export function loadVippsButtonScript(): Promise<void> {
  return new Promise((resolve, reject) => {
    // Check if script is already loaded
    if ((window as any).VippsCheckoutButton) {
      resolve();
      return;
    }

    // Check if script tag already exists
    const existingScript = document.querySelector(
      `script[src="${VIPPS_BUTTON_SCRIPT_URL}"]`,
    );
    if (existingScript) {
      existingScript.addEventListener("load", () => resolve());
      existingScript.addEventListener("error", () =>
        reject(new Error("Failed to load Vipps button script")),
      );
      return;
    }

    // Create and load the script
    const script = document.createElement("script");
    script.src = VIPPS_BUTTON_SCRIPT_URL;
    script.async = true;
    script.type = "text/javascript";

    script.addEventListener("load", () => resolve());
    script.addEventListener("error", () =>
      reject(new Error("Failed to load Vipps button script")),
    );

    document.head.appendChild(script);
  });
}

export function getVippsButtonSvgUrl(): string {
  return new URL(VIPPS_BUTTON_SVG_PATH, import.meta.url).toString();
}
