import { Feed, FeedSimple, Reaction } from "./feeds/Feed"
import { Source, SourceStat } from "./sources/Source"
import * as router from "./router"

const BASE_URI = document.location.origin
const AUTH_TOKEN_STORAGE_KEY = "AUTH_TOKEN_STORAGE_KEY"
const FEEDS_LIMIT = 15
const SOURCES_LIMIT = 1000

export interface ApiError {
    errors: Array<{ message: string }>
}

function withToken(): Promise<string> {
    return new Promise((resolve, reject) => {
        const token = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY) || sessionStorage.getItem(AUTH_TOKEN_STORAGE_KEY)
        if (!token) {
            router.replace("/login")
            reject({
                errors: [{ message: "Unauthorized" }]
            })
        } else {
            resolve(token)
        }
    })
}

function query(query: string): Promise<any> {
    return fetch(`${BASE_URI}/graphql`, fetchOptions(query))
        .then(response => response.json())
        .then(success)
        .then(log)
}

function fetchOptions(query: string): RequestInit {
    return {
        method: "POST",
        body: JSON.stringify({
            query: query.replace(/\s\s*/g, " "),
        }),
        headers: {
            "Content-Type": "application/json"
        }
    }
}

export function login(email: string, password: string): Promise<void> {
    return query(`query {
        login(email: "${email}", password: "${password}")
    }`).then(result => {
        localStorage.setItem(AUTH_TOKEN_STORAGE_KEY, result.login)
    })
}

export function signup(login: string, email: string, password: string): Promise<void> {
    return query(`mutation {
        signup(login: "${login}", email: "${email}", password: "${password}")
    }`).then(result => {
        localStorage.setItem(AUTH_TOKEN_STORAGE_KEY, result.signup)
    })
}

export function loadUnfollowedSources(): Promise<Source[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                unfollowedSources {
                    uuid
                    sourceType
                    rssSource {
                        title
                        xmlUrl
                        htmlUrl
                    }
                    error
                }
            }
        }
    `))
    .then(result => result.auth.unfollowedSources)
}

export function loadMySources(): Promise<Source[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                mySources(limit: ${SOURCES_LIMIT}) {
                    uuid
                    sourceType
                    rssSource {
                        title
                        xmlUrl
                        htmlUrl
                    }
                    error
                }
            }
        }
    `))
    .then(result => result.auth.mySources)
}

export function loadMySourcesStats(): Promise<SourceStat[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                sourcesStats {
                    uuid
                    count
                }
            }
        }
    `))
    .then(result => result.auth.sourcesStats)
}

export function loadUnreadedFeeds(): Promise<Feed[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                unreadedFeeds(limit: ${FEEDS_LIMIT}) {
                    uuid
                    url
                    readable {
                        url
                        title
                        content
                        excerpt
                        leadImageUrl
                    }
                    rss {
                        title
                        content
                        summary
                    }
                }
            }
        }
    `))
    .then(result => result.auth.unreadedFeeds)
}

export function loadUnreadedFeedsBySource(sourceUuid: string): Promise<Feed[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                unreadedFeedsBySource(sourceUuid: "${sourceUuid}", limit: ${FEEDS_LIMIT}) {
                    uuid
                    url
                    readable {
                        url
                        title
                        content
                        excerpt
                        leadImageUrl
                    }
                    rss {
                        title
                        content
                        summary
                    }
                }
            }
        }
    `))
    .then(result => result.auth.unreadedFeedsBySource)
}

export function feedsByReaction(reaction: Reaction): Promise<FeedSimple[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                feedsByReaction(reaction: "${reaction}", limit: 10000) {
                    uuid
                    url
                    readable {
                        title
                    }
                    rss {
                        title
                    }
                }
            }
        }
    `))
    .then(result => result.auth.feedsByReaction)
}

function log<T>(t: T): T {
    // console.log("fetch log", t)
    return t
}

export function fallowSource(source: Source): Promise<Source> {
    return withToken().then(token => query(`
        mutation {
            auth(token: "${token}") {
                fallowSource(sourceUuid: "${source.uuid}") {
                    uuid
                    sourceType
                    rssSource {
                        title
                        xmlUrl
                        htmlUrl
                    }
                    error
                }
            }
        }
    `))
    .then(result => result.auth.fallowSource)
}

export function addSource(xmlUrl: string): Promise<Source> {
    return query(`
        mutation {
            addRssSource(xmlUrl: "${xmlUrl}") {
                uuid
                sourceType
                rssSource {
                    title
                    xmlUrl
                    htmlUrl
                }
                error
                created
                updated
            }
        }
    `)
    .then(result => result.addRssSource)
}

export function feedReaction(feed: Feed, reaction: Reaction): Promise<Feed> {
    return withToken().then(token => query(`
        mutation {
            auth(token: "${token}") {
                feedReaction(feedUuid: "${feed.uuid}", reaction: "${reaction}")
            }
        }
    `))
    .then(() => feed)
}

export function success(result: any) {
    if (result.errors) {
        throw { errors: result.errors }
    } else {
        return result.data
    }
}

export function logout() {
    localStorage.removeItem(AUTH_TOKEN_STORAGE_KEY)
    router.replace("/login")
}
