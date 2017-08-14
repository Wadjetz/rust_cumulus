import { Reducer } from "redux"
import { RouterState } from "react-router-redux"

import { LoginState } from "./login/LoginReducer"
import { FeedsState } from "./feeds/FeedsReducer"
import { MindStreamState } from "./mindstream/MindStreamReducer"
import { SourcesState } from "./sources/SourcesReducer"
import { AuthState } from "./AuthReducer"

export interface State {
    auth: AuthState
    login: LoginState
    feeds: FeedsState
    mindStream: MindStreamState
    sources: SourcesState
    router: Reducer<RouterState>
}
