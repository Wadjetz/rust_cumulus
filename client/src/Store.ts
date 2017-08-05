import { Reducer } from "redux"
import { RouterState } from "react-router-redux"

import { LoginState } from "./login/LoginReducer"
import { FeedsState } from "./feeds/FeedsReducer"
import { MindStreamState } from "./mindstream/MindStreamReducer"
import { SourcesState } from "./sources/SourcesReducer"

export interface State {
    login: LoginState
    feeds: FeedsState
    mindStream: MindStreamState
    sources: SourcesState
    router: Reducer<RouterState>
}
