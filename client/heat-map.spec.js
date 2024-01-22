export const spec = {
  $schema: "https://vega.github.io/schema/vega/v5.json",
  description:
    "A heatmap showing average daily temperatures in Seattle for each hour of the day.",
  width: 1000,
  height: 500,
  padding: 5,

  signals: [],

  data: [
    {
      name: "data0",
      url: "./group_by_result.json",
      format: { type: "json" },
      transform: [
        // { type: "formula", as: "hour", expr: "hours(datum.date)" },
        // {
        //   type: "formula",
        //   as: "day",
        //   expr: "datetime(year(datum.date), month(datum.date), date(datum.date))",
        // },
      ],
    },
  ],

  scales: [
    {
      name: "x",
      type: "band",
      domain: { data: "data0", field: "Place" },
      range: "width",
    },
    {
      name: "y",
      type: "band",
      domain: { data: "data0", field: "StoreId" },
      range: "height",
    },
    {
      name: "color",
      range: { scheme: "purples" },
      domain: { data: "data0", field: "count" },
      zero: false,
      nice: true,
    },
  ],

  axes: [
    {
      orient: "bottom",
      scale: "x",
      domain: false,
      title: "Place",
    },
    {
      orient: "left",
      scale: "y",
      domain: false,
      title: "StoreId",
    },
  ],

  marks: [
    {
      type: "rect",
      from: { data: "data0" },
      encode: {
        enter: {
          x: { scale: "x", field: "Place" },
          y: { scale: "y", field: "StoreId" },
          width: { value: 5 },
          height: { scale: "y", band: 1 },
          tooltip: {
            signal: "datum",
          },
        },
        update: {
          fill: { scale: "color", field: "count" },
        },
      },
    },
  ],
};
