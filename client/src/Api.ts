import { Feed, Reaction } from "./feeds/Feed"
import { Source } from "./sources/Source"
import * as router from "./router"

const BASE_URI = document.location.origin
const AUTH_TOKEN_STORAGE_KEY = "AUTH_TOKEN_STORAGE_KEY"

export interface ApiError {
    errors: { message: string }[]
}

function withToken(): Promise<string> {
    return new Promise((resolve: any, reject: any) => {
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
    console.log("query", BASE_URI)
    return fetch(`${BASE_URI}/graphql`, fetchOptions(query))
        .then(response => response.json())
        .then(success)
        .then(log)
}

function fetchOptions(query: string) {
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
                mySources {
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

export function loadUnreadedFeeds(): Promise<Feed[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                unreadedFeeds {
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

export function feedsByReaction(reaction: Reaction): Promise<Feed[]> {
    return withToken().then(token => query(`
        query {
            auth(token: "${token}") {
                feedsByReaction(reaction: "${reaction}") {
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
    .then(result => result.auth.feedsByReaction)
}

function log<T>(t: T): T {
    console.log("fetch log", t)
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
