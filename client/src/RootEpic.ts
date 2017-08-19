import { combineEpics } from "redux-observable"
import {
    addSourceEpic,
    loadUnfollowedSourcesEpic,
    fallowSourceEpic,
    loadMySourcesEpic,
    fallowSourcesSuccessEpic
} from "./sources/SourcesEpics"
import { loadUnreadedFeedsEpic, readFeedEpic, reloadUnreadedFeedsEpic } from "./mindstream/MindStreamEpics"
import { loadfeedsEpic } from "./feeds/FeedsEpics"
import { loginEpic, loginSuccessEpic, loginErrorEpic } from "./login/LoginEpics"

const RootEpic = combineEpics(
    addSourceEpic,
    loadUnfollowedSourcesEpic,
    loadMySourcesEpic,
    fallowSourceEpic,
    fallowSourcesSuccessEpic,

    loadUnreadedFeedsEpic,
    reloadUnreadedFeedsEpic,
    readFeedEpic,

    loadfeedsEpic,

    loginEpic,
    loginSuccessEpic,
    loginErrorEpic
)

export default RootEpic
