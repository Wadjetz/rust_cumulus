import { Source } from "./Source"

export const SOURCES_ON_LOAD = "SOURCES_ON_LOAD"
export const SOURCES_ON_LOAD_SUCCESS = "SOURCES_ON_LOAD_SUCCESS"
export const SOURCES_ON_LOAD_ERROR = "SOURCES_ON_LOAD_ERROR"

export function sourcesOnLoad() {
    return { type: SOURCES_ON_LOAD }
}

export function sourcesOnLoadSuccess(sources: Source[]) {
    return {
        type: SOURCES_ON_LOAD_SUCCESS,
        sources
    }
}

export function sourcesOnLoadError(error: any) {
    return {
        type: SOURCES_ON_LOAD_ERROR,
        error
    }
}

export const FALLOW_SOURCE_ON_LOAD = "FALLOW_SOURCE_ON_LOAD"
export const FALLOW_SOURCE_ON_LOAD_SUCCESS = "FALLOW_SOURCE_ON_LOAD_SUCCESS"
export const FALLOW_SOURCE_ON_LOAD_ERROR = "FALLOW_SOURCE_ON_LOAD_ERROR"

export function fallowSourcesOnLoad() {
    return { type: FALLOW_SOURCE_ON_LOAD }
}

export function fallowSourcesOnLoadSuccess(source: Source) {
    return { type: FALLOW_SOURCE_ON_LOAD_SUCCESS, source }
}

export function fallowSourcesOnLoadError(error: any) {
    return {
        type: FALLOW_SOURCE_ON_LOAD_ERROR,
        error
    }
}
