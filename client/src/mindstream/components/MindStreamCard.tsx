import * as React from "react"
import { Feed } from "../../feeds/Feed"
import FeedReadable from "../../components/FeedReadable"
import FeedRss from "../../components/FeedRss"

interface Props {
    feed: Feed
}

export default class MindStreamCard extends React.Component<Props, {}> {
    render() {
        return (
            <div>
                {this.renderContent()}
            </div>
        )
    }

    renderContent = () => {
        const { feed } = this.props
        if (feed.readable) {
            return <FeedReadable readable={feed.readable} />
        } else if (feed.rss) {
            return <FeedRss rss={feed.rss} feed_url={feed.url} />
        }
    }
}
