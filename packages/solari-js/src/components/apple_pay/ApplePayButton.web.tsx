import { useState } from "react";
import type { ApplePayButtonProps } from "./ApplePayButton";
import {
  APPLE_PAY_BUTTON_BORDER_RADIUS,
  APPLE_PAY_BUTTON_HEIGHT,
} from "./ApplePayButton";

export function ApplePayButton(props: ApplePayButtonProps) {
  const [isPressed, setIsPressed] = useState(false);

  const handleClick = async () => {
    try {
      await props.onClick();
    } catch (error) {
      console.error("Apple Pay button click handler error:", error);
    }
  };

  return (
    <button
      type="button"
      onClick={handleClick}
      onPointerDown={() => setIsPressed(true)}
      onPointerUp={() => setIsPressed(false)}
      onPointerCancel={() => setIsPressed(false)}
      onPointerLeave={() => setIsPressed(false)}
      onKeyDown={() => setIsPressed(true)}
      onKeyUp={() => setIsPressed(false)}
      style={{
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
        transition: "transform 120ms ease, filter 120ms ease",
        transform: isPressed ? "scale(0.985)" : "scale(1)",
        filter: isPressed ? "brightness(0.96)" : "none",
      }}
    >
      Apple Pay
    </button>
  );
}
