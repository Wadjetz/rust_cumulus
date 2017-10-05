import { Feed } from "../feeds/Feed"
import { MindStreamAction } from "./MindStreamActions"

export interface MindStreamState {
    feeds: Feed[],
    loading: boolean
    error?: any
}

const initState: MindStreamState = {
    feeds: [],
    loading: false,
    error: undefined,
}

const MindStreamReducer = (state: MindStreamState = initState, action: MindStreamAction) => {
    switch (action.type) {
        case "LOAD_UNREADED_FEEDS": return { ...state, loading: true }
        case "LOAD_UNREADED_FEEDS_SUCCESS": return { ...state, feeds: action.feeds, loading: false }
        case "LOAD_UNREADED_FEEDS_ERROR": return { ...state, loading: false, error: action.error }
        case "READ_FEED": return { ...state, loading: true }
        case "READ_FEED_ERROR": return { ...state, loading: false, error: action.error }
        case "READ_FEED_SUCCESS": return {
            ...state,
            loading: false,
            feeds: state.feeds.filter(feed => feed.uuid !== action.feed.uuid)
        }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE": return { ...state, loading: true }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS": return { ...state, feeds: action.feeds, loading: false }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE_ERROR": return { ...state, loading: false, error: action.error }
        default: return state
    }
}

export default MindStreamReducer

