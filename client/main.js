import { spec } from "./heat-map.spec.js";
import { spec as minSpec } from "./heat-map-min.spec.js";
const myWorker = new Worker("./workers/worker.js");

console.time("initiated worker");
myWorker.onmessage = (e) => {
  const data = e.data;
  console.log(data);
  console.timeEnd("initiated worker");
};

// const result = await vegaEmbed("#vis", spec);
// const result = await vegaEmbed("#vis", minSpec);

// console.log(result.view);
