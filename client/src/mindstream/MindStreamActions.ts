import { Feed, Reaction } from "../feeds/Feed"

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

export type LOAD_UNREADED_FEEDS_ERROR = {
    type: "LOAD_UNREADED_FEEDS_ERROR"
    error: any
}
export function loadUnreadedFeedsError(error: any): LOAD_UNREADED_FEEDS_ERROR {
    return { type: "LOAD_UNREADED_FEEDS_ERROR", error }
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

export type READ_FEED_SUCCESS = {
    type: "READ_FEED_SUCCESS"
    feed: Feed
    sourceUuid?: string
}
export function readFeedSuccess(feed: Feed, sourceUuid?: string): READ_FEED_SUCCESS {
    return { type: "READ_FEED_SUCCESS", feed, sourceUuid }
}

export type READ_FEED_ERROR = {
    type: "READ_FEED_ERROR"
    error: any
}
export function readFeedError(error: any): READ_FEED_ERROR {
    return { type: "READ_FEED_ERROR", error }
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
}
export function loadUnreadedFeedsBySourceSuccess(): LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS {
    return { type: "LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS" }
}

export type LOAD_UNREADED_FEEDS_BY_SOURCE_ERROR = {
    type: "LOAD_UNREADED_FEEDS_BY_SOURCE_ERROR"
}
export function loadUnreadedFeedsBySourceError(error: any): LOAD_UNREADED_FEEDS_BY_SOURCE_ERROR {
    return { type: "LOAD_UNREADED_FEEDS_BY_SOURCE_ERROR" }
}

export type MindStreamAction =
    LOAD_UNREADED_FEEDS |
    LOAD_UNREADED_FEEDS_SUCCESS |
    LOAD_UNREADED_FEEDS_ERROR |
    READ_FEED |
    READ_FEED_SUCCESS |
    READ_FEED_ERROR |
    LOAD_UNREADED_FEEDS_BY_SOURCE |
    LOAD_UNREADED_FEEDS_BY_SOURCE_SUCCESS |
    LOAD_UNREADED_FEEDS_BY_SOURCE_ERROR
