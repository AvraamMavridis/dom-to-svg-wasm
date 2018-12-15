const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
    entry: [
        './bootstrap.js'
    ],
    output: {
        path: __dirname + '/dist',
        filename: "bootstrap.js"
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
            }
        ]
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ]
};
