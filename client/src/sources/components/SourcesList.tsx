import * as React from "react"
import * as styles from "./SourcesList.css"
import { Source, SourceStat } from "../../sources/Source"
import SourcesCard from "../components/SourceCard"

interface Props {
    sources: Source[]
    mySourcesStats?: SourceStat[]
    fallowSource?(source: Source): void
}

export default class SourcesList extends React.PureComponent<Props> {
    render() {
        const { sources, fallowSource, mySourcesStats } = this.props
        return (
            <div className={styles.sourcesList}>
                {sources.map(source => {
                    const stats = mySourcesStats && mySourcesStats.find(s => s.uuid === source.uuid)
                    return <SourcesCard count={stats && stats.count} key={source.uuid} source={source} fallowSource={fallowSource}/>
                })}
            </div>
        )
    }
}
