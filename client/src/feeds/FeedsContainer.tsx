import * as React from "react"
import { connect, Dispatch } from "react-redux"

import { State } from "../Store"
import * as FeedsActions from "./FeedsActions"
import FeedsList from "./components/FeedsList"
import Header from "../components/Header"

interface Props extends State {
    onLoad: () => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        this.props.onLoad()
    }
    render() {
        return (
            <div style={{ flex: 1 }}>
                <Header />
                <FeedsList feeds={this.props.feeds.feeds} />
            </div>
        )
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
