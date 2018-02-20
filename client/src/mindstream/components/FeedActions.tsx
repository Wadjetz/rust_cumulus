import * as React from "react"
import * as styles from "./FeedActions.css"
import { Feed, Reaction } from "../../feeds/Feed"
import FeedAction from "./FeedAction"

interface Props {
    feed: Feed
    sourceUuid?: string
    loading: boolean
    nextFeedLoader: boolean
    onNextFeed(feed: Feed, sourceUuid: string | undefined): void
    onPreviousFeed(sourceUuid: string | undefined): void
    onReaction(feed: Feed, reaction: Reaction, sourceUuid?: string): () => void
}

export default class FeedActions extends React.PureComponent<Props> {
    render() {
        const { feed, loading, nextFeedLoader, sourceUuid, onReaction, onNextFeed, onPreviousFeed } = this.props
        return (
            <div className={styles.container}>
                <FeedAction className={styles.actionLike} name="Previous" loading={loading} onClick={() => onPreviousFeed(sourceUuid)} />
                <FeedAction className={styles.actionRead} name="Liked" loading={loading} onClick={onReaction(feed, "Liked", sourceUuid)} />
                <FeedAction className={styles.actionNext} name="Next" loading={nextFeedLoader} onClick={() => onNextFeed(feed, sourceUuid)} />
            </div>
        )
    }
}
