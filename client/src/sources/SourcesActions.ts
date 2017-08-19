import { Source } from "./Source"

export const LOAD_UNFOLLOWED_SOURCES = "LOAD_UNFOLLOWED_SOURCES"
export const loadUnfollowedSources = () => ({ type: LOAD_UNFOLLOWED_SOURCES })

export const LOAD_UNFOLLOWED_SOURCES_SUCCESS = "LOAD_UNFOLLOWED_SOURCES_SUCCESS"
export const loadUnfollowedSourcesSuccess = (sources: Source[]) => ({ type: LOAD_UNFOLLOWED_SOURCES_SUCCESS, sources })

export const LOAD_UNFOLLOWED_SOURCES_ERROR = "LOAD_UNFOLLOWED_SOURCES_ERROR"
export const loadUnfollowedSourcesError = (error: any) => ({ type: LOAD_UNFOLLOWED_SOURCES_ERROR, error })

export const LOAD_MY_SOURCES = "LOAD_MY_SOURCES"
export const loadMySources = () => ({ type: LOAD_MY_SOURCES })

export const LOAD_MY_SOURCES_SUCCESS = "LOAD_MY_SOURCES_SUCCESS"
export const loadMySourcesSuccess = (sources: Source[]) => ({ type: LOAD_MY_SOURCES_SUCCESS, sources })

export const LOAD_MY_SOURCES_ERROR = "LOAD_MY_SOURCES_ERROR"
export const loadMySourcesError = (error: any) => ({ type: LOAD_MY_SOURCES_ERROR, error })

export const ADD_MY_SOURCE = "ADD_MY_SOURCE"
export const addMySource = (source: Source) => ({ type: ADD_MY_SOURCE, source })

export const FALLOW_SOURCE = "FALLOW_SOURCE"
export const fallowSources = (source: Source) => ({ type: FALLOW_SOURCE, source })

export const FALLOW_SOURCE_SUCCESS = "FALLOW_SOURCE_SUCCESS"
export const fallowSourcesSuccess = (source: Source) => ({ type: FALLOW_SOURCE_SUCCESS, source })

export const FALLOW_SOURCE_ERROR = "FALLOW_SOURCE_ERROR"
export const fallowSourcesError = (error: any) => ({ type: FALLOW_SOURCE_ERROR, error })

export const ADD_SOURCE_ON_CHANGE = "ADD_SOURCE_ON_CHANGE"
export const addSourceOnChange = (field: string, value: string) => ({ type: ADD_SOURCE_ON_CHANGE, field, value })

export const ADD_SOURCE = "ADD_SOURCE"
export const addSource = (sourceUrl: string) => ({ type: ADD_SOURCE, sourceUrl })

export const ADD_SOURCE_SUCCESS = "ADD_SOURCE_SUCCESS"
export const addSourceSuccess = (source: Source) => ({ type: ADD_SOURCE_SUCCESS, source })

export const ADD_SOURCE_ERROR = "ADD_SOURCE_ERROR"
export const addSourceError = (error: any) => ({ type: ADD_SOURCE_ERROR, error })
