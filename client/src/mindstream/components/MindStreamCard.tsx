import * as React from "react"
import { Feed, Rss } from "../../feeds/Feed"
import FeedReadable from "./FeedReadable"

interface Props {
    feed: Feed
}

export default class MindStreamCard extends React.PureComponent<Props> {
    render() {
        return (
            <div>
                {this.renderContent()}
            </div>
        )
    }

    renderContent = () => {
        const { feed } = this.props
        const { readable, rss } = feed
        if (readable) {
            const { title, url, content } = readable
            const composedTitle = title || rss && rss.title || "No Title"
            return <FeedReadable title={composedTitle} url={url} leadImageUrl={this.isImageAlreadyShow()} content={content} />
        } else if (rss) {
            return <FeedReadable title={rss.title || "No title"} url={feed.url} content={getRssContent(rss)} />
        }
    }

    isImageAlreadyShow = (): string | undefined => {
        const { readable } = this.props.feed
        if (readable && readable.leadImageUrl && readable.content.indexOf(readable.leadImageUrl) !== -1) {
            return readable.leadImageUrl
        }
    }
}

function getRssContent(rss: Rss): string {
    const { content, summary } = rss
    if (content && !summary) {
        return content
    } else if (summary && !content) {
        return summary
    } else if (content && summary) {
        return content.length > summary.length ? content : summary
    } else {
        return ""
    }
}
