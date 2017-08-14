import { combineEpics } from "redux-observable"
import { addSourceEpic, loadUnfollowedSourcesEpic, fallowSourceEpic } from "./sources/SourcesEpics"
import { loadUnreadedFeedsEpic, readFeedEpic, reloadUnreadedFeedsEpic } from "./mindstream/MindStreamEpics"
import { loadfeedsEpic } from "./feeds/FeedsEpics"
import { loginEpic, loginSuccessEpic } from "./login/LoginEpics"

const RootEpic = combineEpics(
    addSourceEpic,
    loadUnfollowedSourcesEpic,
    fallowSourceEpic,

    loadUnreadedFeedsEpic,
    reloadUnreadedFeedsEpic,
    readFeedEpic,

    loadfeedsEpic,

    loginEpic,
    loginSuccessEpic
)

export default RootEpic
