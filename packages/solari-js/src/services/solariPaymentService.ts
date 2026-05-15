// Vipps official colors and branding
export const VIPPS_COLORS = {
  primary: "#ff5b24", // Official Vipps orange
  dark: "#23160d",
  light: "#fff8ef",
} as const;

const TERMINAL_STATUSES = new Set(["completed", "failed", "cancelled"]);
const POLL_INTERVAL_MS = 2_500;
const POLL_TIMEOUT_MS = 90_000;
const DEFAULT_SUCCESS_REDIRECT_PATH = "/?payment=success";
const DEFAULT_FALLBACK_REDIRECT_PATH = "/";

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

type ApiGetter = () => Promise<ApiResponse>;
type ClientPlatform = "web" | "native";

export type VippsPaymentClient = {
  getPaymentStatus: () => Promise<ApiResponse>;
  resetPayment: () => Promise<ApiResponse>;
  startVippsPayment: () => Promise<VippsPaymentFlowResult>;
  resolveApiBaseUrl: () => string;
  getCallbackUrl: () => string;
};

export type CreateWebClientOptions = {
  apiBaseUrl?: string;
  callbackUrl?: string;
};

export type CreateNativeClientOptions = {
  apiBaseUrl?: string;
  callbackUrl?: string;
  redirectScheme?: string;
  openUrl?: NativeUrlOpener;
};

type NativeUrlOpener = (
  redirectUrl: string,
  appReturnUrl: string,
) => Promise<string | null | void> | string | null | void;

