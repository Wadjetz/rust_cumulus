import { Source } from "./Source"
import {
    LOAD_UNFOLLOWED_SOURCES, LOAD_UNFOLLOWED_SOURCES_ERROR, LOAD_UNFOLLOWED_SOURCES_SUCCESS,
    FALLOW_SOURCE, FALLOW_SOURCE_ERROR, FALLOW_SOURCE_SUCCESS,
    ADD_SOURCE_ON_CHANGE, ADD_SOURCE, ADD_SOURCE_ERROR, ADD_SOURCE_SUCCESS
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
        case ADD_SOURCE_ON_CHANGE: return { ...state, [action.field]: action.value }

        case ADD_SOURCE: return { ...state, addSourceLoading: true }
        case ADD_SOURCE_SUCCESS: return { ...state, addSourceLoading: false, sources: [...state.sources, action.source] }
        case ADD_SOURCE_ERROR: return { ...state, addSourceLoading: false, error: action.error }

        case LOAD_UNFOLLOWED_SOURCES: return { ...state, loading: true }
        case LOAD_UNFOLLOWED_SOURCES_SUCCESS: return { ...state, sources: action.sources, loading: false }
        case LOAD_UNFOLLOWED_SOURCES_ERROR: return { ...state, loading: false, error: action.error }

        case FALLOW_SOURCE: return { ...state, loading: true}
        case FALLOW_SOURCE_ERROR: return { ...state, loading: false, error: action.error }
        case FALLOW_SOURCE_SUCCESS: return {
            ...state,
            sources: state.sources.filter(source => source.uuid !== action.source.uuid),
            loading: false
        }


        default: return state
    }
}

export default SourcesReducer


