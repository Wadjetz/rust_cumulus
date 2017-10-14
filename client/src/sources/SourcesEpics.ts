import * as SourcesActions from "./SourcesActions"
import { Epic } from "redux-observable"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const addSourceEpic: Epic<any, GlobalState> = (action$) => action$.ofType("ADD_SOURCE")
    .mergeMap(action =>
        Api.addSource(action.sourceUrl)
            .then(SourcesActions.addSourceSuccess)
            .catch(SourcesActions.addSourceError)
    )

export const loadUnfollowedSourcesEpic: Epic<any, GlobalState> = (action$, state) => action$.ofType("LOAD_UNFOLLOWED_SOURCES")
    .mergeMap(action =>
        Api.loadUnfollowedSources()
            .then(SourcesActions.loadUnfollowedSourcesSuccess)
            .catch(SourcesActions.loadUnfollowedSourcesError)
    )

export const loadMySourcesEpic: Epic<any, GlobalState> = (action$, state) => action$.ofType("LOAD_MY_SOURCES")
    .mergeMap((action: SourcesActions.LOAD_MY_SOURCES) =>
        Api.loadMySources()
            .then(result => SourcesActions.loadMySourcesSuccess(result.mySources, result.sourcesStats))
            .catch(SourcesActions.loadMySourcesError)
    )

export const fallowSourceEpic: Epic<any, GlobalState> = (action$, state) => action$.ofType("FALLOW_SOURCE")
    .mergeMap(action =>
        Api.fallowSource(action.source)
            .then(SourcesActions.fallowSourcesSuccess)
            .catch(SourcesActions.fallowSourcesError)
    )

export const fallowSourcesSuccessEpic: Epic<any, GlobalState> = (action$) => action$.ofType("FALLOW_SOURCE_SUCCESS")
    .map((action: SourcesActions.FALLOW_SOURCE_SUCCESS) => SourcesActions.addMySource(action.source))
