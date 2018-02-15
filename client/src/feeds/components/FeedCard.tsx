import * as React from "react"
import { FeedSimple } from "../Feed"
import * as styles from "./FeedCard.css"

interface Props {
    feed: FeedSimple
}

export default class FeedCard extends React.PureComponent<Props> {
    render() {
        const { feed } = this.props
        const readable = feed.readable
        const rss = feed.rss
        return (
            <div className={styles.feedCard}>
                <div><a href={feed.url}>{(readable && readable.title) || (rss && rss.title)}</a></div>
            </div>
        )
    }
}
