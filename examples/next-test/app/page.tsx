"use client";

import { type FormEvent, useEffect, useMemo, useState } from "react";
import {
  Elements,
  PaymentElement,
  useElements,
  useStripe,
} from "@stripe/react-stripe-js";
import { loadStripe } from "@stripe/stripe-js";
import {
  VippsButtonWeb,
  createWebClient,
  type PaymentSnapshot,
  VIPPS_COLORS,
} from "@solari/solari-js";

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

type VippsReturnMessage = {
  type: "solari-vipps-return";
  ok: boolean;
  status?: string;
  redirectTo?: string;
};

function isVippsReturnMessage(payload: unknown): payload is VippsReturnMessage {
  if (!payload || typeof payload !== "object") {
    return false;
  }

  const candidate = payload as Partial<VippsReturnMessage>;
  return (
    candidate.type === "solari-vipps-return" &&
    typeof candidate.ok === "boolean"
  );
}

const vippsClient = createWebClient();

type StripePayPacket = {
  amount: number;
  currency?: string;
  description?: string;
};

type StripePayResult = {
  flow: string;
  payment_intent_id: string;
  client_secret: string;
  publishable_key: string;
};

type StripeIntentClientCompat = {
  applePayPay: (payload: StripePayPacket) => Promise<StripePayResult>;
  stripePay: (payload: StripePayPacket) => Promise<StripePayResult>;
};

type StripeFlow = "card" | "apple_pay";

function StripeCheckoutForm(props: {
  onResult: (message: string) => void;
  disabled?: boolean;
}) {
  const stripe = useStripe();
  const elements = useElements();
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (!stripe || !elements) {
      return;
    }

    setSubmitting(true);
    const result = await stripe.confirmPayment({
      elements,
      redirect: "if_required",
    });

    if (result.error) {
      props.onResult(result.error.message || "Stripe confirmation failed.");
      setSubmitting(false);
      return;
    }

    const status = result.paymentIntent?.status || "unknown";
    props.onResult(`Stripe confirm result: ${status}`);
    setSubmitting(false);
  };

  return (
    <form onSubmit={handleSubmit}>
      <PaymentElement />
      <button
        type="submit"
        disabled={!stripe || !elements || submitting || props.disabled}
        style={{
          marginTop: "12px",
          backgroundColor: "#635bff",
          color: "white",
          border: "none",
          padding: "12px 24px",
          borderRadius: "4px",
          cursor: "pointer",
          fontSize: "16px",
        }}
      >
        {submitting ? "Confirming..." : "Confirm Payment"}
      </button>
    </form>
  );
}

