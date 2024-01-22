onmessage = (e) => {
  console.log("Message received from main script");
  const workerResult = `Result: ${e.data[0] * e.data[1]}`;
  console.log("Posting message back to main script");
  postMessage(workerResult);
};

console.time("Fetch_And_Parse_51MB");
console.log(performance.now());
fetch("../distribution.json")
  .then((data) => data.json())
  .then((data) => {
    console.log(performance.now());
    console.log(data.length);
    console.log(data);
    console.timeEnd("Fetch_And_Parse_51MB");
    postMessage(data);
  });
