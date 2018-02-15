import { FeedsAction } from "./FeedsActions"

import { FeedSimple } from "./Feed"

export interface FeedsState {
    feeds: FeedSimple[]
    loading: boolean
    error?: any
}

const initState: FeedsState = {
    feeds: [],
    loading: false,
    error: undefined,
}

const FeedsReducer = (state: FeedsState = initState, action: FeedsAction) => {
    switch (action.type) {
        case "LOAD_FEEDS": return { ...state, loading: true }
        case "LOAD_FEEDS_SUCCESS": return { ...state, feeds: action.feeds, loading: false }
        case "LOAD_FEEDS_ERROR": return { ...state, loading: false, error: action.error }

        default: return state
    }
}

export default FeedsReducer