export default function HomePage() {
  const [payment, setPayment] = useState<PaymentSnapshot>(FALLBACK_STATUS);
  const [isLoading, setIsLoading] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [networkError, setNetworkError] = useState<string | null>(null);
  const [paymentNotice, setPaymentNotice] = useState<string | null>(null);
  const [stripeAmount, setStripeAmount] = useState(25);
  const [stripeClientSecret, setStripeClientSecret] = useState<string | null>(
    null,
  );
  const [stripePublishableKey, setStripePublishableKey] = useState<
    string | null
  >(null);
  const [stripeIntentId, setStripeIntentId] = useState<string | null>(null);
  const [stripeFlow, setStripeFlow] = useState<StripeFlow>("card");
  const [stripeMessage, setStripeMessage] = useState<string | null>(null);

  const stripePromise = useMemo(
    () => (stripePublishableKey ? loadStripe(stripePublishableKey) : null),
    [stripePublishableKey],
  );

  const formatErrorMessage = (error: unknown): string => {
    if (error instanceof Error && error.message) {
      return error.message;
    }

    return "Unable to reach payment backend.";
  };

  const refreshPaymentStatus = async () => {
    try {
      const response = await vippsClient.getPaymentStatus();
      setPayment(response.payment);
      setNetworkError(null);
    } catch (error) {
      setNetworkError(formatErrorMessage(error));
    }
  };

  // Auto-refresh payment status
  useEffect(() => {
    if (!autoRefresh) return;

    const interval = setInterval(refreshPaymentStatus, 2000);
    return () => clearInterval(interval);
  }, [autoRefresh]);

  // Initial load
  useEffect(() => {
    refreshPaymentStatus();
  }, []);

  useEffect(() => {
    const query = new URLSearchParams(window.location.search);
    if (query.get("payment") !== "success") {
      return;
    }

    setPaymentNotice("Payment completed successfully.");

    query.delete("payment");
    const nextQuery = query.toString();
    const nextUrl = `${window.location.pathname}${nextQuery ? `?${nextQuery}` : ""}`;
    window.history.replaceState(null, "", nextUrl);
  }, []);

  useEffect(() => {
    const handleVippsReturnMessage = (event: MessageEvent<unknown>) => {
      if (event.origin !== window.location.origin) {
        return;
      }

      if (!isVippsReturnMessage(event.data)) {
        return;
      }

      if (event.data.ok) {
        setPaymentNotice("Payment completed successfully.");

        if (event.data.redirectTo) {
          window.location.assign(event.data.redirectTo);
          return;
        }
      }

      void refreshPaymentStatus();
    };

    window.addEventListener("message", handleVippsReturnMessage);
    return () => {
      window.removeEventListener("message", handleVippsReturnMessage);
    };
  }, []);

  const handleStartPayment = async () => {
    setIsLoading(true);
    try {
      const result = await vippsClient.startVippsPayment();
      setPayment(result.payment);
      setNetworkError(null);
    } catch (error) {
      const errorMessage = formatErrorMessage(error);

      setPayment((prev: PaymentSnapshot) => ({
        ...prev,
        status: "failed",
        last_error: errorMessage,
      }));
      setNetworkError(errorMessage);
    } finally {
      setIsLoading(false);
    }
  };

  const handleReset = async () => {
    try {
      const response = await vippsClient.resetPayment();
      setPayment(response.payment);
      setNetworkError(null);
    } catch (error) {
      setNetworkError(formatErrorMessage(error));
    }
  };

  const handleCreateStripeIntent = async (flow: StripeFlow) => {
    const amount = Math.round(stripeAmount * 100);
    if (!Number.isFinite(amount) || amount <= 0) {
      setStripeMessage("Amount must be greater than 0.");
      return;
    }

    setIsLoading(true);
    setStripeMessage("Creating Stripe payment intent...");

    try {
      // Keep the example working across SDK versions with slightly different d.ts surfaces.
      const stripeIntentClient =
        vippsClient as unknown as StripeIntentClientCompat;
      const payload: StripePayPacket = {
        amount,
        currency: "nok",
        description: `Solari ${flow} test`,
      };

      const intent =
        flow === "apple_pay"
          ? await stripeIntentClient.applePayPay(payload)
          : await stripeIntentClient.stripePay(payload);

      setStripeFlow(flow);
      setStripeClientSecret(intent.client_secret);
      setStripePublishableKey(intent.publishable_key);
      setStripeIntentId(intent.payment_intent_id);
      setStripeMessage(
        `Intent ready (${intent.flow}). Continue in Stripe checkout below.`,
      );
    } catch (error) {
      setStripeMessage(formatErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const statusColor: string =
    (
      {
        idle: "#888",
        pending: "#ff9800",
        completed: "#4caf50",
        failed: "#f44336",
        cancelled: "#ff9800",
      } as Record<string, string>
    )[payment.status] || "#888";

  return (
    <main style={{ minHeight: "100vh", backgroundColor: VIPPS_COLORS.light }}>
      <div style={{ maxWidth: "600px", margin: "0 auto", padding: "20px" }}>
        <h1>Solari - Next.js Test</h1>
        <p>Payment testing interface for Vipps integration</p>

        {networkError && (
          <section
            style={{
              backgroundColor: "#fff1f0",
              border: "1px solid #ffccc7",
              color: "#a8071a",
              padding: "12px 16px",
              borderRadius: "8px",
              marginBottom: "20px",
            }}
          >
            <strong>Backend connection error:</strong> {networkError}
          </section>
        )}

        {paymentNotice && (
          <section
            style={{
              backgroundColor: "#f6ffed",
              border: "1px solid #b7eb8f",
              color: "#135200",
              padding: "12px 16px",
              borderRadius: "8px",
              marginBottom: "20px",
            }}
          >
            <strong>{paymentNotice}</strong>
          </section>
        )}

        <section
          style={{
            backgroundColor: "white",
            padding: "20px",
            borderRadius: "8px",
            marginBottom: "20px",
            boxShadow: "0 2px 4px rgba(0,0,0,0.1)",
          }}
        >
          <h2>Payment Status</h2>
          <div
            style={{
              padding: "10px",
              backgroundColor: statusColor,
              color: "white",
              borderRadius: "4px",
              marginBottom: "10px",
              fontWeight: "bold",
            }}
          >
            {payment.status.toUpperCase()}
          </div>

          <div style={{ fontSize: "14px", color: "#666" }}>
            <p>
              <strong>Amount:</strong> {payment.requested_amount} NOK (paid:{" "}
              {payment.paid_amount})
            </p>
            <p>
              <strong>Reference:</strong> {payment.reference || "N/A"}
            </p>
            <p>
              <strong>Attempts:</strong> {payment.attempts}
            </p>
            <p>
              <strong>Updated:</strong> {payment.updated_from}
            </p>
            {payment.last_error && (
              <p style={{ color: "#f44336" }}>
                <strong>Error:</strong> {payment.last_error}
              </p>
            )}
          </div>
        </section>

        <section
          style={{
            backgroundColor: "white",
            padding: "20px",
            borderRadius: "8px",
            marginBottom: "20px",
            boxShadow: "0 2px 4px rgba(0,0,0,0.1)",
          }}
        >
          <h2>Controls</h2>
          <div style={{ display: "flex", gap: "10px", marginBottom: "10px" }}>
            <VippsButtonWeb onClick={handleStartPayment} />
            <button
              onClick={refreshPaymentStatus}
              style={{
                backgroundColor: "#2196F3",
                color: "white",
                border: "none",
                padding: "12px 24px",
                borderRadius: "4px",
                cursor: "pointer",
                fontSize: "16px",
              }}
            >
              Refresh Status
            </button>
            <button
              onClick={handleReset}
              style={{
                backgroundColor: "#666",
                color: "white",
                border: "none",
                padding: "12px 24px",
                borderRadius: "4px",
                cursor: "pointer",
                fontSize: "16px",
              }}
            >
              Reset
            </button>
          </div>

          <label
            style={{
              display: "flex",
              alignItems: "center",
              gap: "8px",
              cursor: "pointer",
            }}
          >
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
            />
            Auto-refresh status
          </label>
        </section>

        <section
          style={{
            backgroundColor: "white",
            padding: "20px",
            borderRadius: "8px",
            marginBottom: "20px",
            boxShadow: "0 2px 4px rgba(0,0,0,0.1)",
          }}
        >
          <h2>Stripe SDK Checkout</h2>
          <p style={{ color: "#555", fontSize: "14px" }}>
            Create a payment intent via Solari backend, then confirm it using
            Stripe.js.
          </p>

          <div style={{ display: "flex", gap: "10px", marginBottom: "12px" }}>
            <input
              type="number"
              min={1}
              step={1}
              value={stripeAmount}
              onChange={(event) => setStripeAmount(Number(event.target.value))}
              style={{
                border: "1px solid #ccc",
                borderRadius: "4px",
                padding: "10px",
                width: "120px",
              }}
            />
            <button
              onClick={() => void handleCreateStripeIntent("card")}
              style={{
                backgroundColor: "#635bff",
                color: "white",
                border: "none",
                padding: "12px 16px",
                borderRadius: "4px",
                cursor: "pointer",
              }}
            >
              Card Intent
            </button>
            <button
              onClick={() => void handleCreateStripeIntent("apple_pay")}
              style={{
                backgroundColor: "#2f2f2f",
                color: "white",
                border: "none",
                padding: "12px 16px",
                borderRadius: "4px",
                cursor: "pointer",
              }}
            >
              Apple Pay Intent
            </button>
          </div>

          {stripeIntentId && (
            <p style={{ marginBottom: "8px", color: "#555", fontSize: "14px" }}>
              Intent: {stripeIntentId} ({stripeFlow})
            </p>
          )}

          {stripeMessage && (
            <p
              style={{ marginBottom: "12px", color: "#333", fontSize: "14px" }}
            >
              {stripeMessage}
            </p>
          )}

          {!stripeClientSecret || !stripePromise ? (
            <p style={{ color: "#777", fontSize: "14px" }}>
              Create an intent first to load Stripe Elements.
            </p>
          ) : (
            <Elements
              stripe={stripePromise}
              options={{ clientSecret: stripeClientSecret }}
            >
              <StripeCheckoutForm
                disabled={isLoading}
                onResult={(message) => setStripeMessage(message)}
              />
            </Elements>
          )}
        </section>

        <section
          style={{
            backgroundColor: "white",
            padding: "20px",
            borderRadius: "8px",
            boxShadow: "0 2px 4px rgba(0,0,0,0.1)",
          }}
        >
          <h2>API Endpoint</h2>
          <code
            style={{
              backgroundColor: "#f5f5f5",
              padding: "10px",
              borderRadius: "4px",
              display: "block",
              wordBreak: "break-all",
              fontSize: "12px",
            }}
          >
            {vippsClient.resolveApiBaseUrl()}
          </code>
        </section>
      </div>
    </main>
  );
}
