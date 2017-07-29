const path = require("path")
const ExtractTextPlugin = require('extract-text-webpack-plugin')

module.exports = {
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
