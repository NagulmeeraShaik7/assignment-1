import fs from "fs";
import csv from "csv-parser";
import { Kafka } from "kafkajs";

const kafka = new Kafka({
  clientId: "csv-producer",
  brokers: ["localhost:19092"], // Redpanda broker (external)
});

const producer = kafka.producer();

const produceFromCSV = async (csvPath) => {
  await producer.connect();
  console.log("✅ Kafka Producer connected");

  const csvStream = fs.createReadStream(csvPath).pipe(csv());

  let count = 0;

  for await (const row of csvStream) {
    // Clean + convert numeric fields
    const message = {
      block_time: row.block_time,
      token_address: row.token_address,
      price_in_sol: parseFloat(row.price_in_sol),
      transaction_signature: row.transaction_signature,
    };

    // Validate required fields
    if (!message.token_address || isNaN(message.price_in_sol)) {
      console.warn(`⚠️ Skipping invalid row: ${JSON.stringify(row)}`);
      continue;
    }

    await producer.send({
      topic: "trade-data",
      messages: [
        {
          key: message.token_address,
          value: JSON.stringify(message),
        },
      ],
    });

    count++;
    if (count % 1000 === 0) {
      console.log(`📤 ${count} messages sent...`);
    }
  }

  console.log(`🏁 Finished producing ${count} messages`);
  await producer.disconnect();
};

// CLI entry
const csvPath = process.argv[2] || "./trades_data.csv";
await produceFromCSV(csvPath);
