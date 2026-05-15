import * as WebBrowser from "expo-web-browser";
import { Platform } from "react-native";

WebBrowser.maybeCompleteAuthSession();

const APP_RETURN_URL = "solari-expo-test://vipps-return";
const TERMINAL_STATUSES = new Set(["completed", "failed", "cancelled"]);
const POLL_INTERVAL_MS = 2_500;
const POLL_TIMEOUT_MS = 90_000;

export type PaymentSnapshot = {
  provider: string;
  status: string;
  requested_amount: number;
  paid_amount: number;
  reference: string | null;
  redirect_url: string | null;
  return_url: string | null;
  attempts: number;
  updated_from: string;
  last_error: string | null;
  last_webhook_payload: unknown | null;
};

type ApiResponse = {
  ok: boolean;
  payment: PaymentSnapshot;
};

export type VippsPaymentFlowResult = {
  payment: PaymentSnapshot;
  browserResult: string;
  apiBaseUrl: string;
};

function readExpoPublicEnv(name: string): string | undefined {
  const maybeProcess = globalThis as {
    process?: {
      env?: Record<string, string | undefined>;
    };
  };

  return maybeProcess.process?.env?.[name];
}

function resolveApiBaseUrl(): string {
  const configuredBaseUrl = readExpoPublicEnv(
    "EXPO_PUBLIC_AXUM_BASE_URL",
  )?.trim();

  if (configuredBaseUrl) {
    return configuredBaseUrl.replace(/\/$/, "");
  }

  if (Platform.OS === "android") {
    return "http://10.0.2.2:3001";
  }

  return "http://127.0.0.1:3001";
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${resolveApiBaseUrl()}${path}`, {
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
      ...(init?.headers ?? {}),
    },
    ...init,
  });

  const rawBody = await response.text();
  const parsedBody = rawBody ? (JSON.parse(rawBody) as T) : null;

  if (!response.ok) {
    const message =
      typeof parsedBody === "string"
        ? parsedBody
        : rawBody || `Request failed with ${response.status}`;

    throw new Error(message);
  }

  return parsedBody as T;
}

async function delay(durationMs: number): Promise<void> {
  await new Promise((resolve) => {
    setTimeout(resolve, durationMs);
  });
}

async function waitForFinalStatus(): Promise<PaymentSnapshot> {
  const startedAt = Date.now();
  let latestPayment = (await getPaymentStatus()).payment;

  while (!TERMINAL_STATUSES.has(latestPayment.status)) {
    if (Date.now() - startedAt >= POLL_TIMEOUT_MS) {
      return latestPayment;
    }

    await delay(POLL_INTERVAL_MS);
    latestPayment = (await getPaymentStatus()).payment;
  }

  return latestPayment;
}

export async function getPaymentStatus(): Promise<ApiResponse> {
  return request<ApiResponse>("/status", {
    method: "GET",
  });
}

export async function resetPayment(): Promise<ApiResponse> {
  return request<ApiResponse>("/wipe", {
    method: "POST",
  });
}

export async function startVippsPayment(): Promise<VippsPaymentFlowResult> {
  await resetPayment();

  const startedPayment = await request<ApiResponse>("/pay", {
    method: "POST",
    body: JSON.stringify({
      return_url: APP_RETURN_URL,
    }),
  });

  if (!startedPayment.payment.redirect_url) {
    throw new Error("Backend did not return a Vipps redirect URL.");
  }

  const browserResult = await WebBrowser.openAuthSessionAsync(
    startedPayment.payment.redirect_url,
    APP_RETURN_URL,
  );

  const payment = await waitForFinalStatus();

  return {
    payment,
    browserResult: browserResult.type,
    apiBaseUrl: resolveApiBaseUrl(),
  };
}

export const vippsPaymentService = {
  getPaymentStatus,
  resetPayment,
  startVippsPayment,
  resolveApiBaseUrl,
};
