import * as React from "react"
import { Feed } from "../Feed"
import FeedReadable from "../../components/FeedReadable"
import * as styles from "./FeedCard.css"

interface Props {
    feed: Feed
}

export default class FeedCard extends React.Component<Props, {}> {
    render() {
        const { feed } = this.props
        const readable = feed.readable
        return (
            <div className={styles.feedCard}>
                {readable ? <FeedReadable readable={readable} /> : null}
            </div>
        )
    }
}
