"use client";

import { useWebSocket, RsiMessage } from "./hooks/useWebSocket";

const WS_URL = "ws://localhost:8081/ws";

export default function HomePage() {
  const messages = useWebSocket(WS_URL);

  return (
    <div style={{ padding: "2rem" }}>
      <h1 style={{ fontSize: "1.5rem", fontWeight: "bold", marginBottom: "1rem" }}>
        🚀 RSI Dashboard
      </h1>

      {messages.length === 0 ? (
        <p>Waiting for live RSI data...</p>
      ) : (
        <table style={{ width: "100%", borderCollapse: "collapse" }}>
          <thead>
            <tr style={{ backgroundColor: "#ccc", color: "#333" }}>
              <th style={{ border: "1px solid #ccc", padding: "8px" }}>Token</th>
              <th style={{ border: "1px solid #ccc", padding: "8px" }}>RSI</th>
              <th style={{ border: "1px solid #ccc", padding: "8px" }}>Timestamp</th>
            </tr>
          </thead>
          <tbody>
            {messages.map((msg: RsiMessage, index) => (
              <tr key={index}>
                <td style={{ border: "1px solid #ccc", textAlign: "center", padding: "6px" }}>
                  {msg.token_address}
                </td>
                <td style={{ border: "1px solid #ccc", textAlign: "center", padding: "6px" }}>
                  {msg.rsi.toFixed(2)}
                </td>
                <td style={{ border: "1px solid #ccc", textAlign: "center", padding: "6px" }}>
                  {msg.timestamp}
                </td>
                
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
