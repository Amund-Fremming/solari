"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { vippsPaymentService, VIPPS_COLORS } from "@solari/solari-js";

export default function VippsReturnPage() {
  const router = useRouter();
  const [status, setStatus] = useState("Processing payment...");
  const [payment, setPayment] = useState<any>(null);

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
          setStatus("✓ Payment completed successfully!");
          setTimeout(() => {
            router.push("/");
          }, 2000);
        } else {
          setStatus(`Payment status: ${response.payment.status}`);
          setTimeout(() => {
            router.push("/");
          }, 3000);
        }
      } catch (error) {
        setStatus(
          `Error checking payment status: ${formatErrorMessage(error)}`,
        );
        setTimeout(() => {
          router.push("/");
        }, 3000);
      }
    };

    checkPaymentStatus();
  }, [router]);

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
        <p style={{ color: "#999", marginTop: "20px" }}>
          Redirecting back to home...
        </p>
      </div>
    </main>
  );
}
