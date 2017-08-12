import { combineEpics } from "redux-observable"
import { addSourceEpic, loadUnfollowedSourcesEpic } from "./sources/SourcesEpics"

const RootEpic = combineEpics(
    addSourceEpic,
    loadUnfollowedSourcesEpic,
)

export default RootEpic
