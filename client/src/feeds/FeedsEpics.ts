import { Epic } from "redux-observable"

import {
    LOAD_FEEDS, loadfeedsError, loadfeedsSuccess
} from "./FeedsActions"
import { State } from "../Store"
import * as Api from "../Api"

export const loadfeedsEpic: Epic<any, State> = (action$, state) => action$.ofType(LOAD_FEEDS)
    .mergeMap(action =>
        Api.loadUnreadedFeeds(state.getState().auth.token)
            .then(loadfeedsSuccess)
            .catch(loadfeedsError)
    )
