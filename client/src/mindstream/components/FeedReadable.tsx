import * as React from "react"
import * as styles from "./FeedReadable.css"

interface Props {
    title: string
    url: string
    leadImageUrl?: string
    content: string
}

export default class FeedReadable extends React.PureComponent<Props> {
    render() {
        const { url, title, content, leadImageUrl } = this.props
        return (
            <div className={styles.container}>
                <div className={styles.feed_readable}>
                    <a className={styles.title} target="_blanc" href={url}>{title}</a>
                    <div className={styles.url}>{url}</div>
                    {leadImageUrl ? <img src={leadImageUrl} /> : null}
                    {<div dangerouslySetInnerHTML={{ __html: content }} />}
                </div>
            </div>
        )
    }
}
