import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { Action } from "redux"

import * as Api from "../Api"
import { State } from "../Store"

import { SourcesState } from "./SourcesReducer"
import * as SourcesActions from "./SourcesActions"

import SourcesList from "./components/SourcesList"

interface Props extends State {
    onLoad: (token: string) => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.sources.sources.length === 0) {
            this.props.onLoad(this.props.login.token)
        }
    }
    render() {
        console.log("FeedsContainer.render", this.props.sources)
        return <SourcesList sources={this.props.sources.sources} />
    }
}

const mapDispatchToProps = (dispatch: Dispatch<State>, state: any) => {
    return {
        onLoad: (token: string) => {
            dispatch(SourcesActions.sourcesOnLoad())
            Api.loadSources(token).then(sources => {
                dispatch(SourcesActions.sourcesOnLoadSuccess(sources))
            }).catch(error => {
                dispatch(SourcesActions.sourcesOnLoadError(error))
            })
        }
    }
}

export default connect((state: State) => state, mapDispatchToProps)(FeedsContainer)

