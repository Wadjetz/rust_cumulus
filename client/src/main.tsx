import "es6-promise/auto"
import "es6-shim"
import * as React from "react"
import * as ReactDOM from "react-dom"
import { Route } from "react-router"
import { Provider } from "react-redux"
import { createStore, combineReducers, applyMiddleware } from "redux"
import { ConnectedRouter, routerReducer, routerMiddleware } from "react-router-redux"
import { composeWithDevTools } from "redux-devtools-extension"
import { createEpicMiddleware } from "redux-observable"
import { persistStore, autoRehydrate } from "redux-persist"
import { history } from "./router"

import "rxjs/add/operator/map"
import "rxjs/add/operator/mergeMap"
import "rxjs/add/operator/filter"
import "rxjs/add/observable/of"

import AppReducer from "./app/AppReducer"
import LoginReducer from "./login/LoginReducer"
import SignupReducer from "./signup/SignupReducer"
import FeedsReducer from "./feeds/FeedsReducer"
import LoginContainer from "./login/LoginContainer"
import SignupContainer from "./signup/SignupContainer"
import FeedsContainer from "./feeds/FeedsContainer"
import MindStreamContainer from "./mindstream/MindStreamContainer"
import MindStreamReducer from "./mindstream/MindStreamReducer"
import SourcesContainer from "./sources/SourcesContainer"
import SourcesReducer from "./sources/SourcesReducer"

import RootEpic from "./RootEpic"

const epicMiddleware = createEpicMiddleware(RootEpic)

const middleware = routerMiddleware(history)

const reducers = combineReducers({
    app: AppReducer as any,
    login: LoginReducer,
    signup: SignupReducer,
    feeds: FeedsReducer,
    mindStream: MindStreamReducer,
    sources: SourcesReducer,
    router: routerReducer
})

const enhancer = composeWithDevTools(
    applyMiddleware(middleware),
    applyMiddleware(epicMiddleware),
    autoRehydrate(),
)
export const store = createStore(reducers, enhancer)

persistStore(store, { blacklist: ["login", "feeds", "mindStream", "sources"] })

ReactDOM.render(
    <Provider store={store}>
        <ConnectedRouter history={history}>
            <div>
                <Route exact path="/" component={MindStreamContainer}/>
                <Route exact path="/stream/:sourceUuid" component={MindStreamContainer}/>
                <Route exact path="/feeds" component={FeedsContainer}/>
                <Route exact path="/sources" component={SourcesContainer}/>
                <Route exact path="/login" component={LoginContainer}/>
                <Route exact path="/signup" component={SignupContainer}/>
            </div>
        </ConnectedRouter>
    </Provider>,
    document.getElementById("app")
)
