import { combineEpics } from "redux-observable"
import { addSourceEpic, loadUnfollowedSourcesEpic, fallowSourceEpic } from "./sources/SourcesEpics"
import { loadUnreadedFeedsEpic, readFeedEpic, reloadUnreadedFeedsEpic } from "./mindstream/MindStreamEpics"
import { loadfeedsEpic } from "./feeds/FeedsEpics"

const RootEpic = combineEpics(
    addSourceEpic,
    loadUnfollowedSourcesEpic,
    fallowSourceEpic,

    loadUnreadedFeedsEpic,
    reloadUnreadedFeedsEpic,
    readFeedEpic,

    loadfeedsEpic,
)

export default RootEpic
