import { Action } from "redux"
import { Feed } from "../feeds/Feed"
import {
    MIND_STREAM_ON_LOAD, MIND_STREAM_ON_LOAD_ERROR, MIND_STREAM_ON_LOAD_SUCCESS,
    READ_FEED_ON_LOAD, READ_FEED_ON_LOAD_ERROR, READ_FEED_ON_LOAD_SUCCESS,
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
        case MIND_STREAM_ON_LOAD: return {
            ...state,
            loading: true
        }

        case MIND_STREAM_ON_LOAD_SUCCESS: return {
            ...state,
            feeds: action.feeds,
            loading: false
        }

        case MIND_STREAM_ON_LOAD_ERROR: return {
            ...state,
            loading: false,
            error: action.error
        }

        case READ_FEED_ON_LOAD: return {
            ...state,
            loading: true
        }

        case READ_FEED_ON_LOAD_SUCCESS: return {
            ...state,
            loading: false,
            feeds: state.feeds.filter(feed => feed.uuid !== action.feed.uuid)
        }

        case READ_FEED_ON_LOAD_ERROR: return {
            ...state,
            loading: false,
            error: action.error
        }

        default: return state
    }
}

export default MindStreamReducer

