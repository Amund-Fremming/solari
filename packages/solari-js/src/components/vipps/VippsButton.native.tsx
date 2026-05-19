import { createElement } from "react";
import { Image, Pressable, StyleSheet, View, ViewStyle } from "react-native";
import { SvgUri } from "react-native-svg";
import type { VippsButtonProps } from "./VippsButton";
import { VIPPS_COLORS } from "../../services/solariPaymentService";

const VIPPS_BUTTON_SVG_SOURCE = Image.resolveAssetSource(
  require("./pay-with-vipps.svg"),
);

const styles = StyleSheet.create({
  button: {
    borderRadius: 24,
    overflow: "hidden",
    justifyContent: "center",
    alignItems: "center",
    paddingVertical: 0,
    paddingHorizontal: 0,
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
  disabled: {
    opacity: 0.5,
  } as ViewStyle,
  buttonSvg: {
    width: "100%",
    height: 44,
  } as ViewStyle,
  buttonContent: {
    width: "100%",
    opacity: 1,
  } as ViewStyle,
  buttonPressed: {
    opacity: 0.88,
  } as ViewStyle,
});

export function VippsButton(props: VippsButtonProps) {
  const buttonStyle: ViewStyle[] = [styles.button];

  const variantStyle = styles.buttonPrimary;

  if (variantStyle && typeof variantStyle === "object") {
    buttonStyle.push(variantStyle as ViewStyle);
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
      style: ({ pressed }: { pressed: boolean }) => [
        ...buttonStyle,
        pressed ? styles.buttonPressed : null,
      ],
      onPress: handlePress,
      testID: "vipps-button",
    },
    createElement(
      View as any,
      {
        style: styles.buttonContent,
      },
      createElement(SvgUri as any, {
        uri: VIPPS_BUTTON_SVG_SOURCE.uri,
        style: styles.buttonSvg,
      }),
    ),
  );
}
