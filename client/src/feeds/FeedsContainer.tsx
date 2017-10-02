import * as React from "react"
import { connect, Dispatch } from "react-redux"

import { GlobalState } from "../app/AppState"
import * as FeedsActions from "./FeedsActions"
import FeedsList from "./components/FeedsList"
import HeaderContainer from "../app/HeaderContainer"

interface Props extends GlobalState {
    onLoad: () => void
}

class FeedsContainer extends React.Component<Props, {}> {
    componentWillMount() {
        this.props.onLoad()
    }
    render() {
        return (
            <div style={{ flex: 1 }}>
                <HeaderContainer />
                <FeedsList feeds={this.props.feeds.feeds} />
            </div>
        )
    }
}

const mapStateToProps = (state: GlobalState) => {
    return {
        ...state
    }
}

const mapDispatchToProps = (dispatch: Dispatch<GlobalState>, state: any) => {
    return {
        onLoad: () => {
            dispatch(FeedsActions.loadfeeds())
        }
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(FeedsContainer)
