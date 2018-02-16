import { Source, SourceStat } from "./Source"

export type SourceAction =
    LOAD_UNFOLLOWED_SOURCES |
    LOAD_UNFOLLOWED_SOURCES_SUCCESS |
    LOAD_UNFOLLOWED_SOURCES_ERROR |
    LOAD_MY_SOURCES |
    LOAD_MY_SOURCES_SUCCESS |
    LOAD_MY_SOURCES_ERROR |
    ADD_MY_SOURCE |
    FALLOW_SOURCE |
    FALLOW_SOURCE_SUCCESS |
    FALLOW_SOURCE_ERROR |
    ADD_SOURCE_ON_CHANGE |
    ADD_SOURCE |
    ADD_SOURCE_SUCCESS |
    ADD_SOURCE_ERROR |
    LOAD_MY_SOURCES_STATS |
    LOAD_MY_SOURCES_STATS_SUCCESS |
    LOAD_MY_SOURCES_STATS_ERROR

export type LOAD_UNFOLLOWED_SOURCES = { type: "LOAD_UNFOLLOWED_SOURCES" }
export function loadUnfollowedSources(): LOAD_UNFOLLOWED_SOURCES {
    return { type: "LOAD_UNFOLLOWED_SOURCES" }
}

export type LOAD_UNFOLLOWED_SOURCES_SUCCESS = { type: "LOAD_UNFOLLOWED_SOURCES_SUCCESS", sources: Source[] }
export function loadUnfollowedSourcesSuccess(sources: Source[]): LOAD_UNFOLLOWED_SOURCES_SUCCESS {
    return { type: "LOAD_UNFOLLOWED_SOURCES_SUCCESS", sources }
}

export type LOAD_UNFOLLOWED_SOURCES_ERROR = { type: "LOAD_UNFOLLOWED_SOURCES_ERROR", error: any }
export function loadUnfollowedSourcesError(error: any): LOAD_UNFOLLOWED_SOURCES_ERROR {
    return { type: "LOAD_UNFOLLOWED_SOURCES_ERROR", error }
}

export type LOAD_MY_SOURCES = { type: "LOAD_MY_SOURCES" }
export function loadMySources(): LOAD_MY_SOURCES {
    return { type: "LOAD_MY_SOURCES" }
}

export type LOAD_MY_SOURCES_SUCCESS = { type: "LOAD_MY_SOURCES_SUCCESS", sources: Source[] }
export function loadMySourcesSuccess(sources: Source[]): LOAD_MY_SOURCES_SUCCESS {
    return { type: "LOAD_MY_SOURCES_SUCCESS", sources }
}

export type LOAD_MY_SOURCES_ERROR = { type: "LOAD_MY_SOURCES_ERROR", error: any }
export function loadMySourcesError(error: any): LOAD_MY_SOURCES_ERROR {
    return { type: "LOAD_MY_SOURCES_ERROR", error }
}

export type LOAD_MY_SOURCES_STATS = { type: "LOAD_MY_SOURCES_STATS" }
export const loadMySourcesStats = (): LOAD_MY_SOURCES_STATS => ({ type: "LOAD_MY_SOURCES_STATS" })

export type LOAD_MY_SOURCES_STATS_SUCCESS = { type: "LOAD_MY_SOURCES_STATS_SUCCESS", stats: SourceStat[] }
export const loadMySourcesStatsSuccess = (stats: SourceStat[]): LOAD_MY_SOURCES_STATS_SUCCESS => ({ type: "LOAD_MY_SOURCES_STATS_SUCCESS", stats })

export type LOAD_MY_SOURCES_STATS_ERROR = { type: "LOAD_MY_SOURCES_STATS_ERROR", error: any }
export const loadMySourcesStatsError = (error: any): LOAD_MY_SOURCES_STATS_ERROR => ({ type: "LOAD_MY_SOURCES_STATS_ERROR", error })

export type ADD_MY_SOURCE = { type: "ADD_MY_SOURCE", source: Source }
export function addMySource(source: Source): ADD_MY_SOURCE {
    return { type: "ADD_MY_SOURCE", source }
}

export type FALLOW_SOURCE = { type: "FALLOW_SOURCE", source: Source }
export function fallowSources(source: Source): FALLOW_SOURCE {
    return { type: "FALLOW_SOURCE", source }
}

export type FALLOW_SOURCE_SUCCESS = { type: "FALLOW_SOURCE_SUCCESS", source: Source }
export function fallowSourcesSuccess(source: Source): FALLOW_SOURCE_SUCCESS {
    return { type: "FALLOW_SOURCE_SUCCESS", source }
}

export type FALLOW_SOURCE_ERROR = { type: "FALLOW_SOURCE_ERROR", error: any }
export function fallowSourcesError(error: any): FALLOW_SOURCE_ERROR {
    return { type: "FALLOW_SOURCE_ERROR", error }
}

export type ADD_SOURCE_ON_CHANGE = {
    type: "ADD_SOURCE_ON_CHANGE"
    field: string,
    value: string
}
export function addSourceOnChange(field: string, value: string): ADD_SOURCE_ON_CHANGE {
    return { type: "ADD_SOURCE_ON_CHANGE", field, value }
}

export type ADD_SOURCE = { type: "ADD_SOURCE", sourceUrl: string }
export function addSource(sourceUrl: string): ADD_SOURCE {
    return { type: "ADD_SOURCE", sourceUrl }
}

export type ADD_SOURCE_SUCCESS = { type: "ADD_SOURCE_SUCCESS", source: Source }
export function addSourceSuccess(source: Source): ADD_SOURCE_SUCCESS {
    return { type: "ADD_SOURCE_SUCCESS", source }
}

export type ADD_SOURCE_ERROR = { type: "ADD_SOURCE_ERROR", error: any }
export function addSourceError(error: any): ADD_SOURCE_ERROR {
    return { type: "ADD_SOURCE_ERROR", error }
}
