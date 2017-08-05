import * as React from "react"
import * as styles from "./FeedReadable.css"
import { Feed, Readable } from "../feeds/Feed"

interface Props {
    readable: Readable
}

export default class FeedReadable extends React.Component<Props, {}> {
    render() {
        const { url, title, content, excerpt, leadImageUrl } = this.props.readable
        return (
            <div className={styles.container}>
                <div className={styles.feed_readable}>
                    <a className={styles.title} target="_blanc" href={url}>{title}</a>
                    {leadImageUrl ? <img src={leadImageUrl} /> : null}
                    {<div dangerouslySetInnerHTML={{ __html: content }} />}
                </div>
            </div>
        )
    }
}
