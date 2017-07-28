import { Feed } from "./Feed"

export const FEEDS_ON_LOAD = "FEEDS_ON_LOAD"
export const FEEDS_ON_LOAD_SUCCESS = "FEEDS_ON_LOAD_SUCCESS"
export const FEEDS_ON_LOAD_ERROR = "FEEDS_ON_LOAD_ERROR"

export function feedsOnLoad() {
    return { type: FEEDS_ON_LOAD }
}

export function feedsOnLoadSuccess(feeds: Feed[]) {
    return {
        type: FEEDS_ON_LOAD_SUCCESS,
        feeds
    }
}

export function feedsOnLoadError(error: any) {
    return {
        type: FEEDS_ON_LOAD_ERROR,
        error
    }
}
