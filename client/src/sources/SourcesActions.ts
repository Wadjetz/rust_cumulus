import { Source } from "./Source"

export const SOURCES_ON_LOAD = "SOURCES_ON_LOAD"
export function sourcesOnLoad() {
    return { type: SOURCES_ON_LOAD }
}

export const SOURCES_ON_LOAD_SUCCESS = "SOURCES_ON_LOAD_SUCCESS"
export function sourcesOnLoadSuccess(sources: Source[]) {
    return {
        type: SOURCES_ON_LOAD_SUCCESS,
        sources
    }
}

export const SOURCES_ON_LOAD_ERROR = "SOURCES_ON_LOAD_ERROR"
export function sourcesOnLoadError(error: any) {
    return {
        type: SOURCES_ON_LOAD_ERROR,
        error
    }
}

export const FALLOW_SOURCE_ON_LOAD = "FALLOW_SOURCE_ON_LOAD"
export function fallowSourcesOnLoad() {
    return { type: FALLOW_SOURCE_ON_LOAD }
}

export const FALLOW_SOURCE_ON_LOAD_SUCCESS = "FALLOW_SOURCE_ON_LOAD_SUCCESS"
export function fallowSourcesOnLoadSuccess(source: Source) {
    return { type: FALLOW_SOURCE_ON_LOAD_SUCCESS, source }
}

export const FALLOW_SOURCE_ON_LOAD_ERROR = "FALLOW_SOURCE_ON_LOAD_ERROR"
export function fallowSourcesOnLoadError(error: any) {
    return {
        type: FALLOW_SOURCE_ON_LOAD_ERROR,
        error
    }
}

export const ADD_SOURCE_ON_CHANGE = "ADD_SOURCE_ON_CHANGE"
export function addSourceOnChange(field: string, value: string) {
    return {
        type: ADD_SOURCE_ON_CHANGE,
        field,
        value
    }
}

export const ADD_SOURCE_ON_LOAD = "ADD_SOURCE_ON_LOAD"
export function addSourceOnLoad() {
    return { type: ADD_SOURCE_ON_LOAD }
}

export const ADD_SOURCE_ON_LOAD_SUCCESS = "ADD_SOURCE_ON_LOAD_SUCCESS"
export function addSourceOnLoadSuccess(source: Source) {
    return { type: ADD_SOURCE_ON_LOAD_SUCCESS, source }
}

export const ADD_SOURCE_ON_LOAD_ERROR = "ADD_SOURCE_ON_LOAD_ERROR"
export function addSourceOnLoadError(error: any) {
    return {
        type: ADD_SOURCE_ON_LOAD_ERROR,
        error
    }
}
