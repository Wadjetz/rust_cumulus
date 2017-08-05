import * as React from "react"
import { Source } from "../../sources/Source"
import SourcesCard from "../components/SourceCard"

interface Props {
    sources: Source[]
    fallowSource: (source: Source) => void
}

export default class SourcesList extends React.Component<Props, {}> {
    render() {
        const { sources, fallowSource } = this.props
        console.log("SourcesList.render", sources)
        return (
            <div>
                {sources.map(source => <SourcesCard key={source.uuid} source={source} fallowSource={fallowSource}/>)}
            </div>
        )
    }
}
