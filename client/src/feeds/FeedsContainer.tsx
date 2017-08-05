import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { Action } from "redux"

import * as Api from "../Api"
import { State } from "../Store"

import { FeedsState } from "./FeedsReducer"
import * as FeedsActions from "./FeedsActions"

import FeedsList from "./components/FeedsList"

interface Props extends State {
    onLoad: (token: string) => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.feeds.feeds.length === 0) {
            this.props.onLoad(this.props.login.token)
        }
    }
    render() {
        console.log("FeedsContainer.render", this.props.feeds.feeds)
        return <FeedsList feeds={this.props.feeds.feeds} />
    }
}

const mapStateToProps = (state: State) => {
    return {
        ...state
    }
}

const mapDispatchToProps = (dispatch: Dispatch<State>, state: any) => {
    return {
        onLoad: (token: string) => {
            dispatch(FeedsActions.feedsOnLoad())
            Api.loadUnreadedFeeds(token).then(feeds => {
                dispatch(FeedsActions.feedsOnLoadSuccess(feeds))
            }).catch(error => {
                dispatch(FeedsActions.feedsOnLoadError(error))
            })
        }
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(FeedsContainer)

