import { Feed } from "./Feed"

export const LOAD_FEEDS = "LOAD_FEEDS"
export const loadfeeds = () => ({ type: LOAD_FEEDS })

export const LOAD_FEEDS_SUCCESS = "LOAD_FEEDS_SUCCESS"
export const loadfeedsSuccess = (feeds: Feed[]) => ({ type: LOAD_FEEDS_SUCCESS, feeds })

export const LOAD_FEEDS_ERROR = "LOAD_FEEDS_ERROR"
export const loadfeedsError = (error: any) => ({ type: LOAD_FEEDS_ERROR, error })
