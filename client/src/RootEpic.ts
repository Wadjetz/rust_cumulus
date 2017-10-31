import { combineEpics } from "redux-observable"
import {
    addSourceEpic,
    loadUnfollowedSourcesEpic,
    fallowSourceEpic,
    loadMySourcesEpic,
    fallowSourcesSuccessEpic
} from "./sources/SourcesEpics"
import { signupEpic, signupSuccessEpic } from "./signup/SignupEpics"
import { loadUnreadedFeedsEpic, readFeedEpic, reloadUnreadedFeedsEpic, loadUnreadedFeedsBySourceEpic, nextFeedEpic } from "./mindstream/MindStreamEpics"
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
    loadUnreadedFeedsBySourceEpic,
    readFeedEpic,
    nextFeedEpic,

    loadfeedsEpic,

    loginEpic,
    loginSuccessEpic,
    loginErrorEpic,

    signupEpic,
    signupSuccessEpic,
)

export default RootEpic
