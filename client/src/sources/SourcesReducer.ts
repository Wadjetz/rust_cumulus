import { Action } from "redux"
import { Source } from "./Source"
import {
    SOURCES_ON_LOAD, SOURCES_ON_LOAD_ERROR, SOURCES_ON_LOAD_SUCCESS,
    FALLOW_SOURCE_ON_LOAD, FALLOW_SOURCE_ON_LOAD_ERROR, FALLOW_SOURCE_ON_LOAD_SUCCESS
} from "./SourcesActions"

export interface SourcesState {
    sources: Source[]
    loading: boolean
    error?: any
}

const initState: SourcesState = {
    sources: [],
    loading: false,
    error: undefined,
}

const SourcesReducer = (state: SourcesState = initState, action: any) => {
    switch (action.type) {
        case SOURCES_ON_LOAD: return {
            ...state,
            loading: true
        }

        case SOURCES_ON_LOAD_SUCCESS: return {
            ...state,
            sources: action.sources,
            loading: false
        }

        case SOURCES_ON_LOAD_ERROR: return {
            ...state,
            loading: false,
            error: action.error
        }

        case FALLOW_SOURCE_ON_LOAD: return {
            ...state,
            loading: true
        }

        case FALLOW_SOURCE_ON_LOAD_SUCCESS: return {
            ...state,
            sources: state.sources.filter(source => source.uuid !== action.source.uuid),
            loading: false
        }

        case FALLOW_SOURCE_ON_LOAD_ERROR: return {
            ...state,
            loading: false,
            error: action.error
        }
        default: return state
    }
}

export default SourcesReducer


