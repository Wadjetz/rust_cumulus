import { Epic } from "redux-observable"

import { loadfeedsError, loadfeedsSuccess } from "./FeedsActions"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const loadfeedsEpic: Epic<any, GlobalState> = (action$) => action$.ofType("LOAD_FEEDS")
    .mergeMap(action =>
        Api.feedsByReaction("Liked")
            .then(loadfeedsSuccess)
            .catch(loadfeedsError)
    )
