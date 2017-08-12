import { Feed, Reaction } from "./feeds/Feed"
import { Source } from "./sources/Source"
import * as router from "./router"

const BASE_URI = "http://localhost:8000"

interface LoginResponse {
    login: string
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

export function login(email: string, password: string): Promise<LoginResponse> {
    const options = fetchOptions(`mutation {
        login(email: "${email}", password: "${password}")
    }`)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
}


export function loadUnfollowedSources(token: string): Promise<Source[]> {
    const options = fetchOptions(`
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
    `)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
    .then(log)
    .then(result => result.auth.unfollowedSources)
}

export function loadUnreadedFeeds(token: string): Promise<Feed[]> {
    const options = fetchOptions(`
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
    `)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
    .then(result => result.auth.unreadedFeeds)
}

function log<T>(t: T): T {
    console.log("fetch log", t)
    return t
}

export function fallowSource(token: string, source: Source): Promise<void> {
    const options = fetchOptions(`
        mutation {
            auth(token: "${token}") {
                fallowSource(sourceUuid: "${source.uuid}") {
                    uuid
                }
            }
        }
    `)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
}

export function addSource(xmlUrl: string): Promise<Source> {
    const options = fetchOptions(`
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
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
    .then(result => result.addRssSource)
}

export function readFeed(token: string, feed: Feed, reaction: Reaction): Promise<void> {
    const options = fetchOptions(`
        mutation {
            auth(token: "${token}") {
                feedReaction(feedUuid: "${feed.uuid}", reaction: "${reaction}")
            }
        }
    `)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
}

export function success(result: any) {
    if (result.errors) {
        if (result.errors.find((e: any) => e.message === "invalid token")) {
            router.push("/login")
        }
        throw { errors: result.errors }
    } else {
        return result.data
    }
}
