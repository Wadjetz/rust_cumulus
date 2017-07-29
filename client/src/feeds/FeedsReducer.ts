import { Action } from "redux"
import { FEEDS_ON_LOAD, FEEDS_ON_LOAD_ERROR, FEEDS_ON_LOAD_SUCCESS } from "./FeedsActions"

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
        case FEEDS_ON_LOAD: return {
            ...state,
            loading: true
        }

        case FEEDS_ON_LOAD_SUCCESS: return {
            ...state,
            feeds: action.feeds,
            loading: false
        }

        case FEEDS_ON_LOAD_ERROR: return {
            ...state,
            loading: false,
            error: action.error
        }

        default: return state
    }
}

export default FeedsReducer
