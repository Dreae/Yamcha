const webpack = require('webpack');
const path = require('path');

const paths = {
  base: __dirname,
  build: path.join(__dirname, 'build'),
  node_modules: path.join(__dirname, 'node_modules'),
  src: path.join(__dirname, 'src'),
}

module.exports = {
  entry: './yamcha-ui/index.tsx',
  output: {
    filename: 'index.js',
    path: path.join(__dirname, '/uiBuild'),
  },
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'tslint-loader',
        exclude: /node_modules/,
        enforce: 'pre',
      },
      {
        test: /\.tsx?$/,
        loaders: [
          'ts-loader'
        ]
      },
      {
        test: /\.css$/,
        loaders: [
          'style-loader',
          'css-loader?sourceMap=true&importLoaders=1'
        ]
      },
      {
        test: /\.(scss|sass)$/,
        loaders: [
          'style-loader',
          'css-loader',
          'sass-loader'
        ]
      },
      {
        test: /\.(woff2?|eot|ttf)/,
        loaders: [
          'file-loader'
        ]
      },
      {
        test: /\.(png|jpe?g|gif)/,
        loaders: [
          'file-loader?name=[name].[ext]'
        ]
      }
    ]
  },
  resolve: {
    extensions: ['.js', '.ts', '.tsx'],
    modules: [
      paths.src,
      paths.node_modules,
    ],
  },
  externals: {
    jquery: 'jQuery',
    jQuery: 'jQuery',
    moment: 'moment',
  },
  devServer: {
    historyApiFallback: {
      index: 'index.html'
    },
    contentBase: './uiBuild/',
  }
};