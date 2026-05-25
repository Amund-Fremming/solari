import type { CSSProperties } from "react";
import type { ApplePayButtonProps } from "./ApplePayButton";
import {
  APPLE_PAY_BUTTON_BORDER_RADIUS,
  APPLE_PAY_BUTTON_HEIGHT,
} from "./ApplePayButton";

type ApplePayButtonCSSProperties = CSSProperties & Record<`--${string}`, string>;

const APPLE_PAY_WEB_BUTTON_STYLE: ApplePayButtonCSSProperties = {
  display: "block",
  width: "100%",
  height: `${APPLE_PAY_BUTTON_HEIGHT}px`,
  borderRadius: `${APPLE_PAY_BUTTON_BORDER_RADIUS}px`,
  border: "none",
  cursor: "pointer",
  WebkitAppearance: "-apple-pay-button",
  "--apple-pay-button-style": "black",
  "--apple-pay-button-type": "buy",
};

const APPLE_PAY_FALLBACK_STYLE: CSSProperties = {
  display: "block",
  width: "100%",
  height: `${APPLE_PAY_BUTTON_HEIGHT}px`,
  borderRadius: `${APPLE_PAY_BUTTON_BORDER_RADIUS}px`,
  border: "none",
  backgroundColor: "#000000",
  color: "#ffffff",
  fontSize: "18px",
  fontWeight: 600,
  letterSpacing: "0.3px",
  cursor: "pointer",
};

export function ApplePayButton(props: ApplePayButtonProps) {
  const handleClick = async () => {
    try {
      await props.onClick();
    } catch (error) {
      console.error("Apple Pay button click handler error:", error);
    }
  };

  const supportsApplePayWebButton =
    typeof CSS !== "undefined" &&
    typeof CSS.supports === "function" &&
    CSS.supports("-webkit-appearance", "-apple-pay-button");

  if (supportsApplePayWebButton) {
    return (
      <button
        type="button"
        onClick={handleClick}
        aria-label="Apple Pay"
        style={APPLE_PAY_WEB_BUTTON_STYLE}
      />
    );
  }

  return (
    <button
      type="button"
      onClick={handleClick}
      style={APPLE_PAY_FALLBACK_STYLE}
    >
      Apple Pay
    </button>
  );
}
