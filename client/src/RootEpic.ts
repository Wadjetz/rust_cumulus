import { combineEpics } from "redux-observable"
import { addSourceEpic, loadUnfollowedSourcesEpic, fallowSourceEpic } from "./sources/SourcesEpics"
import { loadUnreadedFeedsEpic, readFeedEpic } from "./mindstream/MindStreamEpics"

const RootEpic = combineEpics(
    addSourceEpic,
    loadUnfollowedSourcesEpic,
    fallowSourceEpic,

    loadUnreadedFeedsEpic,
    readFeedEpic,
)

export default RootEpic
