import * as React from "react"
import * as ReactDOM from "react-dom"
import { Provider } from "react-redux"
import { store } from "./Store"
import LoginContainer from "./login/LoginContainer"

ReactDOM.render(
    <Provider store={store}>
        <LoginContainer />
    </Provider>,
    document.getElementById("app")
)
