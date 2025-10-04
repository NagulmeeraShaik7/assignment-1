import { NextPage } from "next";
import { useWebSocket, RsiMessage } from "../hooks/useWebSocket";

const WS_URL = "ws://localhost:8081/ws";

const Home: NextPage = () => {
  const messages = useWebSocket(WS_URL);

  return (
    <div style={{ padding: "2rem" }}>
      <h1>🚀 RSI Dashboard</h1>
      <table style={{ width: "100%", borderCollapse: "collapse" }}>
        <thead>
          <tr>
            <th style={{ border: "1px solid black" }}>Token</th>
            <th style={{ border: "1px solid black" }}>RSI</th>
            <th style={{ border: "1px solid black" }}>Timestamp</th>
          </tr>
        </thead>
        <tbody>
          {messages.map((msg: RsiMessage, index) => (
            <tr key={index}>
              <td style={{ border: "1px solid black", textAlign: "center" }}>{msg.token_address}</td>
              <td style={{ border: "1px solid black", textAlign: "center" }}>{msg.rsi.toFixed(2)}</td>
              <td style={{ border: "1px solid black", textAlign: "center" }}>{msg.timestamp}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Home;
