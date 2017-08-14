import * as React from "react"
import { connect, Dispatch } from "react-redux"
import * as ReactCSSTransitionGroup from "react-addons-css-transition-group"

import * as styles from "./MindStreamContainer.css"
import { State } from "../Store"
import { Feed, Reaction } from "../feeds/Feed"
import * as MindStreamActions from "./MindStreamActions"
import MindStream from "./components/MindStream"

interface Props extends State {
    onReaction: (feed: Feed, reaction: Reaction) => () => void
    loadUnreadedFeeds: () => void
}

class MindStreamContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.feeds.feeds.length === 0) {
            this.props.loadUnreadedFeeds()
        }
        document.addEventListener("keydown", this.onKeyPressHandler, false)
    }

    componentWillUnMount() {
        document.removeEventListener("keydown", this.onKeyPressHandler, false)
    }

    render() {
        const { mindStream, onReaction } = this.props
        if (mindStream.feeds.length > 0) {
            const feed = mindStream.feeds[0]
            return (
                <div>
                    <button onClick={onReaction(feed, "Readed")}>Read</button>
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
                        <MindStream
                            key={feed.uuid}
                            feed={feed}
                        />
                    </ReactCSSTransitionGroup>
                </div>
            )
        } else if (mindStream.loading) {
            return <div>Loading</div>
        } else {
            return <div>No more feeds</div>
        }
    }

    onKeyPressHandler = (event: any) => {
        const { mindStream: { feeds }, onReaction } = this.props
        if (feeds.length > 0 && event.code === "ArrowRight" || event.code === "KeyD") {
            onReaction(feeds[0], "Readed")()
        }
    }
}

const mapDispatchToProps = (dispatch: Dispatch<State>, state: any) => {
    return {
        loadUnreadedFeeds: () => {
            dispatch(MindStreamActions.loadUnreadedFeeds())
        },
        onReaction: (feed: Feed, reaction: Reaction) => () => {
            dispatch(MindStreamActions.readFeed(feed, reaction))
        }
    }
}

export default connect((state: State) => state, mapDispatchToProps)(MindStreamContainer)

