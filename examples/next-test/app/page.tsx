"use client";

import { useEffect, useState } from "react";
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

const vippsClient = createWebClient({
  callbackUrl: process.env.NEXT_PUBLIC_VIPPS_WEB_RETURN_URL,
});

export default function HomePage() {
  const [payment, setPayment] = useState<PaymentSnapshot>(FALLBACK_STATUS);
  const [isLoading, setIsLoading] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [networkError, setNetworkError] = useState<string | null>(null);
  const [paymentNotice, setPaymentNotice] = useState<string | null>(null);

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

      setPayment((prev) => ({
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