type VippsClientConfig = {
  platform: ClientPlatform;
  apiBaseUrl?: string;
  callbackUrl?: string;
  redirectScheme?: string;
  openUrl?: NativeUrlOpener;
  noRedirect?: boolean;
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

function withDefaultWebRedirectParams(returnUrl: string): string {
  if (!/^https?:\/\//i.test(returnUrl)) {
    return returnUrl;
  }

  try {
    const parsedUrl = new URL(returnUrl);

    if (!parsedUrl.searchParams.has("success_redirect")) {
      parsedUrl.searchParams.set(
        "success_redirect",
        DEFAULT_SUCCESS_REDIRECT_PATH,
      );
    }

    if (!parsedUrl.searchParams.has("fallback_redirect")) {
      parsedUrl.searchParams.set(
        "fallback_redirect",
        DEFAULT_FALLBACK_REDIRECT_PATH,
      );
    }

    return parsedUrl.toString();
  } catch {
    return returnUrl;
  }
}

function getDefaultWebCallbackUrl(): string {
  if (typeof window !== "undefined") {
    return withDefaultWebRedirectParams(
      `${window.location.origin}/vipps-return`,
    );
  }

  return withDefaultWebRedirectParams("http://localhost:3000/vipps-return");
}

function getDefaultNativeCallbackUrl(configuredScheme?: string): string {
  const normalizedScheme = configuredScheme?.trim().replace(/:\/\/$/, "");

  if (typeof globalThis !== "undefined") {
    const globalScope = globalThis as any;
    const Linking = globalScope.ExpoLinking || globalScope.Linking;

    if (Linking && typeof Linking.createURL === "function") {
      try {
        const generatedUrl = normalizedScheme
          ? Linking.createURL("vipps-return", { scheme: normalizedScheme })
          : Linking.createURL("vipps-return");
        if (typeof generatedUrl === "string" && generatedUrl.trim()) {
          return generatedUrl;
        }
      } catch {
        // Fall through to static default.
      }
    }
  }
  if (normalizedScheme) {
    return `${normalizedScheme}://vipps-return`;
  }

  return "solari-expo-test://vipps-return";
}

function isWebUrl(candidate: string): boolean {
  return /^https?:\/\//i.test(candidate);
}

function normalizeApiBaseUrl(candidate?: string): string | undefined {
  if (!candidate?.trim()) {
    return undefined;
  }

  return candidate.trim().replace(/\/$/, "");
}

function normalizeCallbackUrl(candidate?: string): string | undefined {
  if (!candidate?.trim()) {
    return undefined;
  }

  return candidate.trim();
}

function resolveWebCallbackUrl(configuredCallbackUrl?: string): string {
  const callbackFromOptions = normalizeCallbackUrl(configuredCallbackUrl);
  if (callbackFromOptions) {
    return withDefaultWebRedirectParams(callbackFromOptions);
  }

  const callbackFromEnvironment = normalizeCallbackUrl(
    getEnvironmentVariable("NEXT_PUBLIC_VIPPS_WEB_RETURN_URL"),
  );
  if (callbackFromEnvironment) {
    return withDefaultWebRedirectParams(callbackFromEnvironment);
  }

  return getDefaultWebCallbackUrl();
}

function resolveNativeCallbackUrl(
  configuredCallbackUrl?: string,
  configuredScheme?: string,
): string {
  const normalized = normalizeCallbackUrl(configuredCallbackUrl);

  if (normalized && !isWebUrl(normalized)) {
    return normalized;
  }

  if (normalized && isWebUrl(normalized)) {
    console.warn(
      "Ignoring web callbackUrl for native Vipps flow. Using app deep-link scheme instead.",
    );
  }

  return getDefaultNativeCallbackUrl(configuredScheme);
}

function getVippsProviderReturnUrl(
  apiBaseUrl: string,
  appReturnUrl: string,
): string {
  const encodedAppReturnUrl = encodeURIComponent(appReturnUrl);
  return `${apiBaseUrl}/vipps-return?app_return_url=${encodedAppReturnUrl}`;
}

function resolveClientCallbackUrl(config: VippsClientConfig): string {
  if (config.platform === "native") {
    return resolveNativeCallbackUrl(config.callbackUrl, config.redirectScheme);
  }

  return resolveWebCallbackUrl(config.callbackUrl);
}

function resolveClientApiBaseUrl(config: VippsClientConfig): string {
  const callbackApiBaseUrl = normalizeApiBaseUrl(config.apiBaseUrl);
  if (callbackApiBaseUrl) {
    return callbackApiBaseUrl;
  }

  return resolveApiBaseUrl();
}

function createDefaultClient(): VippsPaymentClient {
  if (detectPlatform() === "expo") {
    return createNativeClient();
  }

  return createWebClient();
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

async function request<T>(
  apiBaseUrl: string,
  path: string,
  init?: RequestInit,
): Promise<T> {
  const targetUrl = `${apiBaseUrl}${path}`;

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
  platform: ClientPlatform,
  appReturnUrl: string,
  preopenedWindow?: Window | null,
  nativeUrlOpener?: NativeUrlOpener,
  noRedirect?: boolean,
): Promise<string> {
  if (platform === "native") {
    if (nativeUrlOpener) {
      const openerResult = await nativeUrlOpener(redirectUrl, appReturnUrl);
      return openerResult ?? "opened";
    }

    if (noRedirect) {
      try {
        const globalScope = globalThis as any;
        const Linking = globalScope.ExpoLinking || globalScope.Linking;

        if (Linking && typeof Linking.openURL === "function") {
          await Linking.openURL(redirectUrl);
          return "opened";
        }
      } catch (error) {
        console.warn("Failed to use native Linking.openURL", error);
      }
    }

    // Use expo-web-browser for Expo - access from global scope
    try {
      const globalScope = globalThis as any;
      const WebBrowser = globalScope.ExpoWebBrowser || globalScope.WebBrowser;

      if (WebBrowser && typeof WebBrowser.openAuthSessionAsync === "function") {
        const result = await WebBrowser.openAuthSessionAsync(
          redirectUrl,
          appReturnUrl,
        );
        return result.type;
      }
    } catch (error) {
      console.warn("Failed to use expo-web-browser", error);
    }

    // Try React Native Linking globals as a native deep-link fallback.
    try {
      const globalScope = globalThis as any;
      const Linking = globalScope.ExpoLinking || globalScope.Linking;

      if (Linking && typeof Linking.openURL === "function") {
        await Linking.openURL(redirectUrl);
        return "opened";
      }
    } catch (error) {
      console.warn("Failed to use native Linking.openURL", error);
    }

    // Fallback to window.open
    if (typeof window !== "undefined" && typeof window.open === "function") {
      window.open(redirectUrl, "vipps-payment", "width=600,height=800");
      return "opened";
    }

    throw new Error(
      "Unable to open Vipps flow in Expo. Provide CreateNativeClientOptions.openUrl (for example Linking.openURL) or initialize expo-web-browser.",
    );
  }

  // Use window.open for web
  if (typeof window !== "undefined" && typeof window.open === "function") {
    if (preopenedWindow && !preopenedWindow.closed) {
      preopenedWindow.location.href = redirectUrl;
      preopenedWindow.focus();

      if (noRedirect) {
        setTimeout(() => {
          if (!preopenedWindow.closed) {
            preopenedWindow.close();
          }
        }, 1200);
      }

      return "opened";
    }

    const popup = window.open(
      redirectUrl,
      "vipps-payment",
      "width=600,height=800",
    );
    if (!popup) {
      // Fallback to same-tab navigation when the browser blocks popups.
      window.location.assign(redirectUrl);
      return "opened-same-tab";
    }

    if (noRedirect) {
      setTimeout(() => {
        if (!popup.closed) {
          popup.close();
        }
      }, 1200);
    }

    return "opened";
  }

  throw new Error("Cannot open auth session in this environment");
}

async function waitForFinalStatusWith(
  getPaymentStatus: ApiGetter,
): Promise<PaymentSnapshot> {
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

function createVippsClient(config: VippsClientConfig): VippsPaymentClient {
  const getCallbackUrl = () => resolveClientCallbackUrl(config);
  const getResolvedApiBaseUrl = () => resolveClientApiBaseUrl(config);

  const getPaymentStatus = async (): Promise<ApiResponse> => {
    return request<ApiResponse>(getResolvedApiBaseUrl(), "/status", {
      method: "GET",
    });
  };

  const resetPayment = async (): Promise<ApiResponse> => {
    return request<ApiResponse>(getResolvedApiBaseUrl(), "/wipe", {
      method: "POST",
    });
  };

  const startVippsPayment = async (): Promise<VippsPaymentFlowResult> => {
    const preopenedWindow =
      config.platform === "web" ? preopenWebPaymentWindow() : null;
    const apiBaseUrl = getResolvedApiBaseUrl();
    const appReturnUrl = config.noRedirect ? "" : getCallbackUrl();

    await resetPayment();

    const payload = config.noRedirect
      ? {}
      : {
          return_url: getVippsProviderReturnUrl(apiBaseUrl, appReturnUrl),
        };

    const startedPayment = await request<ApiResponse>(apiBaseUrl, "/pay", {
      method: "POST",
      body: JSON.stringify(payload),
    });

    if (!startedPayment.payment.redirect_url) {
      throw new Error("Backend did not return a Vipps redirect URL.");
    }

    const authSessionResult = await openAuthUrl(
      startedPayment.payment.redirect_url,
      config.platform,
      appReturnUrl,
      preopenedWindow,
      config.openUrl,
      config.noRedirect,
    );

    if (config.noRedirect) {
      return {
        payment: startedPayment.payment,
        apiBaseUrl,
        authSessionResult,
      };
    }

    const payment = await waitForFinalStatusWith(getPaymentStatus);

    return {
      payment,
      apiBaseUrl,
      authSessionResult,
    };
  };

  return {
    getPaymentStatus,
    resetPayment,
    startVippsPayment,
    resolveApiBaseUrl: getResolvedApiBaseUrl,
    getCallbackUrl,
  };
}

export function createWebClient(
  options: CreateWebClientOptions = {},
): VippsPaymentClient {
  return createVippsClient({
    platform: "web",
    ...options,
  });
}

export function createNativeClient(
  options: CreateNativeClientOptions = {},
): VippsPaymentClient {
  return createVippsClient({
    platform: "native",
    ...options,
  });
}

export function createWebClientNoRedirect(
  options: CreateWebClientOptions = {},
): VippsPaymentClient {
  return createVippsClient({
    platform: "web",
    noRedirect: true,
    ...options,
  });
}

export function createNativeClientNoRedirect(
  options: CreateNativeClientOptions = {},
): VippsPaymentClient {
  return createVippsClient({
    platform: "native",
    noRedirect: true,
    ...options,
  });
}

export const create_web_client = createWebClient;
export const create_native_client = createNativeClient;
export const create_web_client_no_redirect = createWebClientNoRedirect;
export const create_native_client_no_redirect = createNativeClientNoRedirect;

function resolveDefaultApiBaseUrl(): string {
  return createDefaultClient().resolveApiBaseUrl();
}

export async function getPaymentStatus(): Promise<ApiResponse> {
  return createDefaultClient().getPaymentStatus();
}

export async function resetPayment(): Promise<ApiResponse> {
  return createDefaultClient().resetPayment();
}

export async function startVippsPayment(): Promise<VippsPaymentFlowResult> {
  return createDefaultClient().startVippsPayment();
}

export const vippsPaymentService = {
  getPaymentStatus,
  resetPayment,
  startVippsPayment,
  createWebClientNoRedirect,
  createNativeClientNoRedirect,
  resolveApiBaseUrl: resolveDefaultApiBaseUrl,
};
