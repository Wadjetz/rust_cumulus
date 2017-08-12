import { Action } from "redux"
import { Source } from "./Source"
import {
    SOURCES_ON_LOAD, SOURCES_ON_LOAD_ERROR, SOURCES_ON_LOAD_SUCCESS,
    FALLOW_SOURCE_ON_LOAD, FALLOW_SOURCE_ON_LOAD_ERROR, FALLOW_SOURCE_ON_LOAD_SUCCESS,
    ADD_SOURCE_ON_CHANGE, ADD_SOURCE_ON_LOAD, ADD_SOURCE_ON_LOAD_ERROR, ADD_SOURCE_ON_LOAD_SUCCESS
} from "./SourcesActions"

export interface SourcesState {
    sources: Source[]
    loading: boolean
    error?: any
    newSourceUrl: string
    addSourceLoading: boolean
}

const initState: SourcesState = {
    sources: [],
    loading: false,
    error: undefined,
    newSourceUrl: "",
    addSourceLoading: false,
}

const SourcesReducer = (state: SourcesState = initState, action: any) => {
    switch (action.type) {
        case ADD_SOURCE_ON_CHANGE: {
            const field = action.field
            const value = action.value
            return { ...state, [field]: value }
        }

        case ADD_SOURCE_ON_LOAD: return { ...state, addSourceLoading: true }
        case ADD_SOURCE_ON_LOAD_SUCCESS: return {
            ...state,
            addSourceLoading: false,
            sources: [...state.sources, action.source]
        }
        case ADD_SOURCE_ON_LOAD_ERROR: return { ...state, addSourceLoading: false, error: action.error }

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


