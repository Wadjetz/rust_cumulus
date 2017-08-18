import * as React from "react"
import { Feed } from "../Feed"
import * as styles from "./FeedCard.css"

interface Props {
    feed: Feed
}

export default class FeedCard extends React.Component<Props, {}> {
    render() {
        const { feed } = this.props
        const readable = feed.readable
        const rss = feed.rss
        return (
            <div className={styles.feedCard}>
                <h4><a href={feed.url}>{(readable && readable.title) || (rss && rss.title)}</a></h4>
            </div>
        )
    }
}
