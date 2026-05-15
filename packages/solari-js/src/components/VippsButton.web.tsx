import { createElement, useEffect, useRef, useState } from "react";
import type { VippsButtonProps } from "./VippsButton";
import { loadVippsButtonScript, propsToAttributes } from "./VippsButton";

export function VippsButton(props: VippsButtonProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const isLoadedRef = useRef(false);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    const loadScript = async () => {
      if (isLoadedRef.current) return;
      isLoadedRef.current = true;

      try {
        await loadVippsButtonScript();
      } catch (error) {
        console.error("Failed to load Vipps button script:", error);
      }
    };

    // Only load in browser environment
    if (typeof window !== "undefined") {
      loadScript();
    }
  }, []);

  const handleClick = async () => {
    if (isLoading) {
      return;
    }

    try {
      setIsLoading(true);
      await props.onClick();
    } catch (error) {
      console.error("Vipps button click handler error:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const attributes = propsToAttributes(isLoading);

  return (
    <div ref={containerRef} style={{ display: "block", width: "100%" }}>
      {createElement("vipps-mobilepay-button", {
        ...attributes,
        style: { display: "block", width: "100%" },
        onClick: handleClick,
        disabled: isLoading,
      })}
    </div>
  );
}
