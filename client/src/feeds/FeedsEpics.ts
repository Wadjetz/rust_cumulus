import { Epic } from "redux-observable"

import { loadfeedsError, loadfeedsSuccess, FeedsAction } from "./FeedsActions"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const loadfeedsEpic: Epic<FeedsAction, GlobalState> = (action$) => action$.ofType("LOAD_FEEDS")
    .mergeMap(action =>
        Api.feedsByReaction("Liked")
            .then(loadfeedsSuccess)
            .catch(loadfeedsError)
    )
