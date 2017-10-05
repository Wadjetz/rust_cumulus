import * as React from "react"
import * as styles from "./FeedActions.css"
import { Feed, Reaction } from "../../feeds/Feed"

interface Props {
    feed: Feed
    sourceUuid?: string
    loading: boolean
    onReaction: (feed: Feed, reaction: Reaction, sourceUuid?: string) => () => void
}

export default class FeedActions extends React.Component<Props, {}> {
    render() {
        const { feed, loading, onReaction, sourceUuid } = this.props
        return (
            <div className={styles.container}>
                <button className={styles.action} disabled={loading} onClick={onReaction(feed, "Liked", sourceUuid)}>Liked</button>
                <button className={styles.action} disabled={loading} onClick={onReaction(feed, "Readed", sourceUuid)}>Read</button>
            </div>
        )
    }
}
