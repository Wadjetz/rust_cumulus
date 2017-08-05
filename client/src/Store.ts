import { Reducer } from "redux"
import { RouterState } from "react-router-redux"

import { LoginState } from "./login/LoginReducer"
import { FeedsState } from "./feeds/FeedsReducer"
import { MindStreamState } from "./mindstream/MindStreamReducer"

export interface State {
    login: LoginState
    feeds: FeedsState
    mindStream: MindStreamState
    router: Reducer<RouterState>
}
