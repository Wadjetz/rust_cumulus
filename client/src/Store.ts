import { LoginState } from "./login/LoginReducer"
import { Reducer } from "redux"
import { RouterState } from "react-router-redux"

export interface State {
    login: LoginState
    router: Reducer<RouterState>
}
