import { combineEpics } from "redux-observable"
import * as SourcesEpics from "./sources/SourcesEpics"
import { signupEpic, signupSuccessEpic } from "./signup/SignupEpics"
import * as MindStreamEpics from "./mindstream/MindStreamEpics"
import { loadfeedsEpic } from "./feeds/FeedsEpics"
import { loginEpic, loginSuccessEpic, loginErrorEpic } from "./login/LoginEpics"

const RootEpic = combineEpics(
    SourcesEpics.addSourceEpic,
    SourcesEpics.loadUnfollowedSourcesEpic,
    SourcesEpics.loadMySourcesEpic,
    SourcesEpics.loadMySourcesStatsEpic,
    SourcesEpics.fallowSourceEpic,
    SourcesEpics.fallowSourcesSuccessEpic,
    MindStreamEpics.loadUnreadedFeedsEpic,
    MindStreamEpics.reloadUnreadedFeedsEpic,
    MindStreamEpics.loadUnreadedFeedsBySourceEpic,
    MindStreamEpics.readFeedEpic,
    MindStreamEpics.nextFeedEpic,

    loadfeedsEpic,

    loginEpic,
    loginSuccessEpic,
    loginErrorEpic,

    signupEpic,
    signupSuccessEpic,
)

export default RootEpic
