import * as React from "react"
import * as styles from "./FeedRss.css"
import { Rss } from "../feeds/Feed"

interface Props {
    rss: Rss
}

export default class FeedRss extends React.Component<Props, {}> {
    render() {
        const { url, title, content, summary } = this.props.rss
        return (
            <div className={styles.container}>
                <div className={styles.feed_rss}>
                    <a className={styles.title} target="_blanc" href={url}>{title}</a>
                    {<div dangerouslySetInnerHTML={{ __html: content || summary }} />}
                </div>
            </div>
        )
    }
}
