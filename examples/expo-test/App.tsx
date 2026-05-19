import { StatusBar } from "expo-status-bar";
import { useEffect, useState } from "react";
import {
  Linking,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  View,
} from "react-native";
import { StripeProvider, useStripe } from "@stripe/stripe-react-native";

import {
  createNativeClientNoRedirect,
  type PaymentSnapshot,
  VippsButtonNative,
} from "@solari/solari-js/native";

const vippsClient = createNativeClientNoRedirect({
  openUrl: async (url: string) => {
    await Linking.openURL(url);
    return "opened";
  },
});

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

const STRIPE_PUBLISHABLE_KEY =
  process.env.EXPO_PUBLIC_STRIPE_PUBLISHABLE_KEY ?? "";

function AppScreen() {
  const { initPaymentSheet, presentPaymentSheet } = useStripe();

  const [payment, setPayment] = useState<PaymentSnapshot>(FALLBACK_STATUS);
  const [feedback, setFeedback] = useState(
    "Ready to start the Vipps test flow.",
  );
  const [authSessionResult, setAuthSessionResult] = useState<string | null>(
    null,
  );
  const [busy, setBusy] = useState(false);
  const [stripeFeedback, setStripeFeedback] = useState(
    "Stripe SDK ready. Create an intent and open PaymentSheet.",
  );
  const [stripeBusy, setStripeBusy] = useState(false);
  const { initPaymentSheet, presentPaymentSheet, isPlatformPaySupported } =
    useStripe();

  useEffect(() => {
    let cancelled = false;

    async function bootstrap() {
      try {
        const response = await vippsClient.getPaymentStatus();

        if (!cancelled) {
          setPayment(response.payment);
        }
      } catch (error) {
  const [applePaySupported, setApplePaySupported] = useState<boolean | null>(
    null,
  );
        console.error("Failed to bootstrap Vipps payment state", error);
  useEffect(() => {
    let cancelled = false;

    async function detectApplePaySupport() {
      try {
        const supported = await isPlatformPaySupported();
        if (!cancelled) {
          setApplePaySupported(supported);
        }
      } catch (error) {
        console.error("Failed to detect Apple Pay support", error);
        if (!cancelled) {
          setApplePaySupported(false);
        }
      }
    }

    void detectApplePaySupport();

    return () => {
      cancelled = true;
    };
  }, [isPlatformPaySupported]);
        if (!cancelled) {
          const message =
            error instanceof Error
              ? error.message
              : "Unable to reach the backend.";
          setFeedback(message);
        }
      }

    if (flow === "apple_pay" && applePaySupported === false) {
      setStripeFeedback(
        "Apple Pay is not available on this device/build. Use a physical iPhone with Wallet cards and a dev build, not Expo Go.",
      );
      return;
    }
    }

    void bootstrap();

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    let cancelled = false;

    async function syncAfterDeepLinkReturn() {
      setFeedback("Returned from Vipps app. Syncing payment status...");

      try {
        const response = await vippsClient.getPaymentStatus();

        if (!cancelled) {
          setPayment(response.payment);
          setFeedback(`Latest status: ${response.payment.status}.`);
        }
      } catch (error) {
        console.error(
          "Failed to sync status after Vipps deep link return",
          error,
        );

        if (!cancelled) {
          const message =
            error instanceof Error
              ? error.message
              : "Unable to sync payment state after returning from Vipps.";
          setFeedback(message);
        }
      }
    }

    const handleDeepLinkReturn = (event: { url: string }) => {
      if (!event.url.startsWith("solari-expo-test://vipps-return")) {
        return;
      }

      void syncAfterDeepLinkReturn();
    };

    const subscription = Linking.addEventListener("url", handleDeepLinkReturn);

    void Linking.getInitialURL().then((initialUrl) => {
      if (
        !initialUrl ||
        !initialUrl.startsWith("solari-expo-test://vipps-return")
      ) {
        return;
      }

      void syncAfterDeepLinkReturn();
    });

    return () => {
      cancelled = true;
      subscription.remove();
    };
  }, []);

  async function handleRefresh() {
    setBusy(true);
    setFeedback("Refreshing payment state from the backend...");

    try {
      const response = await vippsClient.getPaymentStatus();
      setPayment(response.payment);
      setFeedback(`Latest status: ${response.payment.status}.`);
    } catch (error) {
      console.error("Failed to refresh Vipps payment state", error);
      const message =
        error instanceof Error ? error.message : "Refresh failed.";
      setFeedback(message);
    } finally {
      setBusy(false);
    }
  }

  async function handleVippsPress() {
    setBusy(true);
    setAuthSessionResult(null);
    setFeedback("Creating the Vipps payment and opening the approval flow...");

    try {
      const result = await vippsClient.startVippsPayment();
      const authResult = (result as { authSessionResult?: string | null })
        .authSessionResult;
      setPayment(result.payment);
      setAuthSessionResult(authResult ?? null);

      if (result.payment.status === "completed") {
        setFeedback(
          `Vipps payment completed via ${result.apiBaseUrl} (auth session: ${authResult ?? "unknown"}).`,
        );
      } else if (result.payment.status === "pending") {
        setFeedback(
          `Payment is pending (auth session: ${authResult ?? "unknown"}). Check your webhook tunnel if it does not settle.`,
        );
      } else {
        setFeedback(
          `Vipps payment ended with status ${result.payment.status} (auth session: ${authResult ?? "unknown"}).`,
        );
      }
    } catch (error) {
      console.error("Vipps payment flow failed", error);
      const message =
        error instanceof Error ? error.message : "Vipps payment failed.";
      setFeedback(message);
    } finally {
      setBusy(false);
    }
  }

  async function handleStripePress(flow: "card" | "apple_pay") {
    if (!STRIPE_PUBLISHABLE_KEY) {
      setStripeFeedback(
        "Set EXPO_PUBLIC_STRIPE_PUBLISHABLE_KEY before using Stripe SDK in Expo.",
      );
      return;
    }

    setStripeBusy(true);
    setStripeFeedback(
      `Creating ${flow} intent and opening Stripe PaymentSheet...`,
    );

    try {
      const payload = {
        amount: 2500,
        currency: "nok",
          <Text style={styles.panelHint}>
            {applePaySupported === null
              ? "Checking Apple Pay availability..."
              : applePaySupported
                ? "Apple Pay is available on this device."
                : "Apple Pay unavailable here. PaymentSheet will fall back to Link/card methods."}
          </Text>
        description: `Solari ${flow} Expo test`,
      };

      const intent =
        flow === "apple_pay"
          ? await vippsClient.applePayPay(payload)
          : await vippsClient.stripePay(payload);

      const initResult = await initPaymentSheet({
        merchantDisplayName: "Solari Test",
        paymentIntentClientSecret: intent.client_secret,
        allowsDelayedPaymentMethods: true,
        returnURL: "solari-expo-test://vipps-return",
        applePay:
          flow === "apple_pay"
            ? {
                merchantCountryCode: "NO",
              }
            : undefined,
      });

      if (initResult.error) {
        setStripeFeedback(
          `Stripe init failed: ${formatStripeError(initResult.error)}`,
        );
        setStripeBusy(false);
        return;
      }

      const presentResult = await presentPaymentSheet();
      if (presentResult.error) {
        setStripeFeedback(
          `Stripe sheet failed: ${formatStripeError(presentResult.error)}`,
        );
      } else {
        setStripeFeedback("Stripe PaymentSheet completed.");
      }
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Stripe flow failed.";
      setStripeFeedback(message);
    } finally {
      setStripeBusy(false);
    }
  }

  return (
    <View style={styles.safeArea}>
      <StatusBar style="dark" />
      <ScrollView contentContainerStyle={styles.content}>
        <View style={styles.heroCard}>
          <Text style={styles.eyebrow}>Solari x Vipps</Text>
          <Text style={styles.title}>Pay with Vipps</Text>
          <Text style={styles.subtitle}>
            This screen drives your axum test backend and opens the Vipps
            approval flow.
          </Text>

          <VippsButtonNative onClick={handleVippsPress} />

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
            {vippsClient.resolveApiBaseUrl()}
          </Text>
          <Text style={styles.panelHint}>
            Set EXPO_PUBLIC_AXUM_BASE_URL to your ngrok HTTPS URL when testing
            on a real device.
          </Text>
        </View>

        <View style={styles.panel}>
          <Text style={styles.panelLabel}>Flow status</Text>
          <Text style={styles.feedback}>{feedback}</Text>
          <View style={styles.row}>
            <Text style={styles.rowLabel}>Auth session</Text>
            <Text style={styles.rowValue}>{authSessionResult ?? "-"}</Text>
          </View>
        </View>

        <View style={styles.panel}>
          <Text style={styles.panelLabel}>Stripe SDK</Text>
          <Text style={styles.panelHint}>
            Uses Stripe PaymentSheet from the Expo frontend SDK.
          </Text>
          <View style={styles.buttonRow}>
            <Pressable
              accessibilityRole="button"
              disabled={stripeBusy}
              onPress={() => void handleStripePress("card")}
              style={({ pressed }) => [
                styles.stripeButton,
                pressed && !stripeBusy ? styles.secondaryButtonPressed : null,
              ]}
            >
              <Text style={styles.stripeButtonLabel}>Pay card (2500 NOK)</Text>
            </Pressable>
            <Pressable
              accessibilityRole="button"
              disabled={stripeBusy}
              onPress={() => void handleStripePress("apple_pay")}
              style={({ pressed }) => [
                styles.appleButton,
                pressed && !stripeBusy ? styles.secondaryButtonPressed : null,
              ]}
            >
              <Text style={styles.stripeButtonLabel}>Pay Apple Pay intent</Text>
            </Pressable>
          </View>
          <Text style={styles.feedback}>{stripeFeedback}</Text>
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
    </View>
  );
}

