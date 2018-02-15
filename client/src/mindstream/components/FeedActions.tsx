import * as React from "react"
import * as styles from "./FeedActions.css"
import { Feed, Reaction } from "../../feeds/Feed"

interface Props {
    feed: Feed
    sourceUuid?: string
    loading: boolean
    onNextFeed(feed: Feed, sourceUuid: string | undefined): void
    onPreviousFeed(sourceUuid: string | undefined): void
    onReaction(feed: Feed, reaction: Reaction, sourceUuid?: string): () => void
}

export default class FeedActions extends React.PureComponent<Props> {
    render() {
        const { feed, loading, sourceUuid, onReaction, onNextFeed, onPreviousFeed } = this.props
        return (
            <div className={styles.container}>
                <button className={`${styles.action} ${styles.actionLike}`} disabled={loading} onClick={() => onPreviousFeed(sourceUuid)}>Previous</button>
                <button className={`${styles.action} ${styles.actionRead}`} disabled={loading} onClick={onReaction(feed, "Liked", sourceUuid)}>Liked</button>
                <button className={`${styles.action} ${styles.actionNext}`} disabled={loading} onClick={() => onNextFeed(feed, sourceUuid)}>Next</button>
            </div>
        )
    }
}
