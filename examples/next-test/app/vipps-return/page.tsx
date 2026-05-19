"use client";

import { useEffect } from "react";
import { createWebClient } from "@solari/solari-js";

const vippsClient = createWebClient();

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
  const finishFlow = (delayMs: number, message: ReturnMessage) => {
    window.setTimeout(() => {
      const openedAsPopup = !!window.opener && !window.opener.closed;

      if (openedAsPopup) {
        try {
          window.opener.postMessage(message, window.location.origin);
        } catch {
          // Ignore cross-window messaging issues and continue closing.
        }
      }

      window.close();
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

        if (response.payment.status === "completed") {
          finishFlow(700, {
            type: "solari-vipps-return",
            ok: true,
            status: response.payment.status,
            redirectTo: successRedirectPath,
          });
        } else {
          finishFlow(1_200, {
            type: "solari-vipps-return",
            ok: false,
            status: response.payment.status,
            redirectTo: fallbackRedirectPath,
          });
        }
      } catch (error) {
        console.warn("Unable to verify Vipps payment in return page", error);
        finishFlow(2_000, {
          type: "solari-vipps-return",
          ok: false,
          status: "error",
          redirectTo: fallbackRedirectPath,
        });
      }
    };

    checkPaymentStatus();
  }, []);

  return null;
}
