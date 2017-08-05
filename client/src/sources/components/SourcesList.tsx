import * as React from "react"
import { Source } from "../../sources/Source"
import FeedReadable from "../../components/FeedReadable"

interface Props {
    sources: Source[]
}

export default class SourcesList extends React.Component<Props, {}> {
    render() {
        const { sources } = this.props
        console.log("SourcesList.render", sources)
        return (
            <div>
                {sources.length}
            </div>
        )
    }
}
