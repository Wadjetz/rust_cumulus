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

export type RELOAD_UNREADED_FEEDS = {
    type: "RELOAD_UNREADED_FEEDS"
}
export function reloadUnreadedFeeds(): RELOAD_UNREADED_FEEDS {
    return { type: "RELOAD_UNREADED_FEEDS" }
}
export type READ_FEED = {
    type: "READ_FEED"
    feed: Feed
    reaction: Reaction
}
export function readFeed(feed: Feed, reaction: Reaction): READ_FEED {
    return { type: "READ_FEED", feed, reaction }
}

export type READ_FEED_SUCCESS = {
    type: "READ_FEED_SUCCESS"
    feed: Feed
}
export function readFeedSuccess(feed: Feed): READ_FEED_SUCCESS {
    return { type: "READ_FEED_SUCCESS", feed }
}

export type READ_FEED_ERROR = {
    type: "READ_FEED_ERROR"
    error: any
}
export function readFeedError(error: any): READ_FEED_ERROR {
    return { type: "READ_FEED_ERROR", error }
}

export type MindStreamAction =
    LOAD_UNREADED_FEEDS |
    LOAD_UNREADED_FEEDS_SUCCESS |
    LOAD_UNREADED_FEEDS_ERROR |
    RELOAD_UNREADED_FEEDS |
    READ_FEED |
    READ_FEED_SUCCESS |
    READ_FEED_ERROR
