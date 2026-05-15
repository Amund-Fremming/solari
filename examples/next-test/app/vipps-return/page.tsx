"use client";

import { useEffect, useState } from "react";
import {
  createWebClient,
  type PaymentSnapshot,
  VIPPS_COLORS,
} from "@solari/solari-js";

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
  const [status, setStatus] = useState("Checking payment status...");
  const [details, setDetails] = useState(
    "Please wait while we sync the payment with your app.",
  );
  const [nextAction, setNextAction] = useState<
    { label: string; href: string } | undefined
  >(undefined);
  const [payment, setPayment] = useState<PaymentSnapshot | null>(null);

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

          // If browser prevents closing, keep user on this page with clear guidance.
          window.setTimeout(() => {
            if (!window.closed) {
              setStatus("Payment synced");
              setDetails("You can close this window and continue in your app.");
              setNextAction(undefined);
            }
          }, 250);
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
          setStatus("Payment completed");
          setDetails("Closing this window and returning to your app...");
          finishFlow(700, true, {
            type: "solari-vipps-return",
            ok: true,
            status: response.payment.status,
            redirectTo: successRedirectPath,
          });
        } else {
          setStatus(`Payment status: ${response.payment.status}`);
          setDetails("Returning to your app...");
          finishFlow(1_200, true, {
            type: "solari-vipps-return",
            ok: false,
            status: response.payment.status,
            redirectTo: fallbackRedirectPath,
          });
        }
      } catch (error) {
        setStatus("Could not verify payment");
        setDetails(
          `You can close this window and continue in your app. (${formatErrorMessage(error)})`,
        );
        setNextAction({ label: "Return to app", href: fallbackRedirectPath });
        finishFlow(2_000, true, {
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
          padding: "36px",
          borderRadius: "16px",
          textAlign: "center",
          boxShadow: "0 20px 45px rgba(35, 22, 13, 0.12)",
          width: "min(560px, calc(100vw - 32px))",
        }}
      >
        <h1
          style={{
            color: VIPPS_COLORS.primary,
            marginBottom: "10px",
            fontSize: "clamp(1.4rem, 3vw, 2rem)",
          }}
        >
          {status}
        </h1>
        <p style={{ color: VIPPS_COLORS.dark, margin: "0 0 20px" }}>
          {details}
        </p>
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

        {nextAction && (
          <a
            href={nextAction.href}
            style={{
              display: "inline-block",
              marginTop: "18px",
              backgroundColor: VIPPS_COLORS.primary,
              color: "white",
              textDecoration: "none",
              padding: "10px 16px",
              borderRadius: "999px",
              fontWeight: 600,
            }}
          >
            {nextAction.label}
          </a>
        )}
      </div>
    </main>
  );
}
