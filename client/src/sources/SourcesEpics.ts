import { Action } from "redux"
import {
    ADD_SOURCE, addSourceSuccess, addSourceError,
    LOAD_UNFOLLOWED_SOURCES, loadUnfollowedSourcesSuccess, loadUnfollowedSourcesError,
    FALLOW_SOURCE, fallowSourcesSuccess, fallowSourcesError
} from "./SourcesActions"
import { ActionsObservable, Epic } from "redux-observable"
import { State } from "../Store"
import * as Api from "../Api"

export const addSourceEpic: Epic<any, State> = (action$) => action$.ofType(ADD_SOURCE)
    .mergeMap(action =>
        Api.addSource(action.sourceUrl)
            .then(addSourceSuccess)
            .catch(addSourceError)
    )

export const loadUnfollowedSourcesEpic: Epic<any, State> = (action$, state) => action$.ofType(LOAD_UNFOLLOWED_SOURCES)
    .mergeMap(action =>
        Api.loadUnfollowedSources(state.getState().login.token)
            .then(loadUnfollowedSourcesSuccess)
            .catch(loadUnfollowedSourcesError)
    )

export const fallowSourceEpic: Epic<any, State> = (action$, state) => action$.ofType(FALLOW_SOURCE)
    .mergeMap(action =>
        Api.fallowSource(state.getState().login.token, action.source)
            .then(fallowSourcesSuccess)
            .catch(fallowSourcesError)
    )
