import { Feed, Reaction } from "./feeds/Feed"
import { Source } from "./sources/Source"
import * as router from "./router"

const BASE_URI = document.location.origin

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

export function login(email: string, password: string): Promise<string> {
    return query(`mutation {
        login(email: "${email}", password: "${password}")
    }`).then(result => result.login)
}


export function loadUnfollowedSources(token: string): Promise<Source[]> {
    return query(`
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
    .then(result => result.auth.unfollowedSources)
}

export function loadUnreadedFeeds(token: string): Promise<Feed[]> {
    return query(`
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
    .then(result => result.auth.unreadedFeeds)
}

function log<T>(t: T): T {
    console.log("fetch log", t)
    return t
}

export function fallowSource(token: string, source: Source): Promise<Source> {
    return query(`
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
    `)
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

export function readFeed(token: string, feed: Feed, reaction: Reaction): Promise<Feed> {
    return query(`
        mutation {
            auth(token: "${token}") {
                feedReaction(feedUuid: "${feed.uuid}", reaction: "${reaction}")
            }
        }
    `)
    .then(() => feed)
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
