import {
    ADD_SOURCE, addSourceSuccess, addSourceError,
    LOAD_UNFOLLOWED_SOURCES, loadUnfollowedSourcesSuccess, loadUnfollowedSourcesError,
    LOAD_MY_SOURCES, loadMySourcesSuccess, loadMySourcesError,
    FALLOW_SOURCE, FALLOW_SOURCE_SUCCESS, fallowSourcesSuccess, fallowSourcesError,
    addMySource
} from "./SourcesActions"
import { Epic } from "redux-observable"
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
        Api.loadUnfollowedSources()
            .then(loadUnfollowedSourcesSuccess)
            .catch(loadUnfollowedSourcesError)
    )

export const loadMySourcesEpic: Epic<any, State> = (action$, state) => action$.ofType(LOAD_MY_SOURCES)
    .mergeMap(action =>
        Api.loadMySources()
            .then(loadMySourcesSuccess)
            .catch(loadMySourcesError)
    )

export const fallowSourceEpic: Epic<any, State> = (action$, state) => action$.ofType(FALLOW_SOURCE)
    .mergeMap(action =>
        Api.fallowSource(action.source)
            .then(fallowSourcesSuccess)
            .catch(fallowSourcesError)
    )

export const fallowSourcesSuccessEpic: Epic<any, State> = (action$) => action$.ofType(FALLOW_SOURCE_SUCCESS)
    .map(action => addMySource(action.source))
