import { Epic } from "redux-observable"

import {
    MindStreamAction, READ_FEED, READ_FEED_SUCCESS, LOAD_UNREADED_FEEDS_BY_SOURCE, NEXT_FEED, loadUnreadedFeedsSuccess, loadUnreadedFeedsError,
    readFeedSuccess, readFeedError, loadUnreadedFeeds, loadUnreadedFeedsBySource
} from "./MindStreamActions"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const loadUnreadedFeedsEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$.ofType("LOAD_UNREADED_FEEDS")
    .mergeMap(action =>
        Api.loadUnreadedFeeds()
            .then(loadUnreadedFeedsSuccess)
            .catch(loadUnreadedFeedsError)
    )

export const loadUnreadedFeedsBySourceEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$.ofType("LOAD_UNREADED_FEEDS_BY_SOURCE")
    .mergeMap((action: LOAD_UNREADED_FEEDS_BY_SOURCE) =>
        Api.loadUnreadedFeedsBySource(action.sourceUuid)
            .then(loadUnreadedFeedsSuccess)
            .catch(loadUnreadedFeedsError)
    )

export const nextFeedEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$.ofType("NEXT_FEED")
    .mergeMap((action: NEXT_FEED) =>
        Api.feedReaction(action.feed, "Readed")
            .then(readFeedSuccess)
            .catch(readFeedError)
    )

export const reloadUnreadedFeedsEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$
    .filter(action => action.type === "READ_FEED_SUCCESS" && state.getState().mindStream.feeds.length === 0)
    .map((action: READ_FEED_SUCCESS) => {
        if (action.sourceUuid) {
            return loadUnreadedFeedsBySource(action.sourceUuid)
        } else {
            return loadUnreadedFeeds()
        }
    })

export const readFeedEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$.ofType("READ_FEED")
    .mergeMap((action: READ_FEED) =>
        Api.feedReaction(action.feed, action.reaction)
            .then(feed => readFeedSuccess(feed, action.sourceUuid))
            .catch(readFeedError)
    )
