import * as React from "react"
import { connect, Dispatch } from "react-redux"

import { State } from "../Store"
import * as SourcesActions from "./SourcesActions"
import { Source } from "./Source"
import SourcesList from "./components/SourcesList"
import AddSourceForm from "./components/AddSourceForm"

interface Props extends State {
    onLoad: () => void
    addSourceOnChange: (field: "newSourceUrl") => (value: any) => void
    addSourceOnSubmit: (sourceUrl: string) => void,
    fallowSource: (source: Source) => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.sources.sources.length === 0) {
            this.props.onLoad()
        }
    }
    render() {
        const { sources, addSourceOnChange, addSourceOnSubmit } = this.props
        return (
            <div>
                <AddSourceForm
                    newSourceUrl={sources.newSourceUrl}
                    loading={false}
                    onChange={addSourceOnChange}
                    onSubmit={addSourceOnSubmit}
                />
                {this.renderSourceList()}
            </div>
        )
    }

    renderSourceList = () => {
        const { fallowSource, sources } = this.props
        if (sources.sources.length > 0) {
            return (
                <SourcesList
                    sources={sources.sources}
                    fallowSource={fallowSource}
                />
            )
        } else {
            return <div>Empty</div>
        }
    }
}

const mapDispatchToProps = (dispatch: Dispatch<State>, state: any) => {
    return {
        addSourceOnChange: (field: string, value: string) => {
            dispatch(SourcesActions.addSourceOnChange(field, value))
        },
        addSourceOnSubmit: (sourceUrl: string) => {
            dispatch(SourcesActions.addSource(sourceUrl))
        },
        onLoad: () => {
            dispatch(SourcesActions.loadUnfollowedSources())
        },
        fallowSource: (source: Source) => {
            dispatch(SourcesActions.fallowSources(source))
        }
    }
}

export default connect((state: State) => state, mapDispatchToProps)(FeedsContainer)
