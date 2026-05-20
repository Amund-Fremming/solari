import { jsx as _jsx } from "react/jsx-runtime";
import { createElement, useEffect, useRef, useState } from "react";
import { loadVippsButtonScript, propsToAttributes } from "./VippsButton";
export function VippsButton(props) {
    const containerRef = useRef(null);
    const isLoadedRef = useRef(false);
    const [isPressed, setIsPressed] = useState(false);
    useEffect(() => {
        const loadScript = async () => {
            if (isLoadedRef.current)
                return;
            isLoadedRef.current = true;
            try {
                await loadVippsButtonScript();
            }
            catch (error) {
                console.error("Failed to load Vipps button script:", error);
            }
        };
        // Only load in browser environment
        if (typeof window !== "undefined") {
            loadScript();
        }
    }, []);
    const handleClick = async () => {
        try {
            await props.onClick();
        }
        catch (error) {
            console.error("Vipps button click handler error:", error);
        }
    };
    const attributes = propsToAttributes();
    return (_jsx("div", { ref: containerRef, style: { display: "block", width: "100%" }, children: createElement("vipps-mobilepay-button", {
            ...attributes,
            style: {
                display: "block",
                width: "100%",
                cursor: "pointer",
                transition: "transform 120ms ease, filter 120ms ease",
                transform: isPressed ? "scale(0.985)" : "scale(1)",
                filter: isPressed ? "brightness(0.96)" : "none",
            },
            onClick: handleClick,
            onPointerDown: () => setIsPressed(true),
            onPointerUp: () => setIsPressed(false),
            onPointerCancel: () => setIsPressed(false),
            onPointerLeave: () => setIsPressed(false),
            onKeyDown: () => setIsPressed(true),
            onKeyUp: () => setIsPressed(false),
        }) }));
}
//# sourceMappingURL=VippsButton.web.js.map