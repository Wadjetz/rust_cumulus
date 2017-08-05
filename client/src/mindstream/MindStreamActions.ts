import { Feed } from "../feeds/Feed"

export const MIND_STREAM_ON_LOAD = "MIND_STREAM_ON_LOAD"
export const MIND_STREAM_ON_LOAD_SUCCESS = "MIND_STREAM_ON_LOAD_SUCCESS"
export const MIND_STREAM_ON_LOAD_ERROR = "MIND_STREAM_ON_LOAD_ERROR"

export function mindStreamOnLoad() {
    return { type: MIND_STREAM_ON_LOAD }
}

export function mindStreamOnLoadSuccess(feeds: Feed[]) {
    return {
        type: MIND_STREAM_ON_LOAD_SUCCESS,
        feeds
    }
}

export function mindStreamOnLoadError(error: any) {
    return {
        type: MIND_STREAM_ON_LOAD_ERROR,
        error
    }
}

export const READ_FEED_ON_LOAD = "READ_FEED_ON_LOAD"
export const READ_FEED_ON_LOAD_SUCCESS = "READ_FEED_ON_LOAD_SUCCESS"
export const READ_FEED_ON_LOAD_ERROR = "READ_FEED_ON_LOAD_ERROR"

export function readFeedOnLoad() {
    return { type: READ_FEED_ON_LOAD }
}

export function readFeedOnLoadSuccess(feed: Feed) {
    return { type: READ_FEED_ON_LOAD_SUCCESS, feed }
}

export function readFeedOnLoadError(error: any) {
    return { type: READ_FEED_ON_LOAD_ERROR, error }
}
