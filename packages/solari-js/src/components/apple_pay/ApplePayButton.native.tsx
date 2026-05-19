import { createElement } from "react";
import { Pressable, StyleSheet, Text, ViewStyle } from "react-native";
import type { ApplePayButtonProps } from "./ApplePayButton";
import {
  APPLE_PAY_BUTTON_BORDER_RADIUS,
  APPLE_PAY_BUTTON_HEIGHT,
} from "./ApplePayButton";

const styles = StyleSheet.create({
  button: {
    height: APPLE_PAY_BUTTON_HEIGHT,
    borderRadius: APPLE_PAY_BUTTON_BORDER_RADIUS,
    alignSelf: "stretch",
    backgroundColor: "#000000",
    justifyContent: "center",
    alignItems: "center",
    paddingHorizontal: 16,
  } as ViewStyle,
  buttonPressed: {
    opacity: 0.88,
  } as ViewStyle,
  text: {
    color: "#ffffff",
    fontSize: 18,
    fontWeight: "600",
    letterSpacing: 0.3,
  },
});

export function ApplePayButton(props: ApplePayButtonProps) {
  const handlePress = async () => {
    try {
      await props.onClick();
    } catch (error) {
      console.error("Apple Pay button press error:", error);
    }
  };

  return createElement(
    Pressable as any,
    {
      style: ({ pressed }: { pressed: boolean }) => [
        styles.button,
        pressed ? styles.buttonPressed : null,
      ],
      onPress: handlePress,
      testID: "apple-pay-button",
    },
    createElement(Text as any, { style: styles.text }, "Apple Pay"),
  );
}
