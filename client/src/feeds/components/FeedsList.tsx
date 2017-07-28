import * as React from "react"
import { Feed } from "../Feed"
import FeedCard from "./FeedCard"

interface Props {
    feeds: Feed[]
}

export default class FeedsList extends React.Component<Props, {}> {
    render() {
        const { feeds } = this.props
        return (
            <div>
                {feeds.map(feed => <FeedCard key={feed.uuid} feed={feed} /> )}
            </div>
        )
    }
}
