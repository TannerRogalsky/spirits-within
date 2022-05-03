const path = require("path");
const HtmlWebpackPlugin = require('html-webpack-plugin');

const dist = path.resolve(__dirname, "..", "dist");

module.exports = {
  entry: './index.js',
  output: {
    path: dist,
    filename: '[name].js',
  },
  plugins: [
    new HtmlWebpackPlugin({
        title: "Spirits Within"
    })
  ],
  experiments: {
    asyncWebAssembly: true,
  }
};