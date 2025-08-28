import HtmlWebpackPlugin from "html-webpack-plugin";
import CssMinimizerPlugin from "css-minimizer-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import webpack from "webpack";

import { fileURLToPath } from 'url';
import path from "node:path";


const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default {
  entry: {
    app: "./src/index.js",
  },
  output: {
    path: path.resolve("./dist"),
    publicPath: path.resolve(__dirname, "/static/"),
    filename: "[name].bundle.js",
    clean: true,
  },
  devtool: "source-map",
  optimization: {
    minimize: true,
    runtimeChunk: {
      name: "runtime",
    },
    splitChunks: {
      chunks: "async",
      minSize: 20000,
      minRemainingSize: 0,
      minChunks: 1,
      maxAsyncRequests: 30,
      maxInitialRequests: 30,
      enforceSizeThreshold: 50000,
      cacheGroups: {
        defaultVendors: {
          test: /[\\/]node_modules[\\/]/,
          priority: -10,
          reuseExistingChunk: true,
        },
        default: {
          minChunks: 2,
          priority: -20,
          reuseExistingChunk: true,
        },
      },
    },
  },
  devServer: {
    hot: true,
    open: true,
    historyApiFallback: {
      index: "index.html",
    },
  },
  module: {
    rules: [
      {
        test: /\.riot$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "@riotjs/webpack-loader",
            options: {
              hot: true,
            },
          },
        ],
      },
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
      },
      {
        test: /\.(eot|svg|ttf|woff|woff2|png|jpg|gif|ico)$/i,
        type: 'asset',
        generator: {
          publicPath: '/static/',
          filename: '[name][ext][query]'
        },
      },      
      {
        test: /\.html$/i,
        use: ['html-loader'],
      }
    ],
  },
  optimization: {
    minimizer: [
      new CssMinimizerPlugin()
    ]
  },
  plugins: [
    new MiniCssExtractPlugin(),
    new HtmlWebpackPlugin({
      template: "src/index.html",
    }),
    new webpack.HotModuleReplacementPlugin(),
  ],
};
