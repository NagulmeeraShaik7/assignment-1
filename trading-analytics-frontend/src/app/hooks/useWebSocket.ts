"use client";

import { useEffect, useState, useRef } from "react";

export interface RsiMessage {
  token_address: string;
  rsi: number;
  timestamp: string;
}

export const useWebSocket = (url: string) => {
  const [messages, setMessages] = useState<RsiMessage[]>([]);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket(url);
    wsRef.current = ws;

    ws.onopen = () => {
      console.log("✅ WebSocket connected:", url);
    };

    ws.onmessage = (event) => {
      try {
        const data: RsiMessage = JSON.parse(event.data);
        setMessages((prev) => [data, ...prev].slice(0, 50)); // Keep last 50
      } catch (error) {
        console.error("❌ Invalid WebSocket message:", error);
      }
    };

    ws.onclose = () => {
      console.log("❌ WebSocket disconnected");
    };

    return () => {
      ws.close();
    };
  }, [url]);

  return messages;
};
