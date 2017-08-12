import {
    LOAD_UNREADED_FEEDS, loadUnreadedFeedsSuccess, loadUnreadedFeedsError,
    READ_FEED, readFeedSuccess, readFeedError,
} from "./MindStreamActions"
import { Epic } from "redux-observable"
import { State } from "../Store"
import * as Api from "../Api"

export const loadUnreadedFeedsEpic: Epic<any, State> = (action$, state) => action$.ofType(LOAD_UNREADED_FEEDS)
    .mergeMap(action =>
        Api.loadUnreadedFeeds(state.getState().login.token)
            .then(loadUnreadedFeedsSuccess)
            .catch(loadUnreadedFeedsError)
    )

export const readFeedEpic: Epic<any, State> = (action$, state) => action$.ofType(READ_FEED)
    .mergeMap(action =>
        Api.readFeed(state.getState().login.token, action.feed, action.reaction)
            .then(readFeedSuccess)
            .catch(readFeedError)
    )
