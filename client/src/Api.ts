import { Feed } from "./feeds/Feed"
import * as router from "./router"

const BASE_URI = "http://localhost:8000"

interface LoginResponse {
    login: string
}
export function login(email: string, password: string): Promise<LoginResponse> {
    const query = `mutation {
        login(email: "${email}", password: "${password}")
    }`.replace(/\s\s*/g, " ")
    return fetch(`${BASE_URI}/graphql`, {
        method: "POST",
        body: JSON.stringify({
            query: query,
        }),
        headers: {
            "Content-Type": "application/json"
        }
    })
    .then(response => response.json())
    .then(success)
}

export function loadFeeds(token: string): Promise<Feed[]> {
    const query = `
    query {
        auth(token: "${token}") {
            feeds {
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
    `.replace(/\s\s*/g, " ")
    return fetch(`${BASE_URI}/graphql`, {
        method: "POST",
        body: JSON.stringify({
            query: query,
        }),
        headers: {
            "Content-Type": "application/json"
        }
    })
    .then(response => response.json())
    .then(success)
    .then(result => result.auth.feeds)
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
