/**
 * Native button component for Vipps (React Native/Expo)
 * Since the Vipps checkout button library is web-based,
 * this component provides a native button that integrates with the payment service
 */

import { createElement, Fragment } from "react";
import {
  Text,
  StyleSheet,
  ViewStyle,
  TextStyle,
  Pressable,
} from "react-native";
import { SvgXml } from "react-native-svg";
import type { VippsButtonProps } from "./VippsButton";
import { VIPPS_COLORS } from "../services/vippsPaymentService";

const VIPPS_ICON_SVG = `<svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg"><circle cx="10" cy="10" r="10" fill="white"/><path d="M6.52 5.84H8.99C10.45 5.84 11.26 6.58 11.26 7.73C11.26 8.26 10.94 8.9 10.29 9.27C11.2 9.57 11.56 10.2 11.56 10.81C11.56 11.98 10.62 12.9 9.13 12.9H6.52V5.84ZM7.93 9.93V11.65H8.99C9.63 11.65 10.08 11.39 10.08 10.79C10.08 10.29 9.65 9.93 8.99 9.93H7.93ZM7.93 7.09V8.73H8.88C9.45 8.73 9.83 8.42 9.83 7.91C9.83 7.39 9.49 7.09 8.88 7.09H7.93Z" fill="#FF5B24"/><path d="M13.73 6.76C14.22 6.76 14.63 7.13 14.63 7.67C14.63 8.2 14.22 8.57 13.73 8.57C13.24 8.57 12.83 8.2 12.83 7.67C12.83 7.13 13.24 6.76 13.73 6.76ZM12.99 9.2H14.45V12.9H12.99V9.2Z" fill="#FF5B24"/></svg>`;

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
    marginRight: 6,
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
      createElement(SvgXml as any, {
        xml: VIPPS_ICON_SVG,
        width: 20,
        height: 20,
        style: styles.logo,
      }),
      createElement(Text as any, { style: textStyle }, getButtonText("pay")),
    ),
  );
}
