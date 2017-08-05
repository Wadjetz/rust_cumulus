import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { Action } from "redux"

import * as Api from "../Api"
import { State } from "../Store"

import { SourcesState } from "./SourcesReducer"
import * as SourcesActions from "./SourcesActions"
import { Source } from "./Source"

import SourcesList from "./components/SourcesList"

interface Props extends State {
    onLoad: (token: string) => void
    fallowSource: (token: string) => (source: Source) => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.sources.sources.length === 0) {
            this.props.onLoad(this.props.login.token)
        }
    }
    render() {
        const { fallowSource, sources } = this.props
        console.log("SourceContainer.render", this.props)
        return <SourcesList sources={sources.sources} fallowSource={fallowSource(this.props.login.token)} />
    }
}

const mapDispatchToProps = (dispatch: Dispatch<State>, state: any) => {
    return {
        onLoad: (token: string) => {
            dispatch(SourcesActions.sourcesOnLoad())
            Api.loadUnfollowedSources(token).then(sources => {
                console.log("loadUnfollowedSources", sources)
                dispatch(SourcesActions.sourcesOnLoadSuccess(sources))
            }).catch(error => {
                dispatch(SourcesActions.sourcesOnLoadError(error))
            })
        },
        fallowSource: (token: string) => (source: Source) => {
            console.log("fallowSource", source)
            dispatch(SourcesActions.fallowSourcesOnLoad())
            Api.fallowSource(token, source).then(() => {
                console.log("fallowSource")
                dispatch(SourcesActions.fallowSourcesOnLoadSuccess(source))
            }).catch(error => {
                dispatch(SourcesActions.fallowSourcesOnLoadError(error))
            })
        }
    }
}

export default connect((state: State) => state, mapDispatchToProps)(FeedsContainer)

