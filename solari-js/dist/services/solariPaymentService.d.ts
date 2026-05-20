import type { ApplePayPayPacket, AxumPayPacket, PaymentApiResponse, SolariClientOptions, StripePaymentIntentResponse, StripePayPacket, VippsCreatePaymentPacket, VippsCreatePaymentResponse, VippsPayPacket, VippsPayResponse, VippsStartFlowOptions, VippsStartFlowResult, VippsTokenResponse } from "../types/types.js";
export declare const VIPPS_COLORS: {
    readonly primary: "#ff5b24";
    readonly dark: "#23160d";
    readonly light: "#fff8ef";
};
export declare class SolariPaymentService {
    private readonly baseUrl;
    private readonly logger;
    private readonly openUrl?;
    constructor(options?: SolariClientOptions);
    resolveApiBaseUrl(): string;
    private request;
    pay(payload?: AxumPayPacket): Promise<PaymentApiResponse>;
    getPaymentStatus(): Promise<PaymentApiResponse>;
    resetPayment(): Promise<PaymentApiResponse>;
    vippsPay(payload: VippsPayPacket): Promise<VippsPayResponse>;
    vippsGetToken(): Promise<VippsTokenResponse>;
    vippsFetchToken(): Promise<VippsTokenResponse>;
    vippsCreatePayment(payload: VippsCreatePaymentPacket): Promise<VippsCreatePaymentResponse>;
    startVippsPayment(options?: VippsStartFlowOptions): Promise<VippsStartFlowResult>;
    applePayPay(payload: ApplePayPayPacket): Promise<StripePaymentIntentResponse>;
    stripePay(payload: StripePayPacket): Promise<StripePaymentIntentResponse>;
}
export type VippsPaymentClient = SolariPaymentService;
export type CreateWebClientOptions = SolariClientOptions;
export type CreateNativeClientOptions = SolariClientOptions;
export declare function createWebClient(options?: CreateWebClientOptions): SolariPaymentService;
export declare function createNativeClient(options?: CreateNativeClientOptions): SolariPaymentService;
export declare function createWebClientNoRedirect(options?: CreateWebClientOptions): SolariPaymentService;
export declare function createNativeClientNoRedirect(options?: CreateNativeClientOptions): SolariPaymentService;
export declare const create_web_client: typeof createWebClient;
export declare const create_native_client: typeof createNativeClient;
export declare const create_web_client_no_redirect: typeof createWebClientNoRedirect;
export declare const create_native_client_no_redirect: typeof createNativeClientNoRedirect;
export declare const vippsPaymentService: SolariPaymentService;
export declare function getPaymentStatus(): Promise<PaymentApiResponse>;
export declare function resetPayment(): Promise<PaymentApiResponse>;
export declare function startVippsPayment(options?: VippsStartFlowOptions): Promise<VippsStartFlowResult>;
//# sourceMappingURL=solariPaymentService.d.ts.map