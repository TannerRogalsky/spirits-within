const path = require("path");
// const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: './index.js',
  output: {
    path: dist,
    filename: '[name].js',
  },
  plugins: [
  	// new CleanWebpackPlugin(),
    new HtmlWebpackPlugin()
  ],
  experiments: {
    asyncWebAssembly: true,
  }
};