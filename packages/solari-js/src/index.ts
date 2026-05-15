export type PaymentStatus = "approved" | "declined";

export interface PaymentSummary {
  provider: string;
  transactionId: string;
  status: PaymentStatus;
}

export function isApproved(summary: PaymentSummary): boolean {
  return summary.status === "approved";
}

// Re-export Vipps payment service
export * from "./services/solariPaymentService";
export { vippsPaymentService } from "./services/solariPaymentService";

// Re-export Vipps button components and types
export * from "./components";
