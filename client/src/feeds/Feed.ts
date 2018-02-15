export type Reaction = "Readed" | "ReadLater" | "Viewed" | "Liked" | "Disliked" | "Archived"

export interface FeedSimple {
    uuid: string
    url: string
    readable?: ReadableSimple
    rss?: RssSimple
}

export interface ReadableSimple {
    title?: string
}

export interface RssSimple {
    title?: string
}

export interface Feed {
    uuid: string
    url: string
    readable?: Readable
    rss?: Rss
}

export interface Rss {
    url?: string
    title?: string
    content?: string
    summary?: string
}

export interface Readable {
    url: string
    title?: string
    content: string
    excerpt: string
    leadImageUrl?: string
}
