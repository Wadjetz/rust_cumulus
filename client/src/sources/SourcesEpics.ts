import { Action } from "redux"
import {
    ADD_SOURCE_ON_LOAD, addSourceOnLoadSuccess, addSourceOnLoadError,
    SOURCES_ON_LOAD, sourcesOnLoadSuccess, sourcesOnLoadError,
} from "./SourcesActions"
import { ActionsObservable, Epic } from "redux-observable"
import { State } from "../Store"
import * as Api from "../Api"

export const addSourceEpic: Epic<any, State> = (action$) => action$.ofType(ADD_SOURCE_ON_LOAD)
    .mergeMap(action =>
        Api.addSource(action.sourceUrl)
            .then(addSourceOnLoadSuccess)
            .catch(addSourceOnLoadError)
    )

export const loadUnfollowedSourcesEpic: Epic<any, State> = (action$, state) => action$.ofType(SOURCES_ON_LOAD)
    .mergeMap(action =>
        Api.loadUnfollowedSources(state.getState().login.token)
            .then(sourcesOnLoadSuccess)
            .catch(sourcesOnLoadError)
    )
