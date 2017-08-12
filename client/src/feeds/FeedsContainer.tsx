import * as React from "react"
import { connect, Dispatch } from "react-redux"

import { State } from "../Store"
import * as FeedsActions from "./FeedsActions"
import FeedsList from "./components/FeedsList"

interface Props extends State {
    onLoad: () => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.feeds.feeds.length === 0) {
            this.props.onLoad()
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
        onLoad: () => {
            dispatch(FeedsActions.loadfeeds())
        }
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(FeedsContainer)
