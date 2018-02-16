import { Source, SourceStat } from "./Source"
import { SourceAction } from "./SourcesActions"

export interface SourcesState {
    sources: Source[]
    mySources: Source[]
    mySourcesStats: SourceStat[]
    mySourcesStatsLoader: boolean
    loading: boolean
    error?: any
    newSourceUrl: string
    addSourceLoading: boolean
}

const initState: SourcesState = {
    sources: [],
    mySources: [],
    mySourcesStats: [],
    mySourcesStatsLoader: false,
    loading: false,
    error: undefined,
    newSourceUrl: "",
    addSourceLoading: false,
}

const SourcesReducer = (state: SourcesState = initState, action: SourceAction) => {
    switch (action.type) {
        case "ADD_SOURCE_ON_CHANGE": return { ...state, [action.field]: action.value }

        case "ADD_SOURCE": return { ...state, addSourceLoading: true }
        case "ADD_SOURCE_SUCCESS": return {
            ...state,
            addSourceLoading: false,
            newSourceUrl: "",
            sources: [...state.sources, action.source]
        }
        case "ADD_SOURCE_ERROR": return { ...state, addSourceLoading: false, error: action.error }

        case "LOAD_UNFOLLOWED_SOURCES": return { ...state, loading: true }
        case "LOAD_UNFOLLOWED_SOURCES_SUCCESS": return { ...state, sources: action.sources, loading: false }
        case "LOAD_UNFOLLOWED_SOURCES_ERROR": return { ...state, loading: false, error: action.error }

        case "LOAD_MY_SOURCES": return { ...state, loading: true }
        case "LOAD_MY_SOURCES_SUCCESS": return { ...state, mySources: action.sources, loading: false }
        case "LOAD_MY_SOURCES_ERROR": return { ...state, loading: false, error: action.error }

        case "LOAD_MY_SOURCES_STATS": return { ...state, mySourcesStatsLoader: true }
        case "LOAD_MY_SOURCES_STATS_SUCCESS": return { ...state, mySourcesStats: action.stats, mySourcesStatsLoader: false }
        case "LOAD_MY_SOURCES_STATS_ERROR": return { ...state, error: action.error, mySourcesStatsLoader: false }

        case "ADD_MY_SOURCE": return { ...state, mySources: [...state.mySources, action.source] }

        case "FALLOW_SOURCE": return { ...state, loading: true}
        case "FALLOW_SOURCE_ERROR": return { ...state, loading: false, error: action.error }
        case "FALLOW_SOURCE_SUCCESS": return {
            ...state,
            sources: state.sources.filter(source => source.uuid !== action.source.uuid),
            loading: false
        }
        default: return state
    }
}

export default SourcesReducer
