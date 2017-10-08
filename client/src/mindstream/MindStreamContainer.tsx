import * as React from "react"
import { connect, Dispatch } from "react-redux"
import * as ReactCSSTransitionGroup from "react-addons-css-transition-group"
import { match as RouterMatch } from "react-router"

import * as styles from "./MindStream.css"
import { GlobalState } from "../app/AppState"
import { Feed, Reaction } from "../feeds/Feed"
import * as MindStreamActions from "./MindStreamActions"
import MindStreamCard from "./components/MindStreamCard"
import HeaderContainer from "../app/HeaderContainer"
import FeedActions from "./components/FeedActions"

interface DispatchProps {
    onReaction: (feed: Feed, reaction: Reaction, sourceUuid?: string) => () => void
    loadUnreadedFeeds: () => void
    loadUnreadedFeedsBySource: (sourceUuid: string) => void
}

interface StateProps {
    feeds: Feed[]
    loading: boolean
}

interface Params {
    sourceUuid?: string
}

type Props = StateProps & DispatchProps & Params

class MindStreamContainer extends React.PureComponent<Props> {
    componentWillMount() {
        const { sourceUuid, loadUnreadedFeedsBySource, loadUnreadedFeeds } = this.props
        if (sourceUuid) {
            loadUnreadedFeedsBySource(sourceUuid)
        } else {
            loadUnreadedFeeds()
        }
        document.addEventListener("keydown", this.onKeyPressHandler, false)
    }

    componentWillUnmount() {
        document.removeEventListener("keydown", this.onKeyPressHandler, false)
    }

    render() {
        return (
            <div className={styles.container}>
                <HeaderContainer />
                {this.renderStream()}
            </div>
        )
    }

    renderStream = () => {
        const { feeds, loading, onReaction, sourceUuid } = this.props
        if (feeds.length > 0) {
            const feed = feeds[0]
            return (
                <div>
                    <FeedActions feed={feed} loading={loading} onReaction={onReaction} sourceUuid={sourceUuid} />
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
        const { feeds, onReaction, sourceUuid } = this.props
        if (feeds.length > 0 && event.code === "ArrowRight" || event.code === "KeyD") {
            onReaction(feeds[0], "Viewed", sourceUuid)()
        }
    }
}

const mapStateToProps = (state: GlobalState, props?: { match?: RouterMatch<Params> }): StateProps & Params => {
    const sourceUuid = props && props.match && props.match.params && props.match.params.sourceUuid
    return {
        sourceUuid,
        feeds: state.mindStream.feeds,
        loading: state.mindStream.loading,
    }
}

const mapDispatchToProps = (dispatch: Dispatch<GlobalState>): DispatchProps => {
    return {
        loadUnreadedFeeds: () => dispatch(MindStreamActions.loadUnreadedFeeds()),
        loadUnreadedFeedsBySource: (sourceUuid: string) => {
            dispatch(MindStreamActions.loadUnreadedFeedsBySource(sourceUuid))
        },
        onReaction: (feed, reaction, sourceUuid?: string) => () => dispatch(MindStreamActions.readFeed(feed, reaction, sourceUuid)),
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(MindStreamContainer)
