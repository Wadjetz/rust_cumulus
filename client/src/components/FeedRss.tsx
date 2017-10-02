import * as React from "react"
import * as styles from "./FeedRss.css"
import { Rss } from "../feeds/Feed"

interface Props {
    rss: Rss
    feed_url: string
}

export default class FeedRss extends React.Component<Props, {}> {
    render() {
        const { feed_url, rss: { url, title, content, summary } } = this.props
        return (
            <div className={styles.container}>
                <div className={styles.feed_rss}>
                    <a className={styles.title} target="_blanc" href={url || feed_url}>{title}</a>
                    <div>{url || feed_url}</div>
                    {<div dangerouslySetInnerHTML={{ __html: content || summary || "" }} />}
                </div>
            </div>
        )
    }
}
