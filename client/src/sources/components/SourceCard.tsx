import * as React from "react"
import { Source } from "../../sources/Source"
import FeedReadable from "../../components/FeedReadable"

interface Props {
    source: Source
    fallowSource: (source: Source) => void
}

export default class SourcesCard extends React.Component<Props, {}> {
    render() {
        const { source, fallowSource } = this.props
        console.log("SourcesCard.render", source)
        return (
            <div>
                <h4>{source.rssSource.title}</h4>
                {source.rssSource.xmlUrl}
                <button onClick={() => fallowSource(source)}>Fallow</button>
            </div>
        )
    }
}
