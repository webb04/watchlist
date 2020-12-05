module.exports = {
  entry: "./src/scripts/app.tsx",
  output: {
    path: __dirname + "/assets/scripts/",
    filename: "app.js",
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js"],
  },
  module: {
    rules: [{ test: /\.tsx?$/, loader: "ts-loader" }],
  },
};
