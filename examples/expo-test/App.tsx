import { StatusBar } from "expo-status-bar";
import { useEffect, useState } from "react";
import {
  ActivityIndicator,
  Pressable,
  SafeAreaView,
  ScrollView,
  StyleSheet,
  Text,
  View,
} from "react-native";

import {
  type PaymentSnapshot,
  startVippsPayment,
  vippsPaymentService,
} from "./src/services/vippsPaymentService";

const FALLBACK_STATUS: PaymentSnapshot = {
  provider: "vipps",
  status: "idle",
  requested_amount: 0,
  paid_amount: 0,
  reference: null,
  redirect_url: null,
  return_url: null,
  attempts: 0,
  updated_from: "startup",
  last_error: null,
  last_webhook_payload: null,
};

export default function App() {
  const [payment, setPayment] = useState<PaymentSnapshot>(FALLBACK_STATUS);
  const [feedback, setFeedback] = useState(
    "Ready to start the Vipps test flow.",
  );
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    let cancelled = false;

    async function bootstrap() {
      try {
        const response = await vippsPaymentService.getPaymentStatus();

        if (!cancelled) {
          setPayment(response.payment);
        }
      } catch (error) {
        if (!cancelled) {
          const message =
            error instanceof Error
              ? error.message
              : "Unable to reach the backend.";
          setFeedback(message);
        }
      }
    }

    void bootstrap();

    return () => {
      cancelled = true;
    };
  }, []);

  async function handleRefresh() {
    setBusy(true);
    setFeedback("Refreshing payment state from the backend...");

    try {
      const response = await vippsPaymentService.getPaymentStatus();
      setPayment(response.payment);
      setFeedback(`Latest status: ${response.payment.status}.`);
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Refresh failed.";
      setFeedback(message);
    } finally {
      setBusy(false);
    }
  }

  async function handleVippsPress() {
    setBusy(true);
    setFeedback("Creating the Vipps payment and opening the approval flow...");

    try {
      const result = await startVippsPayment();
      setPayment(result.payment);

      if (result.payment.status === "completed") {
        setFeedback(`Vipps payment completed via ${result.apiBaseUrl}.`);
      } else if (result.payment.status === "pending") {
        setFeedback(
          `Vipps returned ${result.browserResult}; backend status is still pending. Check your webhook tunnel if it does not settle.`,
        );
      } else {
        setFeedback(
          `Vipps payment ended with status ${result.payment.status}.`,
        );
      }
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Vipps payment failed.";
      setFeedback(message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <SafeAreaView style={styles.safeArea}>
      <StatusBar style="dark" />
      <ScrollView contentContainerStyle={styles.content}>
        <View style={styles.heroCard}>
          <Text style={styles.eyebrow}>Solari x Vipps</Text>
          <Text style={styles.title}>Pay with Vipps</Text>
          <Text style={styles.subtitle}>
            This screen drives your axum test backend and opens the Vipps
            approval flow.
          </Text>

          <Pressable
            accessibilityRole="button"
            disabled={busy}
            onPress={handleVippsPress}
            style={({ pressed }) => [
              styles.vippsButton,
              pressed && !busy ? styles.vippsButtonPressed : null,
              busy ? styles.vippsButtonDisabled : null,
            ]}
          >
            {busy ? (
              <ActivityIndicator color="#2f1c00" />
            ) : (
              <Text style={styles.vippsButtonLabel}>Pay with Vipps</Text>
            )}
          </Pressable>

          <Pressable
            accessibilityRole="button"
            disabled={busy}
            onPress={handleRefresh}
            style={({ pressed }) => [
              styles.secondaryButton,
              pressed && !busy ? styles.secondaryButtonPressed : null,
            ]}
          >
            <Text style={styles.secondaryButtonLabel}>Refresh status</Text>
          </Pressable>
        </View>

        <View style={styles.panel}>
          <Text style={styles.panelLabel}>Backend</Text>
          <Text style={styles.panelValue}>
            {vippsPaymentService.resolveApiBaseUrl()}
          </Text>
          <Text style={styles.panelHint}>
            Set EXPO_PUBLIC_AXUM_BASE_URL to your ngrok HTTPS URL when testing
            on a real device.
          </Text>
        </View>

        <View style={styles.panel}>
          <Text style={styles.panelLabel}>Flow status</Text>
          <Text style={styles.feedback}>{feedback}</Text>
        </View>

        <View style={styles.panel}>
          <Text style={styles.panelLabel}>Payment snapshot</Text>
          <View style={styles.row}>
            <Text style={styles.rowLabel}>Status</Text>
            <Text style={styles.rowValue}>{payment.status}</Text>
          </View>
          <View style={styles.row}>
            <Text style={styles.rowLabel}>Requested</Text>
            <Text style={styles.rowValue}>{payment.requested_amount} NOK</Text>
          </View>
          <View style={styles.row}>
            <Text style={styles.rowLabel}>Paid</Text>
            <Text style={styles.rowValue}>{payment.paid_amount} NOK</Text>
          </View>
          <View style={styles.row}>
            <Text style={styles.rowLabel}>Attempts</Text>
            <Text style={styles.rowValue}>{payment.attempts}</Text>
          </View>
          <View style={styles.row}>
            <Text style={styles.rowLabel}>Reference</Text>
            <Text style={styles.rowValue}>{payment.reference ?? "-"}</Text>
          </View>
          <View style={styles.row}>
            <Text style={styles.rowLabel}>Updated from</Text>
            <Text style={styles.rowValue}>{payment.updated_from}</Text>
          </View>
          <View style={styles.stackRow}>
            <Text style={styles.rowLabel}>Last error</Text>
            <Text style={styles.stackValue}>{payment.last_error ?? "-"}</Text>
          </View>
        </View>
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safeArea: {
    flex: 1,
    backgroundColor: "#f6efe4",
  },
  content: {
    paddingHorizontal: 20,
    paddingVertical: 24,
    gap: 18,
  },
  heroCard: {
    backgroundColor: "#23160d",
    borderRadius: 28,
    padding: 24,
    gap: 14,
    shadowColor: "#000000",
    shadowOpacity: 0.14,
    shadowRadius: 16,
    shadowOffset: {
      width: 0,
      height: 8,
    },
    elevation: 8,
  },
  eyebrow: {
    color: "#ffb454",
    fontSize: 13,
    fontWeight: "700",
    letterSpacing: 1.3,
    textTransform: "uppercase",
  },
  title: {
    color: "#fff8ef",
    fontSize: 34,
    fontWeight: "800",
    lineHeight: 38,
  },
  subtitle: {
    color: "#d8c7b4",
    fontSize: 16,
    lineHeight: 24,
  },
  vippsButton: {
    alignItems: "center",
    backgroundColor: "#ff5b24",
    borderRadius: 999,
    minHeight: 58,
    justifyContent: "center",
    paddingHorizontal: 18,
  },
  vippsButtonPressed: {
    opacity: 0.92,
    transform: [{ scale: 0.99 }],
  },
  vippsButtonDisabled: {
    opacity: 0.65,
  },
  vippsButtonLabel: {
    color: "#2f1c00",
    fontSize: 18,
    fontWeight: "800",
  },
  secondaryButton: {
    alignItems: "center",
    borderColor: "#6d5645",
    borderRadius: 999,
    borderWidth: 1,
    justifyContent: "center",
    minHeight: 48,
  },
  secondaryButtonPressed: {
    opacity: 0.84,
  },
  secondaryButtonLabel: {
    color: "#fff8ef",
    fontSize: 15,
    fontWeight: "700",
  },
  panel: {
    backgroundColor: "#fffaf2",
    borderColor: "#eadcca",
    borderRadius: 24,
    borderWidth: 1,
    gap: 12,
    padding: 18,
  },
  panelLabel: {
    color: "#7a6148",
    fontSize: 12,
    fontWeight: "700",
    letterSpacing: 1,
    textTransform: "uppercase",
  },
  panelValue: {
    color: "#23160d",
    fontSize: 16,
    fontWeight: "700",
  },
  panelHint: {
    color: "#6c5d4f",
    fontSize: 14,
    lineHeight: 20,
  },
  feedback: {
    color: "#2f2418",
    fontSize: 15,
    lineHeight: 22,
  },
  row: {
    alignItems: "center",
    flexDirection: "row",
    justifyContent: "space-between",
    gap: 12,
  },
  stackRow: {
    gap: 8,
  },
  rowLabel: {
    color: "#7b6249",
    fontSize: 14,
    fontWeight: "600",
  },
  rowValue: {
    color: "#23160d",
    flex: 1,
    fontSize: 14,
    fontWeight: "700",
    textAlign: "right",
  },
  stackValue: {
    color: "#23160d",
    fontSize: 14,
    lineHeight: 20,
  },
});
