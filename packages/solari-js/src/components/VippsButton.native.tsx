/**
 * Native button component for Vipps (React Native/Expo)
 * Since the Vipps checkout button library is web-based,
 * this component provides a native button that integrates with the payment service
 */

import { createElement, Fragment } from "react";
import {
  Text,
  StyleSheet,
  View,
  ViewStyle,
  TextStyle,
  Pressable,
} from "react-native";
import type { VippsButtonProps } from "./VippsButton";
import { VIPPS_COLORS } from "../services/vippsPaymentService";

const styles = StyleSheet.create({
  button: {
    borderRadius: 4,
    paddingVertical: 12,
    paddingHorizontal: 16,
    justifyContent: "center",
    alignItems: "center",
    flexDirection: "row",
    gap: 8,
  } as ViewStyle,
  buttonRounded: {
    borderRadius: 24,
  } as ViewStyle,
  buttonStretched: {
    alignSelf: "stretch",
  } as ViewStyle,
  buttonPrimary: {
    backgroundColor: VIPPS_COLORS.primary,
  } as ViewStyle,
  buttonDark: {
    backgroundColor: VIPPS_COLORS.dark,
  } as ViewStyle,
  buttonLight: {
    backgroundColor: VIPPS_COLORS.light,
    borderWidth: 1,
    borderColor: "#ccc",
  } as ViewStyle,
  text: {
    fontSize: 16,
    fontWeight: "600",
  } as TextStyle,
  textPrimary: {
    color: "white",
  } as TextStyle,
  textDark: {
    color: "white",
  } as TextStyle,
  textLight: {
    color: VIPPS_COLORS.dark,
  } as TextStyle,
  disabled: {
    opacity: 0.5,
  } as ViewStyle,
  logo: {
    width: 20,
    height: 20,
    marginRight: 4,
  } as ViewStyle,
});

/**
 * Get button text based on verb
 */
function getButtonText(verb: string): string {
  const textMap: Record<string, string> = {
    buy: "Buy with Vipps",
    pay: "Pay with Vipps",
    login: "Login with Vipps",
    register: "Register with Vipps",
    continue: "Continue with Vipps",
    confirm: "Confirm with Vipps",
    donate: "Donate with Vipps",
  };
  return textMap[verb] || "Vipps";
}

/**
 * VippsButton React Native Component (Expo)
 *
 * @example
 * ```tsx
 * import { VippsButton } from '@solari/solari-js/native';
 *
 * export function CheckoutScreen() {
 *   return (
 *     <VippsButton
 *       verb="pay"
 *       language="no"
 *       onClick={async () => {
 *         // Initiate payment flow
 *       }}
 *     />
 *   );
 * }
 * ```
 */
export function VippsButton(props: VippsButtonProps) {
  const buttonStyle: ViewStyle[] = [styles.button];
  const textStyle: TextStyle[] = [styles.text];

  const variantStyle = styles.buttonPrimary;
  const textVariantStyle = styles.textPrimary;

  if (variantStyle && typeof variantStyle === "object") {
    buttonStyle.push(variantStyle as ViewStyle);
  }
  if (textVariantStyle && typeof textVariantStyle === "object") {
    textStyle.push(textVariantStyle as TextStyle);
  }

  buttonStyle.push(styles.buttonRounded);
  buttonStyle.push(styles.buttonStretched);

  const handlePress = async () => {
    try {
      await props.onClick();
    } catch (error) {
      console.error("Vipps button press error:", error);
    }
  };

  return createElement(
    Pressable as any,
    {
      style: buttonStyle,
      onPress: handlePress,
      testID: "vipps-button",
    },
    createElement(
      Fragment,
      null,
      createElement(Text as any, { style: textStyle }, "✓"),
      createElement(Text as any, { style: textStyle }, getButtonText("pay")),
    ),
  );
}
