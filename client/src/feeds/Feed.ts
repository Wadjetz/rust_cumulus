
export interface Feed {
    uuid: string
    url: string
    readable?: Readable,
    rss?: {
        title: string
        content?: string
        summary?: string
    }
}

export interface Readable {
    url: string
    title: string
    content: string
    excerpt: string
    leadImageUrl?: string
}
