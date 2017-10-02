const path = require("path")
const ExtractTextPlugin = require('extract-text-webpack-plugin')
const webpack = require("webpack")

const development = process.env.NODE_ENV === "development";

var config = {
    entry: "./src/main.tsx",
    output: {
        filename: "main.js",
        path: __dirname + "/../static"
    },
    devtool: "source-map",
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".json"],
        modules: [path.resolve(__dirname, './src'), 'node_modules']
    },
    module: {
        rules: [
            { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
            { enforce: "pre", test: /\.js$/, loader: "source-map-loader" },
            {
                test: /\.css$/,
                use: ExtractTextPlugin.extract({
                    fallback: 'style-loader',
                    use: [
                        {
                            loader: 'typings-for-css-modules-loader',
                            options: {
                                modules: true,
                                namedExport: true,
                                camelCase: true
                            }
                        },
                        /*
                        {
                            loader: 'postcss-loader',
                            options: {
                                plugins: postcssPlugins,
                                importLoaders: 1
                            }
                        }
                        */
                    ]
                }),
                exclude: /(node_modules|\.global\.css)/
            }
        ]
    },
    plugins: [
        new ExtractTextPlugin({
            filename: 'main.css',
            allChunks: true
        }),
        new webpack.WatchIgnorePlugin([
            /css\.d\.ts$/
        ]),
    ]
    /*
    externals: {
        "react": "React",
        "react-dom": "ReactDOM"
    },
    */
};

if (!development) {
    config.plugins.push(
        new webpack.DefinePlugin({
            'process.env': {
              NODE_ENV: JSON.stringify('production')
            }
        }),
    ),
    config.plugins.push(new webpack.optimize.UglifyJsPlugin({
        compressor: {screw_ie8: true, keep_fnames: false, warnings: false},
        sourceMap: true,
        mangle: {screw_ie8: true, keep_fnames: false}
    }));
}

module.exports = config
