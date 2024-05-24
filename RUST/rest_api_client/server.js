const express = require("express");
const app = express();
const port = 3000;

app.use(express.json());

app.use((req, res, next) => {
  console.log(`${req.method} ${req.url}`);
  console.log("Body:", req.body);
  console.log("Headers:", req.headers);
  next();
});

// Create
app.post("/api/create", (req, res) => {
  res.send("Create endpoint");
});

// Read
app.get("/api/read", (req, res) => {
  res.send("Read endpoint");
});

// Update
app.put("/api/update", (req, res) => {
  res.send("Update endpoint");
});

// Delete
app.delete("/api/delete", (req, res) => {
  res.send("Delete endpoint");
});

app.listen(port, () => {
  console.log(`Server is listening on port ${port}`);
});
