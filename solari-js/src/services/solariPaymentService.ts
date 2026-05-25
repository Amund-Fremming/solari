import type {
  AnyObject,
  ApplePayPayPacket,
  AxumPayPacket,
  PaymentApiResponse,
  PaymentSnapshot,
  SolariClientOptions,
  SolariLogger,
  StripePaymentIntentResponse,
  StripePayPacket,
  VippsCreatePaymentPacket,
  VippsCreatePaymentResponse,
  VippsPayPacket,
  VippsPayResponse,
  VippsStartFlowOptions,
  VippsStartFlowResult,
  VippsTokenResponse,
} from "../types/types.js";

const DEFAULT_BASE_URL = "http://127.0.0.1:3001";
const TERMINAL_PAYMENT_STATUSES = new Set(["completed", "failed", "cancelled"]);

function normalizeBaseUrl(rawUrl: string): string {
  const trimmed = rawUrl.trim().replace(/\/$/, "");

  if (trimmed.length === 0) {
    return DEFAULT_BASE_URL;
  }

  if (/^https?:\/\//i.test(trimmed)) {
    return trimmed;
  }

  return `https://${trimmed}`;
}

function getEnv(name: string): string | undefined {
  if (typeof process !== "undefined" && process.env?.[name]) {
    return process.env[name];
  }

  const runtime = globalThis as {
    process?: { env?: Record<string, string | undefined> };
  };
  return runtime.process?.env?.[name];
}

function resolveDefaultBaseUrl(): string {
  const configured =
    getEnv("NEXT_PUBLIC_AXUM_BASE_URL") ?? getEnv("EXPO_PUBLIC_AXUM_BASE_URL");

  return configured ? normalizeBaseUrl(configured) : DEFAULT_BASE_URL;
}

