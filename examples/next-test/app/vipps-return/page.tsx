"use client";

import { useEffect, useState } from "react";
import { vippsPaymentService, VIPPS_COLORS } from "@solari/solari-js";

export default function VippsReturnPage() {
  const [status, setStatus] = useState("Processing payment...");
  const [payment, setPayment] = useState<any>(null);

  const finishFlow = (delayMs: number, closePopup: boolean) => {
    window.setTimeout(() => {
      const openedAsPopup = !!window.opener && !window.opener.closed;

      if (closePopup && openedAsPopup) {
        try {
          window.opener.postMessage(
            { type: "solari-vipps-return", ok: true },
            window.location.origin,
          );
        } catch {
          // Ignore cross-window messaging issues and continue closing.
        }

        window.close();
        return;
      }

      window.location.replace("/");
    }, delayMs);
  };

  const formatErrorMessage = (error: unknown): string => {
    if (error instanceof Error && error.message) {
      return error.message;
    }

    return "Unknown error";
  };

  useEffect(() => {
    const checkPaymentStatus = async () => {
      try {
        // Wait a moment for the backend to process the redirect
        await new Promise((resolve) => setTimeout(resolve, 1000));

        const response = await vippsPaymentService.getPaymentStatus();
        setPayment(response.payment);

        if (response.payment.status === "completed") {
          setStatus("Payment completed. Closing window...");
          finishFlow(700, true);
        } else {
          setStatus(
            `Payment status: ${response.payment.status}. Returning to app...`,
          );
          finishFlow(1_500, false);
        }
      } catch (error) {
        setStatus(
          `Error checking payment status: ${formatErrorMessage(error)}`,
        );
        finishFlow(2_000, false);
      }
    };

    checkPaymentStatus();
  }, []);

  return (
    <main
      style={{
        minHeight: "100vh",
        backgroundColor: VIPPS_COLORS.light,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div
        style={{
          backgroundColor: "white",
          padding: "40px",
          borderRadius: "8px",
          textAlign: "center",
          boxShadow: "0 2px 8px rgba(0,0,0,0.1)",
        }}
      >
        <h1 style={{ color: VIPPS_COLORS.primary, marginBottom: "20px" }}>
          {status}
        </h1>
        {payment && (
          <div style={{ color: "#666", fontSize: "14px" }}>
            <p>
              <strong>Reference:</strong> {payment.reference || "N/A"}
            </p>
            <p>
              <strong>Amount:</strong> {payment.paid_amount} NOK
            </p>
          </div>
        )}
        <p style={{ color: "#999", marginTop: "20px" }}>Returning to app...</p>
      </div>
    </main>
  );
}
