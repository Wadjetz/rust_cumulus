import * as React from "react"
import { Feed, Readable } from "../Feed"

interface Props {
    readable: Readable
}

export default class FeedReadable extends React.Component<Props, {}> {
    render() {
        const { readable } = this.props
        return (
            <div>
                <h1><a target="_blanc" href={readable.url}>{readable.title}</a></h1>
                {readable.leadImageUrl ? <img width="200" src={readable.leadImageUrl} /> : null}
                {/*<div dangerouslySetInnerHTML={{ __html: excerpt }} />*/}
                <p>{readable.excerpt}</p>
            </div>
        )
    }
}