function defaultLogger(): SolariLogger {
  return {
    error: (event: string, payload?: AnyObject) =>
      console.error(`[solari] ${event}`, payload),
    warn: (event: string, payload?: AnyObject) =>
      console.warn(`[solari] ${event}`, payload),
    info: (event: string, payload?: AnyObject) =>
      console.info(`[solari] ${event}`, payload),
  };
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function serializeBody(body: unknown): string | undefined {
  if (body === undefined) {
    return undefined;
  }

  return JSON.stringify(body);
}

function normalizeErrorMessage(raw: unknown): string {
  if (raw instanceof Error && raw.message) {
    return raw.message;
  }

  return "Unexpected client error";
}

function parseJsonSafely(rawText: string): unknown | null {
  if (!rawText) {
    return null;
  }

  try {
    return JSON.parse(rawText) as unknown;
  } catch {
    return null;
  }
}

function extractErrorMessage(
  status: number,
  method: string,
  path: string,
  parsedBody: unknown,
  rawText: string,
): string {
  if (parsedBody && typeof parsedBody === "object") {
    const body = parsedBody as Record<string, unknown>;
    const nestedError = body.error;

    if (typeof nestedError === "string" && nestedError.trim().length > 0) {
      return `HTTP ${status} for ${method} ${path}: ${nestedError}`;
    }

    if (nestedError && typeof nestedError === "object") {
      const errObj = nestedError as Record<string, unknown>;
      const message = errObj.message;
      const code = errObj.code;
      if (typeof message === "string" && message.trim().length > 0) {
        if (typeof code === "string" && code.trim().length > 0) {
          return `HTTP ${status} for ${method} ${path}: ${message} (code: ${code})`;
        }
        return `HTTP ${status} for ${method} ${path}: ${message}`;
      }
    }

    const message = body.message;
    if (typeof message === "string" && message.trim().length > 0) {
      return `HTTP ${status} for ${method} ${path}: ${message}`;
    }
  }

  if (rawText.trim().length > 0) {
    return `HTTP ${status} for ${method} ${path}: ${rawText}`;
  }

  return `HTTP ${status} for ${method} ${path}`;
}

export class SolariPaymentService {
  private readonly baseUrl: string;
  private readonly logger: SolariLogger;
  private readonly openUrl?: SolariClientOptions["openUrl"];

  constructor(options: SolariClientOptions = {}) {
    this.baseUrl = normalizeBaseUrl(options.baseUrl || resolveDefaultBaseUrl());
    this.logger = options.logger || defaultLogger();
    this.openUrl = options.openUrl;
  }

  resolveApiBaseUrl(): string {
    return this.baseUrl;
  }

  private async request<TResponse>(
    method: "GET" | "POST",
    path: string,
    body?: unknown,
  ): Promise<TResponse> {
    const url = `${this.baseUrl}${path}`;

    try {
      const response = await fetch(url, {
        method,
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: serializeBody(body),
      });

      const rawText = await response.text();
      const parsedUnknown = parseJsonSafely(rawText);
      const parsedBody = parsedUnknown as TResponse | null;

      if (!response.ok) {
        const message = extractErrorMessage(
          response.status,
          method,
          path,
          parsedUnknown,
          rawText,
        );
        this.logger.error("request_failed", {
          method,
          path,
          status: response.status,
          message,
          response: rawText || null,
        });
        throw new Error(message);
      }

      if (parsedBody === null) {
        this.logger.error("invalid_json_response", {
          method,
          path,
          response: rawText || null,
        });
        throw new Error(
          `Expected JSON response for ${method} ${path}, received: ${rawText || "<empty>"}`,
        );
      }

      return parsedBody;
    } catch (error) {
      this.logger.error("request_exception", {
        method,
        path,
        message: normalizeErrorMessage(error),
      });
      throw error;
    }
  }

  // ---------- Axum test endpoints ----------
  // POST /pay
  pay(payload: AxumPayPacket = {}): Promise<PaymentApiResponse> {
    return this.request<PaymentApiResponse>("POST", "/pay", payload);
  }

  // GET /status
  getPaymentStatus(): Promise<PaymentApiResponse> {
    return this.request<PaymentApiResponse>("GET", "/status");
  }

  // POST /wipe
  resetPayment(): Promise<PaymentApiResponse> {
    return this.request<PaymentApiResponse>("POST", "/wipe", {});
  }

  // ---------- Vipps API (/solari/vipps/*) ----------
  // POST /solari/vipps/pay
  vippsPay(payload: VippsPayPacket): Promise<VippsPayResponse> {
    return this.request<VippsPayResponse>("POST", "/solari/vipps/pay", payload);
  }

  // GET /solari/vipps/token
  vippsGetToken(): Promise<VippsTokenResponse> {
    return this.request<VippsTokenResponse>("GET", "/solari/vipps/token");
  }

  // POST /solari/vipps/token/fetch
  vippsFetchToken(): Promise<VippsTokenResponse> {
    return this.request<VippsTokenResponse>(
      "POST",
      "/solari/vipps/token/fetch",
      {},
    );
  }

  // POST /solari/vipps/payments
  vippsCreatePayment(
    payload: VippsCreatePaymentPacket,
  ): Promise<VippsCreatePaymentResponse> {
    return this.request<VippsCreatePaymentResponse>(
      "POST",
      "/solari/vipps/payments",
      payload,
    );
  }

  async startVippsPayment(
    options: VippsStartFlowOptions = {},
  ): Promise<VippsStartFlowResult> {
    const payPayload: AxumPayPacket = {
      amount: options.amount,
      return_url: options.returnUrl,
    };

    const started = await this.pay(payPayload);
    const redirectUrl = started.payment.redirect_url;

    if (!redirectUrl) {
      this.logger.warn("vipps_missing_redirect_url", { endpoint: "/pay" });
      return {
        payment: started.payment,
        apiBaseUrl: this.baseUrl,
        authSessionResult: null,
      };
    }

    let authSessionResult: string | null = null;
    const urlOpener = options.openUrl ?? this.openUrl;

    if (urlOpener) {
      const result = await urlOpener(redirectUrl);
      authSessionResult = result ?? "opened";
    } else if (
      typeof window !== "undefined" &&
      typeof window.open === "function"
    ) {
      window.open(redirectUrl, "vipps-payment", "width=600,height=800");
      authSessionResult = "opened";
    }

    if (!options.waitForTerminalStatus) {
      return {
        payment: started.payment,
        apiBaseUrl: this.baseUrl,
        authSessionResult,
      };
    }

    const intervalMs = options.pollIntervalMs ?? 2_500;
    const timeoutMs = options.pollTimeoutMs ?? 90_000;
    const startedAt = Date.now();
    let latest: PaymentSnapshot = started.payment;

    while (!TERMINAL_PAYMENT_STATUSES.has(latest.status)) {
      if (Date.now() - startedAt >= timeoutMs) {
        break;
      }

      await sleep(intervalMs);
      latest = (await this.getPaymentStatus()).payment;
    }

    return {
      payment: latest,
      apiBaseUrl: this.baseUrl,
      authSessionResult,
    };
  }

  // ---------- Apple Pay API (/solari/apple-pay/*) ----------
  // POST /solari/apple-pay/pay
  applePayPay(
    payload: ApplePayPayPacket,
  ): Promise<StripePaymentIntentResponse> {
    return this.request<StripePaymentIntentResponse>(
      "POST",
      "/solari/apple-pay/pay",
      payload,
    ).catch((error) => {
      this.logger.error("apple_pay_intent_failed", {
        endpoint: "/solari/apple-pay/pay",
        amount: payload.amount,
        currency: payload.currency ?? null,
        message: normalizeErrorMessage(error),
      });
      throw error;
    });
  }

  // ---------- Stripe API (/solari/stripe/*) ----------
  // POST /solari/stripe/pay
  stripePay(payload: StripePayPacket): Promise<StripePaymentIntentResponse> {
    return this.request<StripePaymentIntentResponse>(
      "POST",
      "/solari/stripe/pay",
      payload,
    ).catch((error) => {
      this.logger.error("stripe_intent_failed", {
        endpoint: "/solari/stripe/pay",
        amount: payload.amount,
        currency: payload.currency ?? null,
        message: normalizeErrorMessage(error),
      });
      throw error;
    });
  }
}

export type VippsPaymentClient = SolariPaymentService;
export type CreateWebClientOptions = SolariClientOptions;
export type CreateNativeClientOptions = SolariClientOptions;

export function createWebClient(
  options: CreateWebClientOptions = {},
): SolariPaymentService {
  return new SolariPaymentService(options);
}

export function createNativeClient(
  options: CreateNativeClientOptions = {},
): SolariPaymentService {
  return new SolariPaymentService(options);
}

export function createWebClientNoRedirect(
  options: CreateWebClientOptions = {},
): SolariPaymentService {
  return new SolariPaymentService(options);
}

export function createNativeClientNoRedirect(
  options: CreateNativeClientOptions = {},
): SolariPaymentService {
  return new SolariPaymentService(options);
}

export const create_web_client = createWebClient;
export const create_native_client = createNativeClient;
export const create_web_client_no_redirect = createWebClientNoRedirect;
export const create_native_client_no_redirect = createNativeClientNoRedirect;

export const vippsPaymentService = new SolariPaymentService();

export async function getPaymentStatus(): Promise<PaymentApiResponse> {
  return vippsPaymentService.getPaymentStatus();
}

export async function resetPayment(): Promise<PaymentApiResponse> {
  return vippsPaymentService.resetPayment();
}

export async function startVippsPayment(
  options: VippsStartFlowOptions = {},
): Promise<VippsStartFlowResult> {
  return vippsPaymentService.startVippsPayment(options);
}
