import * as React from "react"
import * as ReactDOM from "react-dom"
import { Provider } from "react-redux"
import { createStore, compose, combineReducers, applyMiddleware } from "redux"
import { ConnectedRouter, routerReducer, routerMiddleware, push } from "react-router-redux"
import { Route } from "react-router"
import { history } from "./router"

import LoginReducer from "./login/LoginReducer"
import FeedsReducer from "./feeds/FeedsReducer"
import LoginContainer from "./login/LoginContainer"
import FeedsContainer from "./feeds/FeedsContainer"
import Header from "./components/Header"

const middleware = routerMiddleware(history)

const reducers = combineReducers({
    login: LoginReducer,
    feeds: FeedsReducer,
    router: routerReducer
})

export let store = createStore(
    reducers,
    compose(
        applyMiddleware(middleware),
        (window as any).__REDUX_DEVTOOLS_EXTENSION__ && (window as any).__REDUX_DEVTOOLS_EXTENSION__()
    )
)

ReactDOM.render(
    <Provider store={store}>
        <ConnectedRouter history={history}>
            <div>
                <Header />
                <Route exact path="/" component={FeedsContainer}/>
                <Route path="/login" component={LoginContainer}/>
            </div>
        </ConnectedRouter>
    </Provider>,
    document.getElementById("app")
)
