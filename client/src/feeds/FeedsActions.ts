import { FeedSimple } from "./Feed"

export type FeedsAction = LOAD_FEEDS | LOAD_FEEDS_SUCCESS | LOAD_FEEDS_ERROR

export type LOAD_FEEDS = {
    type: "LOAD_FEEDS"
}
export function loadfeeds(): LOAD_FEEDS {
    return { type: "LOAD_FEEDS" }
}

export type LOAD_FEEDS_SUCCESS = {
    type: "LOAD_FEEDS_SUCCESS"
    feeds: FeedSimple[]
}
export function loadfeedsSuccess(feeds: FeedSimple[]): LOAD_FEEDS_SUCCESS {
    return { type: "LOAD_FEEDS_SUCCESS", feeds }
}

export type LOAD_FEEDS_ERROR = {
    type: "LOAD_FEEDS_ERROR"
    error: any
}
export function loadfeedsError(error: any): LOAD_FEEDS_ERROR {
    return { type: "LOAD_FEEDS_ERROR", error }
}
