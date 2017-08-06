import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { Action } from "redux"
import { Feed, Reaction } from "../feeds/Feed"
import * as ReactCSSTransitionGroup from "react-addons-css-transition-group"
import * as styles from "./MindStreamContainer.css"

import * as Api from "../Api"
import { State } from "../Store"


import { MindStreamState } from "./MindStreamReducer"
import * as MindStreamActions from "./MindStreamActions"

import MindStream from "./components/MindStream"

interface Props extends State {
    onReaction: (token: string, feed: Feed, reaction: Reaction) => () => void
    onLoad: (token: string) => void
}

class MindStreamContainer extends React.Component<Props, {}> {
    componentWillMount() {
        if (this.props.feeds.feeds.length === 0) {
            this.props.onLoad(this.props.login.token)
        }
    }
    render() {
        const { mindStream, login, onReaction } = this.props
        console.log("MindStreamContainer.render", this.props)
        if (mindStream.feeds.length > 0) {
            const feed = mindStream.feeds[0]
            return (
                <div>
                    <button onClick={onReaction(login.token, feed, "Readed")}>Read</button>
                    <ReactCSSTransitionGroup
                        transitionName={{
                            enter: styles.transitionEnter,
                            enterActive: styles.transitionEnterActive,
                            leave: styles.transitionLeave,
                            leaveActive: styles.transitionLeaveActive,
                        }}
                        transitionEnterTimeout={400}
                        transitionLeaveTimeout={0}>
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
}

const mapDispatchToProps = (dispatch: Dispatch<State>, state: any) => {
    return {
        onLoad: (token: string) => {
            dispatch(MindStreamActions.mindStreamOnLoad())
            Api.loadUnreadedFeeds(token).then(feeds => {
                dispatch(MindStreamActions.mindStreamOnLoadSuccess(feeds))
            }).catch(error => {
                dispatch(MindStreamActions.mindStreamOnLoadError(error))
            })
        },
        onReaction: (token: string, feed: Feed, reaction: Reaction) => () => {
            dispatch(MindStreamActions.readFeedOnLoad())
            Api.readFeed(token, feed, reaction).then((r) => {
                console.log('readFeed', r)
                dispatch(MindStreamActions.readFeedOnLoadSuccess(feed))
            }).catch(error => {
                dispatch(MindStreamActions.readFeedOnLoadError(error))
            })
        }
    }
}

export default connect((state: State) => state, mapDispatchToProps)(MindStreamContainer)

