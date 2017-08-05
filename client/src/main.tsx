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
import MindStreamContainer from "./mindstream/MindStreamContainer"
import MindStreamReducer from "./mindstream/MindStreamReducer"
import SourcesContainer from "./sources/SourcesContainer"
import SourcesReducer from "./sources/SourcesReducer"
import Header from "./components/Header"

const middleware = routerMiddleware(history)

const reducers = combineReducers({
    login: LoginReducer,
    feeds: FeedsReducer,
    mindStream: MindStreamReducer,
    sources: SourcesReducer,
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
                <Route exact path="/" component={MindStreamContainer}/>
                <Route exact path="/feeds" component={FeedsContainer}/>
                <Route exact path="/sources" component={SourcesContainer}/>
                <Route exact path="/login" component={LoginContainer}/>
            </div>
        </ConnectedRouter>
    </Provider>,
    document.getElementById("app")
)
