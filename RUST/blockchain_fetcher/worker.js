require("dotenv").config();
const { workerData, parentPort } = require("worker_threads");
const Web3 = require("web3");
const { MongoClient } = require("mongodb");

const web3 = new Web3(
  new Web3.providers.HttpProvider(process.env.ETH_NODE_URL)
);
const client = new MongoClient(process.env.MONGODB_URI, {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});

async function fetchAndStoreBlocks(startBlock, endBlock) {
  try {
    await client.connect();
    const db = client.db("ethereum");
    const blocksCollection = db.collection("blocks");
    const transactionsCollection = db.collection("transactions");

    for (let blockNumber = startBlock; blockNumber <= endBlock; blockNumber++) {
      const block = await web3.eth.getBlock(blockNumber, true);

      if (!block) {
        parentPort.postMessage(`Block ${blockNumber} not found.`);
        continue;
      }

      await blocksCollection.insertOne(block);

      if (block.transactions.length > 0) {
        const transactions = block.transactions.map((tx) => ({
          ...tx,
          blockNumber: block.number,
        }));
        await transactionsCollection.insertMany(transactions);
      }

      parentPort.postMessage(
        `Block ${blockNumber} and its transactions stored.`
      );
    }
  } catch (error) {
    parentPort.postMessage(`Error: ${error.message}`);
  } finally {
    await client.close();
    parentPort.close();
  }
}

fetchAndStoreBlocks(workerData.threadStart, workerData.threadEnd);
