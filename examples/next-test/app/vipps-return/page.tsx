"use client";

import { useEffect, useState } from "react";
import { createWebClient, VIPPS_COLORS } from "@solari/solari-js";

const vippsClient = createWebClient({
  callbackUrl: process.env.NEXT_PUBLIC_VIPPS_WEB_RETURN_URL,
});

type ReturnMessage = {
  type: "solari-vipps-return";
  ok: boolean;
  status: string;
  redirectTo: string;
};

function getSafeRedirectPath(
  rawRedirectPath: string | null,
  defaultPath: string,
): string {
  if (!rawRedirectPath) {
    return defaultPath;
  }

  try {
    const parsedUrl = new URL(rawRedirectPath, window.location.origin);
    if (parsedUrl.origin !== window.location.origin) {
      return defaultPath;
    }

    return `${parsedUrl.pathname}${parsedUrl.search}${parsedUrl.hash}`;
  } catch {
    return defaultPath;
  }
}

export default function VippsReturnPage() {
  const [status, setStatus] = useState("Processing payment...");
  const [payment, setPayment] = useState<any>(null);

  const finishFlow = (
    delayMs: number,
    closePopup: boolean,
    message: ReturnMessage,
  ) => {
    window.setTimeout(() => {
      const openedAsPopup = !!window.opener && !window.opener.closed;

      if (openedAsPopup) {
        try {
          window.opener.postMessage(message, window.location.origin);
        } catch {
          // Ignore cross-window messaging issues and continue with redirect/close.
        }

        if (closePopup) {
          window.close();
          return;
        }
      }

      window.location.replace(message.redirectTo);
    }, delayMs);
  };

  const formatErrorMessage = (error: unknown): string => {
    if (error instanceof Error && error.message) {
      return error.message;
    }

    return "Unknown error";
  };

  useEffect(() => {
    const query = new URLSearchParams(window.location.search);
    const successRedirectPath = getSafeRedirectPath(
      query.get("success_redirect"),
      "/?payment=success",
    );
    const fallbackRedirectPath = getSafeRedirectPath(
      query.get("fallback_redirect"),
      "/",
    );

    const checkPaymentStatus = async () => {
      try {
        // Wait a moment for the backend to process the redirect
        await new Promise((resolve) => setTimeout(resolve, 1000));

        const response = await vippsClient.getPaymentStatus();
        setPayment(response.payment);

        if (response.payment.status === "completed") {
          setStatus("Payment completed. Closing window...");
          finishFlow(700, true, {
            type: "solari-vipps-return",
            ok: true,
            status: response.payment.status,
            redirectTo: successRedirectPath,
          });
        } else {
          setStatus(
            `Payment status: ${response.payment.status}. Returning to app...`,
          );
          finishFlow(1_500, false, {
            type: "solari-vipps-return",
            ok: false,
            status: response.payment.status,
            redirectTo: fallbackRedirectPath,
          });
        }
      } catch (error) {
        setStatus(
          `Error checking payment status: ${formatErrorMessage(error)}`,
        );
        finishFlow(2_000, false, {
          type: "solari-vipps-return",
          ok: false,
          status: "error",
          redirectTo: fallbackRedirectPath,
        });
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
