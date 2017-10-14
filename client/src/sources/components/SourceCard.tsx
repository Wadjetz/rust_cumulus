import * as React from "react"
import { Source } from "../../sources/Source"

interface Props {
    source: Source
    count?: number
    fallowSource?: (source: Source) => void
}

export default class SourcesCard extends React.Component<Props, {}> {
    render() {
        const { source, fallowSource, count = 0 } = this.props
        return (
            <div>
                <h4><a href={`#/stream/${source.uuid}`}>{source.rssSource!.title}</a></h4>
                {count}
                {fallowSource ? <button onClick={this.fallowSourceHandler}>Fallow</button> : null }
            </div>
        )
    }

    fallowSourceHandler = () => {
        const { source, fallowSource } = this.props
        if (fallowSource) {
            fallowSource(source)
        }
    }
}
