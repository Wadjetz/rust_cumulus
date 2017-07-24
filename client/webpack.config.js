console.log(__dirname + "/../static")

module.exports = {
    entry: "./src/main.tsx",
    output: {
        filename: "main.js",
        path: __dirname + "/../static"
    },
    devtool: "source-map",
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".json"]
    },
    module: {
        rules: [
            { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
            { enforce: "pre", test: /\.js$/, loader: "source-map-loader" }
        ]
    },
    /*
    externals: {
        "react": "React",
        "react-dom": "ReactDOM"
    },
    */
};
