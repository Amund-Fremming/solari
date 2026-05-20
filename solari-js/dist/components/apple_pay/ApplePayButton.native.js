import { createElement } from "react";
import { Pressable, StyleSheet, Text } from "react-native";
import { APPLE_PAY_BUTTON_BORDER_RADIUS, APPLE_PAY_BUTTON_HEIGHT, } from "./ApplePayButton";
const styles = StyleSheet.create({
    button: {
        height: APPLE_PAY_BUTTON_HEIGHT,
        borderRadius: APPLE_PAY_BUTTON_BORDER_RADIUS,
        alignSelf: "stretch",
        backgroundColor: "#000000",
        justifyContent: "center",
        alignItems: "center",
        paddingHorizontal: 16,
    },
    buttonPressed: {
        opacity: 0.88,
    },
    text: {
        color: "#ffffff",
        fontSize: 18,
        fontWeight: "600",
        letterSpacing: 0.3,
    },
});
export function ApplePayButton(props) {
    const handlePress = async () => {
        try {
            await props.onClick();
        }
        catch (error) {
            console.error("Apple Pay button press error:", error);
        }
    };
    return createElement(Pressable, {
        style: ({ pressed }) => [
            styles.button,
            pressed ? styles.buttonPressed : null,
        ],
        onPress: handlePress,
        testID: "apple-pay-button",
    }, createElement(Text, { style: styles.text }, "Apple Pay"));
}
//# sourceMappingURL=ApplePayButton.native.js.map