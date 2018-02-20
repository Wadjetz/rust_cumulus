import { Feed, Reaction } from "../feeds/Feed"

export type MindStreamAction =
    MIND_STREAM_API_ERROR |
    LOAD_UNREADED_FEEDS |
    LOAD_UNREADED_FEEDS_SUCCESS |
    READ_FEED |
    LOAD_UNREADED_FEEDS_BY_SOURCE |
    LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS |
    NEXT_FEED |
    NEXT_FEED_SUCCESS |
    PREVIOUS_FEED |
    GO_TO_NEXT_FEED

export type MIND_STREAM_API_ERROR = { type: "MIND_STREAM_API_ERROR", error: any }
export const mindStreamApiError = (error: any): MIND_STREAM_API_ERROR => ({ type: "MIND_STREAM_API_ERROR", error })

export type GO_TO_NEXT_FEED = { type: "GO_TO_NEXT_FEED" }
export const goToNextFeed = (): GO_TO_NEXT_FEED => ({ type: "GO_TO_NEXT_FEED" })

export type NEXT_FEED = { type: "NEXT_FEED", feed: Feed, sourceUuid?: string }
export const nextFeed = (feed: Feed, sourceUuid: string | undefined): NEXT_FEED => ({ type: "NEXT_FEED", feed, sourceUuid })

export type NEXT_FEED_SUCCESS = {
    type: "NEXT_FEED_SUCCESS"
    feed: Feed
    sourceUuid?: string
}
export function nextFeedSuccess(feed: Feed, sourceUuid?: string): NEXT_FEED_SUCCESS {
    return { type: "NEXT_FEED_SUCCESS", feed, sourceUuid }
}

export type PREVIOUS_FEED = { type: "PREVIOUS_FEED", sourceUuid?: string }
export const previousFeed = (sourceUuid: string | undefined): PREVIOUS_FEED => ({ type: "PREVIOUS_FEED", sourceUuid })

export type LOAD_UNREADED_FEEDS = {
    type: "LOAD_UNREADED_FEEDS"
}
export function loadUnreadedFeeds(): LOAD_UNREADED_FEEDS {
    return { type: "LOAD_UNREADED_FEEDS" }
}

export type LOAD_UNREADED_FEEDS_SUCCESS = {
    type: "LOAD_UNREADED_FEEDS_SUCCESS"
    feeds: Feed[]
}
export function loadUnreadedFeedsSuccess(feeds: Feed[]): LOAD_UNREADED_FEEDS_SUCCESS {
    return { type: "LOAD_UNREADED_FEEDS_SUCCESS", feeds }
}


export type READ_FEED = {
    type: "READ_FEED"
    feed: Feed
    reaction: Reaction
    sourceUuid?: string
}
export function readFeed(feed: Feed, reaction: Reaction, sourceUuid?: string): READ_FEED {
    return { type: "READ_FEED", feed, reaction, sourceUuid }
}

export type LOAD_UNREADED_FEEDS_BY_SOURCE = {
    type: "LOAD_UNREADED_FEEDS_BY_SOURCE"
    sourceUuid: string
}
export function loadUnreadedFeedsBySource(sourceUuid: string): LOAD_UNREADED_FEEDS_BY_SOURCE {
    return { type: "LOAD_UNREADED_FEEDS_BY_SOURCE", sourceUuid }
}

export type LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS = {
    type: "LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS"
    feeds: Feed[]
}
export function loadUnreadedFeedsBySourceSuccess(feeds: Feed[]): LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS {
    return { type: "LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS", feeds }
}
