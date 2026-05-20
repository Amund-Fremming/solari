import { createElement } from "react";
import { Image, Pressable, StyleSheet, View } from "react-native";
import { SvgUri } from "react-native-svg";
import { VIPPS_COLORS } from "../../services/solariPaymentService";
const VIPPS_BUTTON_SVG_SOURCE = Image.resolveAssetSource(require("./pay-with-vipps.svg"));
const styles = StyleSheet.create({
    button: {
        borderRadius: 24,
        overflow: "hidden",
        justifyContent: "center",
        alignItems: "center",
        paddingVertical: 0,
        paddingHorizontal: 0,
    },
    buttonRounded: {
        borderRadius: 24,
    },
    buttonStretched: {
        alignSelf: "stretch",
    },
    buttonPrimary: {
        backgroundColor: VIPPS_COLORS.primary,
    },
    buttonDark: {
        backgroundColor: VIPPS_COLORS.dark,
    },
    buttonLight: {
        backgroundColor: VIPPS_COLORS.light,
        borderWidth: 1,
        borderColor: "#ccc",
    },
    disabled: {
        opacity: 0.5,
    },
    buttonSvg: {
        width: "100%",
        height: 44,
    },
    buttonContent: {
        width: "100%",
        opacity: 1,
    },
    buttonPressed: {
        opacity: 0.88,
    },
});
export function VippsButton(props) {
    const buttonStyle = [styles.button];
    const variantStyle = styles.buttonPrimary;
    if (variantStyle && typeof variantStyle === "object") {
        buttonStyle.push(variantStyle);
    }
    buttonStyle.push(styles.buttonRounded);
    buttonStyle.push(styles.buttonStretched);
    const handlePress = async () => {
        try {
            await props.onClick();
        }
        catch (error) {
            console.error("Vipps button press error:", error);
        }
    };
    return createElement(Pressable, {
        style: ({ pressed }) => [
            ...buttonStyle,
            pressed ? styles.buttonPressed : null,
        ],
        onPress: handlePress,
        testID: "vipps-button",
    }, createElement(View, {
        style: styles.buttonContent,
    }, createElement(SvgUri, {
        uri: VIPPS_BUTTON_SVG_SOURCE.uri,
        style: styles.buttonSvg,
    })));
}
//# sourceMappingURL=VippsButton.native.js.map