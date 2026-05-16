export type JsonPrimitive = string | number | boolean | null;
export type JsonValue =
  | JsonPrimitive
  | JsonValue[]
  | { [key: string]: JsonValue };
export type AnyObject = Record<string, unknown>;

export type PaymentStatus =
  | "idle"
  | "pending"
  | "completed"
  | "failed"
  | "cancelled"
  | string;

export interface PaymentSnapshot {
  provider: string;
  status: PaymentStatus;
  requested_amount: number;
  paid_amount: number;
  reference: string | null;
  redirect_url: string | null;
  return_url: string | null;
  attempts: number;
  updated_from: string;
  last_error: string | null;
  raw_status?: string | null;
  last_webhook_payload?: unknown | null;
}

export interface PaymentApiResponse {
  ok: boolean;
  payment: PaymentSnapshot;
}

export interface VippsPayPacket {
  amount: number;
  return_url?: string;
}

export interface VippsPayResponse {
  provider: string;
  status: string;
  paid: number;
  reference: string | null;
  redirect_url: string | null;
  return_url: string | null;
}

export interface VippsTokenResponse {
  access_token: string;
  expires_at: number;
}

export interface VippsCreatePaymentPacket {
  amount: number;
  return_url?: string;
}

export interface VippsCreatePaymentResponse {
  reference: string | null;
  redirect_url: string | null;
}

export interface AxumPayPacket {
  amount?: number;
  return_url?: string;
}

export type ApplePayPayPacket = AnyObject;
export type StripePayPacket = AnyObject;

export interface SolariResponse<T = AnyObject> {
  ok?: boolean;
  data?: T;
  [key: string]: unknown;
}

export interface SolariLogger {
  error: (event: string, payload?: AnyObject) => void;
  warn: (event: string, payload?: AnyObject) => void;
  info: (event: string, payload?: AnyObject) => void;
}

export interface SolariClientOptions {
  baseUrl?: string;
  logger?: SolariLogger;
  openUrl?: VippsUrlOpener;
}

export type VippsUrlOpener = (
  redirectUrl: string,
) => Promise<string | null | void> | string | null | void;

export interface VippsStartFlowOptions {
  amount?: number;
  returnUrl?: string;
  openUrl?: VippsUrlOpener;
  waitForTerminalStatus?: boolean;
  pollIntervalMs?: number;
  pollTimeoutMs?: number;
}

export interface VippsStartFlowResult {
  payment: PaymentSnapshot;
  apiBaseUrl: string;
  authSessionResult: string | null;
}
