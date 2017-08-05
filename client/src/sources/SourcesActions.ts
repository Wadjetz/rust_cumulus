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
