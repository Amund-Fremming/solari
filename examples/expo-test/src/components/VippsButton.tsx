import React from "react";
import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  Text,
  ViewStyle,
} from "react-native";
import { VIPPS_COLORS } from "@solari/solari-js";

interface VippsButtonProps {
  onPress: () => void | Promise<void>;
  isLoading?: boolean;
  disabled?: boolean;
  style?: ViewStyle;
  label?: string;
}

/**
 * Official Vipps Button Component
 * Uses official Vipps MobilePay branding and colors
 * Handles payment flow with loading states
 */
export function VippsButton({
  onPress,
  isLoading = false,
  disabled = false,
  style,
  label = "Pay with Vipps",
}: VippsButtonProps) {
  return (
    <Pressable
      accessibilityRole="button"
      disabled={isLoading || disabled}
      onPress={onPress}
      style={({ pressed }) => [
        styles.button,
        pressed && !isLoading && !disabled && styles.buttonPressed,
        (isLoading || disabled) && styles.buttonDisabled,
        style,
      ]}
    >
      {isLoading ? (
        <ActivityIndicator color={VIPPS_COLORS.dark} size="small" />
      ) : (
        <Text style={styles.buttonLabel}>{label}</Text>
      )}
    </Pressable>
  );
}

const styles = StyleSheet.create({
  button: {
    alignItems: "center",
    backgroundColor: VIPPS_COLORS.primary,
    borderRadius: 999,
    minHeight: 58,
    justifyContent: "center",
    paddingHorizontal: 18,
  },
  buttonPressed: {
    opacity: 0.92,
    transform: [{ scale: 0.99 }],
  },
  buttonDisabled: {
    opacity: 0.65,
  },
  buttonLabel: {
    color: VIPPS_COLORS.dark,
    fontSize: 18,
    fontWeight: "800",
  },
});
