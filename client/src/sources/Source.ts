export type SourceType = "Rss"

export interface RssSource {
    title: string
    xmlUrl: string
    htmlUrl: string
}

export interface Source {
    uuid: string
    sourceType: SourceType
    rssSource?: RssSource
    error?: string
}

export interface SourceStat {
    uuid: string
    count: number
}
