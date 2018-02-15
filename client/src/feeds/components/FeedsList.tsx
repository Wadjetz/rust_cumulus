import * as React from "react"
import { FeedSimple } from "../Feed"
import FeedCard from "./FeedCard"

interface Props {
    feeds: FeedSimple[]
}

export default class FeedsList extends React.PureComponent<Props> {
    render() {
        const { feeds } = this.props
        return (
            <div>
                {feeds.map(feed => <FeedCard key={feed.uuid} feed={feed} />)}
            </div>
        )
    }
}
