import { Epic } from "redux-observable"
import { Observable } from "rxjs/Observable"

import * as MindStreamActions from "./MindStreamActions"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const loadUnreadedFeedsEpic: Epic<MindStreamActions.MindStreamAction, GlobalState> = (action$, state) => action$.ofType("LOAD_UNREADED_FEEDS")
    .mergeMap(action =>
        Api.loadUnreadedFeeds()
            .then(MindStreamActions.loadUnreadedFeedsSuccess)
            .catch(MindStreamActions.mindStreamApiError)
    )

export const loadUnreadedFeedsBySourceEpic: Epic<MindStreamActions.MindStreamAction, GlobalState> = (action$, state) => {
    return action$
        .ofType<MindStreamActions.LOAD_UNREADED_FEEDS_BY_SOURCE>("LOAD_UNREADED_FEEDS_BY_SOURCE")
        .mergeMap(action =>
            Api.loadUnreadedFeedsBySource(action.sourceUuid)
                .then(MindStreamActions.loadUnreadedFeedsSuccess)
                .catch(MindStreamActions.mindStreamApiError)
        )
}

export const nextFeedEpic: Epic<MindStreamActions.MindStreamAction, GlobalState> = (action$, state) => {
    return action$
        .ofType<MindStreamActions.NEXT_FEED>("NEXT_FEED")
        .mergeMap(action =>
            Api.feedReaction(action.feed, "Readed")
                .then(feed => MindStreamActions.nextFeedSuccess(feed, action.sourceUuid))
                .catch(MindStreamActions.mindStreamApiError)
        )
}

export const reloadUnreadedFeedsEpic: Epic<MindStreamActions.MindStreamAction, GlobalState> = (action$, state) => {
    return action$
        .ofType<MindStreamActions.NEXT_FEED_SUCCESS>("NEXT_FEED_SUCCESS")
        .mergeMap(action => {
            if (state.getState().mindStream.feeds.length === 1) {
                return Observable.of<MindStreamActions.MindStreamAction>(
                    (action.sourceUuid)
                    ? MindStreamActions.loadUnreadedFeedsBySource(action.sourceUuid)
                    : MindStreamActions.loadUnreadedFeeds(),
                    MindStreamActions.goToNextFeed()
                )
            } else {
                return Observable.of<MindStreamActions.MindStreamAction>(
                    MindStreamActions.goToNextFeed()
                )
            }
        })
}

export const readFeedEpic: Epic<MindStreamActions.MindStreamAction, GlobalState> = (action$, state) => {
    return action$
        .ofType<MindStreamActions.READ_FEED>("READ_FEED")
        .mergeMap(action =>
            Api.feedReaction(action.feed, action.reaction)
                .then(feed => MindStreamActions.nextFeedSuccess(feed, action.sourceUuid))
                .catch(MindStreamActions.mindStreamApiError)
        )
}
