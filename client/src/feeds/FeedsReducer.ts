import { LOAD_FEEDS, LOAD_FEEDS_ERROR, LOAD_FEEDS_SUCCESS } from "./FeedsActions"

import { Feed } from "./Feed"

export interface FeedsState {
    feeds: Feed[]
    loading: boolean
    error?: any
}

const initState: FeedsState = {
    feeds: [],
    loading: false,
    error: undefined,
}

const FeedsReducer = (state: FeedsState = initState, action: any) => {
    switch (action.type) {
        case LOAD_FEEDS: return { ...state, loading: true }
        case LOAD_FEEDS_SUCCESS: return { ...state, feeds: action.feeds, loading: false }
        case LOAD_FEEDS_ERROR: return { ...state, loading: false, error: action.error }

        default: return state
    }
}

export default FeedsReducer