function formatStripeError(error: unknown): string {
  if (error && typeof error === "object") {
    const maybeMessage = (error as { message?: unknown }).message;
    if (typeof maybeMessage === "string" && maybeMessage.trim().length > 0) {
      return maybeMessage;
    }

    const maybeCode = (error as { code?: unknown }).code;
    if (typeof maybeCode === "string" && maybeCode.trim().length > 0) {
      return maybeCode;
    }
  }

  return "Stripe payment sheet error";
}

export default function App() {
  if (!STRIPE_PUBLISHABLE_KEY) {
    return (
      <View style={styles.safeArea}>
        <StatusBar style="dark" />
        <View style={styles.missingStripeContainer}>
          <Text style={styles.title}>Missing Stripe key</Text>
          <Text style={styles.panelHint}>
            Set EXPO_PUBLIC_STRIPE_PUBLISHABLE_KEY, then restart Expo to enable
            Stripe SDK flows.
          </Text>
          <Text style={styles.panelHint}>Vipps flow still works.</Text>
          <StripeProvider publishableKey="pk_test_placeholder">
            <AppScreen />
          </StripeProvider>
        </View>
      </View>
    );
  }

  return (
    <StripeProvider
      publishableKey={STRIPE_PUBLISHABLE_KEY}
      merchantIdentifier="merchant.com.solari.test"
      urlScheme="solari-expo-test"
    >
      <AppScreen />
    </StripeProvider>
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
  },
  rowLabel: {
    color: "#8f7a63",
    fontSize: 13,
    fontWeight: "700",
    textTransform: "uppercase",
  },
  rowValue: {
    color: "#2f2418",
    fontSize: 14,
    fontWeight: "600",
  },
  stackRow: {
    gap: 6,
  },
  stackValue: {
    color: "#2f2418",
    fontSize: 14,
  },
  buttonRow: {
    gap: 10,
  },
  stripeButton: {
    alignItems: "center",
    backgroundColor: "#635bff",
    borderRadius: 999,
    justifyContent: "center",
    minHeight: 48,
    paddingHorizontal: 16,
  },
  appleButton: {
    alignItems: "center",
    backgroundColor: "#1f1f1f",
    borderRadius: 999,
    justifyContent: "center",
    minHeight: 48,
    paddingHorizontal: 16,
  },
  stripeButtonLabel: {
    color: "#ffffff",
    fontSize: 14,
    fontWeight: "700",
  },
  missingStripeContainer: {
    paddingHorizontal: 20,
    paddingTop: 32,
    gap: 12,
  },
});
