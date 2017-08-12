import { Feed } from "../feeds/Feed"
import {
    LOAD_UNREADED_FEEDS, LOAD_UNREADED_FEEDS_ERROR, LOAD_UNREADED_FEEDS_SUCCESS,
    READ_FEED, READ_FEED_ERROR, READ_FEED_SUCCESS,
} from "./MindStreamActions"

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

const MindStreamReducer = (state: MindStreamState = initState, action: any) => {
    switch (action.type) {
        case LOAD_UNREADED_FEEDS: return { ...state, loading: true }
        case LOAD_UNREADED_FEEDS_SUCCESS: return { ...state, feeds: action.feeds, loading: false }
        case LOAD_UNREADED_FEEDS_ERROR: return { ...state, loading: false, error: action.error }

        case READ_FEED: return { ...state, loading: true }
        case READ_FEED_ERROR: return { ...state, loading: false, error: action.error }
        case READ_FEED_SUCCESS: return {
            ...state,
            loading: false,
            feeds: state.feeds.filter(feed => feed.uuid !== action.feed.uuid)
        }

        default: return state
    }
}

export default MindStreamReducer

