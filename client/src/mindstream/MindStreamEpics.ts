import { Action } from "redux"
import { Epic } from "redux-observable"

import {
    LOAD_UNREADED_FEEDS, loadUnreadedFeedsSuccess, loadUnreadedFeedsError,
    READ_FEED, READ_FEED_SUCCESS, readFeedSuccess, readFeedError,
} from "./MindStreamActions"
import { State } from "../Store"
import * as Api from "../Api"

export const loadUnreadedFeedsEpic: Epic<any, State> = (action$, state) => action$.ofType(LOAD_UNREADED_FEEDS)
    .mergeMap(action =>
        Api.loadUnreadedFeeds()
            .then(loadUnreadedFeedsSuccess)
            .catch(loadUnreadedFeedsError)
    )

export const reloadUnreadedFeedsEpic: Epic<Action, State> = (action$, state) => action$
    .filter(action => action.type === READ_FEED_SUCCESS && state.getState().mindStream.feeds.length === 0)
    .mergeMap(action =>
        Api.loadUnreadedFeeds()
            .then(loadUnreadedFeedsSuccess)
            .catch(loadUnreadedFeedsError)
    )

export const readFeedEpic: Epic<any, State> = (action$, state) => action$.ofType(READ_FEED)
    .mergeMap(action =>
        Api.feedReaction(action.feed, action.reaction)
            .then(readFeedSuccess)
            .catch(readFeedError)
    )
