export const VIPPS_COLORS = {
    primary: "#ff5b24",
    dark: "#23160d",
    light: "#fff8ef",
};
const DEFAULT_BASE_URL = "http://127.0.0.1:3001";
const TERMINAL_PAYMENT_STATUSES = new Set(["completed", "failed", "cancelled"]);
function getEnv(name) {
    if (typeof process !== "undefined" && process.env?.[name]) {
        return process.env[name];
    }
    const runtime = globalThis;
    return runtime.process?.env?.[name];
}
function resolveDefaultBaseUrl() {
    const configured = getEnv("NEXT_PUBLIC_AXUM_BASE_URL") ?? getEnv("EXPO_PUBLIC_AXUM_BASE_URL");
    return configured?.trim().replace(/\/$/, "") || DEFAULT_BASE_URL;
}
function defaultLogger() {
    return {
        error: (event, payload) => console.error(`[solari] ${event}`, payload),
        warn: (event, payload) => console.warn(`[solari] ${event}`, payload),
        info: (event, payload) => console.info(`[solari] ${event}`, payload),
    };
}
function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
function serializeBody(body) {
    if (body === undefined) {
        return undefined;
    }
    return JSON.stringify(body);
}
function normalizeErrorMessage(raw) {
    if (raw instanceof Error && raw.message) {
        return raw.message;
    }
    return "Unexpected client error";
}
function parseJsonSafely(rawText) {
    if (!rawText) {
        return null;
    }
    try {
        return JSON.parse(rawText);
    }
    catch {
        return null;
    }
}
function extractErrorMessage(status, method, path, parsedBody, rawText) {
    if (parsedBody && typeof parsedBody === "object") {
        const body = parsedBody;
        const nestedError = body.error;
        if (typeof nestedError === "string" && nestedError.trim().length > 0) {
            return `HTTP ${status} for ${method} ${path}: ${nestedError}`;
        }
        if (nestedError && typeof nestedError === "object") {
            const errObj = nestedError;
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
    constructor(options = {}) {
        this.baseUrl = (options.baseUrl || resolveDefaultBaseUrl()).replace(/\/$/, "");
        this.logger = options.logger || defaultLogger();
        this.openUrl = options.openUrl;
    }
    resolveApiBaseUrl() {
        return this.baseUrl;
    }
    async request(method, path, body) {
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
            const parsedBody = parsedUnknown;
            if (!response.ok) {
                const message = extractErrorMessage(response.status, method, path, parsedUnknown, rawText);
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
                throw new Error(`Expected JSON response for ${method} ${path}, received: ${rawText || "<empty>"}`);
            }
            return parsedBody;
        }
        catch (error) {
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
    pay(payload = {}) {
        return this.request("POST", "/pay", payload);
    }
    // GET /status
    getPaymentStatus() {
        return this.request("GET", "/status");
    }
    // POST /wipe
    resetPayment() {
        return this.request("POST", "/wipe", {});
    }
    // ---------- Vipps API (/solari/vipps/*) ----------
    // POST /solari/vipps/pay
    vippsPay(payload) {
        return this.request("POST", "/solari/vipps/pay", payload);
    }
    // GET /solari/vipps/token
    vippsGetToken() {
        return this.request("GET", "/solari/vipps/token");
    }
    // POST /solari/vipps/token/fetch
    vippsFetchToken() {
        return this.request("POST", "/solari/vipps/token/fetch", {});
    }
    // POST /solari/vipps/payments
    vippsCreatePayment(payload) {
        return this.request("POST", "/solari/vipps/payments", payload);
    }
    async startVippsPayment(options = {}) {
        const payPayload = {
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
        let authSessionResult = null;
        const urlOpener = options.openUrl ?? this.openUrl;
        if (urlOpener) {
            const result = await urlOpener(redirectUrl);
            authSessionResult = result ?? "opened";
        }
        else if (typeof window !== "undefined" &&
            typeof window.open === "function") {
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
        const intervalMs = options.pollIntervalMs ?? 2500;
        const timeoutMs = options.pollTimeoutMs ?? 90000;
        const startedAt = Date.now();
        let latest = started.payment;
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
    applePayPay(payload) {
        return this.request("POST", "/solari/apple-pay/pay", payload).catch((error) => {
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
    stripePay(payload) {
        return this.request("POST", "/solari/stripe/pay", payload).catch((error) => {
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
export function createWebClient(options = {}) {
    return new SolariPaymentService(options);
}
export function createNativeClient(options = {}) {
    return new SolariPaymentService(options);
}
export function createWebClientNoRedirect(options = {}) {
    return new SolariPaymentService(options);
}
export function createNativeClientNoRedirect(options = {}) {
    return new SolariPaymentService(options);
}
export const create_web_client = createWebClient;
export const create_native_client = createNativeClient;
export const create_web_client_no_redirect = createWebClientNoRedirect;
export const create_native_client_no_redirect = createNativeClientNoRedirect;
export const vippsPaymentService = new SolariPaymentService();
export async function getPaymentStatus() {
    return vippsPaymentService.getPaymentStatus();
}
export async function resetPayment() {
    return vippsPaymentService.resetPayment();
}
export async function startVippsPayment(options = {}) {
    return vippsPaymentService.startVippsPayment(options);
}
//# sourceMappingURL=solariPaymentService.js.map