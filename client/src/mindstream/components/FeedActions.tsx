import * as React from "react"
import * as styles from "./FeedActions.css"
import { Feed, Reaction } from "../../feeds/Feed"

interface Props {
    feed: Feed
    loading: boolean
    onReaction: (feed: Feed, reaction: Reaction) => () => void
}

export default class FeedActions extends React.Component<Props, {}> {
    render() {
        const { feed, loading, onReaction } = this.props
        return (
            <div className={styles.container}>
                <button className={styles.action} disabled={loading} onClick={onReaction(feed, "Liked")}>Liked</button>
                <button className={styles.action} disabled={loading} onClick={onReaction(feed, "Readed")}>Read</button>
            </div>
        )
    }
}
