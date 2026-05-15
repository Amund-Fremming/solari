// Vipps official colors and branding
export const VIPPS_COLORS = {
  primary: "#ff5b24", // Official Vipps orange
  dark: "#23160d",
  light: "#fff8ef",
} as const;

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
  apiBaseUrl: string;
  authSessionResult: string | null;
};

// Platform detection
type Platform = "expo" | "web" | "unknown";

function detectPlatform(): Platform {
  // Check for browser environment first (most common for web)
  if (typeof window !== "undefined" && typeof document !== "undefined") {
    return "web";
  }

  // Check for React Native / Expo environment
  if (typeof globalThis !== "undefined") {
    const g = globalThis as any;
    // React Native globals
    if (g.React?.createElement || g.__DEV__ !== undefined || g.expo) {
      return "expo";
    }
  }

  return "unknown";
}

function getAppReturnUrl(): string {
  const platform = detectPlatform();

  if (platform === "expo") {
    return "solari-expo-test://vipps-return";
  }

  // Web/Next.js
  if (typeof window !== "undefined") {
    return `${window.location.origin}/vipps-return`;
  }

  return "http://localhost:3000/vipps-return";
}

function getVippsProviderReturnUrl(apiBaseUrl: string): string {
  const appReturnUrl = getAppReturnUrl();

  if (detectPlatform() === "expo") {
    const encodedAppReturnUrl = encodeURIComponent(appReturnUrl);
    return `${apiBaseUrl}/vipps-return?app_return_url=${encodedAppReturnUrl}`;
  }

  return appReturnUrl;
}

function getEnvironmentVariable(name: string): string | undefined {
  // In Node/SSR context
  if (typeof process !== "undefined" && process.env) {
    return process.env[name];
  }

  // In browser context, check for process.env (Next.js)
  if (typeof globalThis !== "undefined") {
    const g = globalThis as any;
    if (g.process?.env?.[name]) {
      return g.process.env[name];
    }
  }

  return undefined;
}

function getReactNativePlatform(): string {
  // Try to detect React Native Platform
  if (typeof globalThis !== "undefined") {
    const g = globalThis as any;
    // In React Native, Platform is a global or accessible through the RN environment
    if (g.Platform?.OS) {
      return g.Platform.OS;
    }
  }
  return "unknown";
}

function resolveApiBaseUrl(): string {
  const configuredBaseUrl =
    getEnvironmentVariable("NEXT_PUBLIC_AXUM_BASE_URL") ||
    getEnvironmentVariable("EXPO_PUBLIC_AXUM_BASE_URL");

  if (configuredBaseUrl?.trim()) {
    return configuredBaseUrl.trim().replace(/\/$/, "");
  }

  const platform = detectPlatform();

  if (platform === "expo") {
    // Expo-specific defaults
    const platformOS = getReactNativePlatform();
    if (platformOS === "android") {
      return "http://10.0.2.2:3001";
    }
  }

  // Default for web and fallback
  return "http://127.0.0.1:3001";
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const targetUrl = `${resolveApiBaseUrl()}${path}`;

  try {
    const response = await fetch(targetUrl, {
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        ...(init?.headers ?? {}),
      },
      ...init,
    });

    const rawBody = await response.text();
    let parsedBody: T | string | null = null;

    if (rawBody) {
      try {
        parsedBody = JSON.parse(rawBody) as T;
      } catch {
        parsedBody = rawBody;
      }
    }

    if (!response.ok) {
      const message =
        typeof parsedBody === "string"
          ? parsedBody
          : rawBody || `Request failed with ${response.status}`;

      throw new Error(message);
    }

    if (parsedBody === null) {
      throw new Error(`Empty response from ${path}`);
    }

    return parsedBody as T;
  } catch (error) {
    if (error instanceof TypeError) {
      throw new Error(
        `Network error: could not reach Solari backend at ${targetUrl}. ` +
          "Start the backend service or configure NEXT_PUBLIC_AXUM_BASE_URL/EXPO_PUBLIC_AXUM_BASE_URL.",
      );
    }

    throw error;
  }
}

async function delay(durationMs: number): Promise<void> {
  await new Promise((resolve) => {
    setTimeout(resolve, durationMs);
  });
}

function preopenWebPaymentWindow(): Window | null {
  if (typeof window === "undefined" || typeof window.open !== "function") {
    return null;
  }

  // Must run in direct response to a user action to avoid popup blockers.
  return window.open("", "vipps-payment", "width=600,height=800");
}

async function openAuthUrl(
  redirectUrl: string,
  preopenedWindow?: Window | null,
): Promise<string> {
  const platform = detectPlatform();

  if (platform === "expo") {
    // Use expo-web-browser for Expo - access from global scope
    try {
      const globalScope = globalThis as any;
      const WebBrowser = globalScope.ExpoWebBrowser || globalScope.WebBrowser;

      if (WebBrowser && typeof WebBrowser.openAuthSessionAsync === "function") {
        const result = await WebBrowser.openAuthSessionAsync(
          redirectUrl,
          getAppReturnUrl(),
        );
        return result.type;
      }
    } catch (error) {
      console.warn("Failed to use expo-web-browser", error);
    }

    // Fallback to window.open
    if (typeof window !== "undefined" && typeof window.open === "function") {
      window.open(redirectUrl, "vipps-payment", "width=600,height=800");
      return "opened";
    }

    throw new Error(
      "Unable to open Vipps flow in Expo. Ensure expo-web-browser is initialized and available to the shared SDK.",
    );
  }

  // Use window.open for web
  if (typeof window !== "undefined" && typeof window.open === "function") {
    if (preopenedWindow && !preopenedWindow.closed) {
      preopenedWindow.location.href = redirectUrl;
      preopenedWindow.focus();
      return "opened";
    }

    const popup = window.open(
      redirectUrl,
      "vipps-payment",
      "width=600,height=800",
    );
    if (!popup) {
      throw new Error(
        "Popup blocked by browser. Please allow popups for this site and try again.",
      );
    }
    return "opened";
  }

  throw new Error("Cannot open auth session in this environment");
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
  const preopenedWindow =
    detectPlatform() === "web" ? preopenWebPaymentWindow() : null;
  const apiBaseUrl = resolveApiBaseUrl();

  await resetPayment();

  const startedPayment = await request<ApiResponse>("/pay", {
    method: "POST",
    body: JSON.stringify({
      return_url: getVippsProviderReturnUrl(apiBaseUrl),
    }),
  });

  if (!startedPayment.payment.redirect_url) {
    throw new Error("Backend did not return a Vipps redirect URL.");
  }

  const authSessionResult = await openAuthUrl(
    startedPayment.payment.redirect_url,
    preopenedWindow,
  );
  const payment = await waitForFinalStatus();

  return {
    payment,
    apiBaseUrl,
    authSessionResult,
  };
}

export const vippsPaymentService = {
  getPaymentStatus,
  resetPayment,
  startVippsPayment,
  resolveApiBaseUrl,
};
