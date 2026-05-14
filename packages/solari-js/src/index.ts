export type PaymentStatus = "approved" | "declined";

export interface PaymentSummary {
  provider: string;
  transactionId: string;
  status: PaymentStatus;
}

export function isApproved(summary: PaymentSummary): boolean {
  return summary.status === "approved";
}
