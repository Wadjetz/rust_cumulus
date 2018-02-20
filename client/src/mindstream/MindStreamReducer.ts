import { Feed } from "../feeds/Feed"
import { MindStreamAction } from "./MindStreamActions"

export interface MindStreamState {
    previusFeeds: Feed[]
    feeds: Feed[]
    nextFeedLoader: boolean
    loading: boolean
}

const initState: MindStreamState = {
    previusFeeds: [],
    feeds: [],
    nextFeedLoader: false,
    loading: false,
}

const MindStreamReducer = (state: MindStreamState = initState, action: MindStreamAction) => {
    switch (action.type) {
        case "MIND_STREAM_API_ERROR": return { ...state, loading: false, nextFeedLoader: false }
        case "LOAD_UNREADED_FEEDS": return { ...state, loading: true }
        case "LOAD_UNREADED_FEEDS_SUCCESS": return { ...state, feeds: action.feeds, loading: false }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE": return { ...state, loading: true }
        case "LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS": return { ...state, feeds: action.feeds, loading: false }
        case "NEXT_FEED": return { ...state, nextFeedLoader: true }
        case "NEXT_FEED_SUCCESS": return { ...state, nextFeedLoader: false }
        case "GO_TO_NEXT_FEED": return goToNextFeedReduce(state, action)
        case "PREVIOUS_FEED": return previousFeedReduce(state, action)
        default: return state
    }
}

function goToNextFeedReduce(state: MindStreamState, action: MindStreamAction): MindStreamState {
    const [first, ...rest] = state.feeds
    return { ...state, feeds: rest, previusFeeds: [first, ...state.previusFeeds.slice(0, 10)] }
}

function previousFeedReduce(state: MindStreamState, action: MindStreamAction): MindStreamState {
    const [first, ...rest] = state.previusFeeds
    if (first) {
        return { ...state, feeds: [first, ...state.feeds], previusFeeds: rest }
    } else {
        return state
    }
}

export default MindStreamReducer
