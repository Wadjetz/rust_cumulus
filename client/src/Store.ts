import { Reducer } from "redux"
import { RouterState } from "react-router-redux"

import { LoginState } from "./login/LoginReducer"
import { FeedsState } from "./feeds/FeedsReducer"

export interface State {
    login: LoginState
    feeds: FeedsState
    router: Reducer<RouterState>
}
