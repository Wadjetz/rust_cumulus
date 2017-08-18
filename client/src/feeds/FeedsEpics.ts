import { Epic } from "redux-observable"

import {
    LOAD_FEEDS, loadfeedsError, loadfeedsSuccess
} from "./FeedsActions"
import { State } from "../Store"
import * as Api from "../Api"

export const loadfeedsEpic: Epic<any, State> = (action$) => action$.ofType(LOAD_FEEDS)
    .mergeMap(action =>
        Api.feedsByReaction("Liked")
            .then(loadfeedsSuccess)
            .catch(loadfeedsError)
    )
