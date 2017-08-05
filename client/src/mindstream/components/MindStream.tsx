import * as React from "react"
import { Feed, Reaction } from "../../feeds/Feed"
import FeedReadable from "../../components/FeedReadable"

interface Props {
    feed: Feed
}

export default class MindStream extends React.Component<Props, {}> {
    render() {
        const { feed } = this.props
        console.log("MindStream.render", feed)
        return (
            <div>
                {feed.readable ? <FeedReadable readable={feed.readable} /> : null}
            </div>
        )
    }
}
