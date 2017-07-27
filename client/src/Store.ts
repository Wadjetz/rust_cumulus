import { createStore, combineReducers } from "redux"
import LoginReducer, { LoginState } from "./login/LoginReducer"

const reducers = combineReducers({
    login: LoginReducer
})

export interface State {
    login: LoginState
}

export let store = createStore(
    reducers,
    (window as any).__REDUX_DEVTOOLS_EXTENSION__ && (window as any).__REDUX_DEVTOOLS_EXTENSION__()
)
