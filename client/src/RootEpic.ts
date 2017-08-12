import { combineEpics } from "redux-observable"
import { addSourceEpic, loadUnfollowedSourcesEpic, fallowSourceEpic } from "./sources/SourcesEpics"
import { loadUnreadedFeedsEpic } from "./mindstream/MindStreamEpics"

const RootEpic = combineEpics(
    addSourceEpic,
    loadUnfollowedSourcesEpic,
    fallowSourceEpic,

    loadUnreadedFeedsEpic,
)

export default RootEpic
