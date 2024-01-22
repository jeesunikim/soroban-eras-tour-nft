const path = require("path");

module.exports = {
  mode: "development",
  entry: "./frontend/src/index.ts",
  devServer: {
    static: "./frontend/dist",
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  devtool: "inline-source-map",
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "frontend/dist/js"),
    publicPath: "/",
    clean: true,
  },
};
