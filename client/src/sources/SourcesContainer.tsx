import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { Action } from "redux"

import * as Api from "../Api"
import { State } from "../Store"

import { SourcesState } from "./SourcesReducer"
import * as SourcesActions from "./SourcesActions"
import { Source } from "./Source"

import SourcesList from "./components/SourcesList"
import AddSourceForm from "./components/AddSourceForm"

interface Props extends State {
    onLoad: () => void
    addSourceOnChange: (field: "newSourceUrl") => (value: any) => void
    addSourceOnSubmit: (sourceUrl: string) => void,
    fallowSource: (token: string) => (source: Source) => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.sources.sources.length === 0) {
            this.props.onLoad()
        }
    }
    render() {
        const { fallowSource, sources, addSourceOnChange, addSourceOnSubmit } = this.props
        console.log("SourceContainer.render", this.props)
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
        const { fallowSource, sources, addSourceOnChange } = this.props
        if (sources.sources.length > 0) {
            return (
                <SourcesList
                    sources={sources.sources}
                    fallowSource={fallowSource(this.props.login.token)}
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
            dispatch(SourcesActions.addSourceOnLoad(sourceUrl))
        },
        onLoad: () => {
            dispatch(SourcesActions.sourcesOnLoad())
        },
        fallowSource: (token: string) => (source: Source) => {
            dispatch(SourcesActions.fallowSourcesOnLoad())
            Api.fallowSource(token, source).then(() => {
                dispatch(SourcesActions.fallowSourcesOnLoadSuccess(source))
            }).catch(error => {
                dispatch(SourcesActions.fallowSourcesOnLoadError(error))
            })
        }
    }
}

export default connect((state: State) => state, mapDispatchToProps)(FeedsContainer)

