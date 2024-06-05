require("dotenv").config();
const { Worker } = require("worker_threads");

const startBlock = parseInt(process.argv[2], 10);
const endBlock = parseInt(process.argv[3], 10);
const numThreads = parseInt(process.argv[4], 10) || 4; // Default to 4 threads if not provided

if (isNaN(startBlock) || isNaN(endBlock)) {
  console.error("Please provide valid start and end block numbers.");
  process.exit(1);
}

const blocksPerThread = Math.ceil((endBlock - startBlock + 1) / numThreads);

for (let i = 0; i < numThreads; i++) {
  const threadStart = startBlock + i * blocksPerThread;
  const threadEnd = Math.min(endBlock, threadStart + blocksPerThread - 1);

  const worker = new Worker("./worker.js", {
    workerData: {
      threadStart,
      threadEnd,
    },
  });

  worker.on("message", (msg) => {
    console.log(`Thread ${i}: ${msg}`);
  });

  worker.on("error", (err) => {
    console.error(`Thread ${i} error:`, err);
  });

  worker.on("exit", (code) => {
    if (code !== 0) {
      console.error(`Thread ${i} exited with code ${code}`);
    }
  });
}
