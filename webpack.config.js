const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  mode: 'development',
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: 'index.html' },
        { from: 'pkg/', to: 'pkg/' },
      ],
    }),
  ],
};