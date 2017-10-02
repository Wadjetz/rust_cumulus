import * as React from "react"
import { connect, Dispatch } from "react-redux"
import * as ReactCSSTransitionGroup from "react-addons-css-transition-group"

import * as styles from "./components/MindStream.css"
import { State } from "../Store"
import { Feed, Reaction } from "../feeds/Feed"
import * as MindStreamActions from "./MindStreamActions"
import MindStreamCard from "./components/MindStreamCard"
import Header from "../components/Header"

interface DispatchProps {
    onReaction: (feed: Feed, reaction: Reaction) => () => void
    loadData: () => void
}

interface StateProps {
    feeds: Feed[]
    loading: boolean
}

type Props = StateProps & DispatchProps

class MindStreamContainer extends React.PureComponent<Props> {
    componentWillMount() {
        if (this.props.feeds.length === 0) {
            this.props.loadData()
        }
        document.addEventListener("keydown", this.onKeyPressHandler, false)
    }

    componentWillUnMount() {
        document.removeEventListener("keydown", this.onKeyPressHandler, false)
    }

    render() {
        return (
            <div className={styles.container}>
                <Header />
                {this.renderStream()}
            </div>
        )
    }

    renderStream = () => {
        const { feeds, loading, onReaction } = this.props
        if (feeds.length > 0) {
            const feed = feeds[0]
            return (
                <div>
                    <button disabled={loading} onClick={onReaction(feed, "Readed")}>Read</button>
                    <button disabled={loading} onClick={onReaction(feed, "Liked")}>Liked</button>
                    <ReactCSSTransitionGroup
                        transitionName={{
                            enter: styles.transitionEnter,
                            enterActive: styles.transitionEnterActive,
                            leave: styles.transitionLeave,
                            leaveActive: styles.transitionLeaveActive,
                        }}
                        transitionEnterTimeout={400}
                        transitionLeaveTimeout={0}
                        key={feed.uuid}>
                        <MindStreamCard
                            key={feed.uuid}
                            feed={feed}
                        />
                    </ReactCSSTransitionGroup>
                </div>
            )
        } else if (loading) {
            return <div>Loading</div>
        } else {
            return <div>No more feeds</div>
        }
    }

    onKeyPressHandler = (event: any) => {
        const { feeds, onReaction } = this.props
        if (feeds.length > 0 && event.code === "ArrowRight" || event.code === "KeyD") {
            onReaction(feeds[0], "Readed")()
        }
    }
}

const mapStateToProps = (state: State): StateProps => {
    return {
        feeds: state.mindStream.feeds,
        loading: state.mindStream.loading,
    }
}

const mapDispatchToProps = (dispatch: Dispatch<State>): DispatchProps => {
    return {
        loadData: () => dispatch(MindStreamActions.loadUnreadedFeeds()),
        onReaction: (feed, reaction) => () => dispatch(MindStreamActions.readFeed(feed, reaction))
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(MindStreamContainer)
