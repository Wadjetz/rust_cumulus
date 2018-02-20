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
    onReaction(feed: Feed, reaction: Reaction, sourceUuid?: string): () => void
    loadUnreadedFeeds(): void
    loadUnreadedFeedsBySource(sourceUuid: string): void
    onNextFeed(feed: Feed, sourceUuid: string | undefined): void
    onPreviousFeed(sourceUuid: string | undefined): void
}

interface StateProps {
    feeds: Feed[]
    loading: boolean
    nextFeedLoader: boolean
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
        document.removeEventListener("keydown", this.onKeyPressHandler as any, false) // TODO remove any
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
        const { feeds, loading, nextFeedLoader, sourceUuid, onReaction, onNextFeed, onPreviousFeed } = this.props
        if (feeds.length > 0) {
            const feed = feeds[0]
            return (
                <div>
                    <FeedActions
                        feed={feed}
                        loading={loading}
                        nextFeedLoader={nextFeedLoader}
                        sourceUuid={sourceUuid}
                        onNextFeed={onNextFeed}
                        onPreviousFeed={onPreviousFeed}
                        onReaction={onReaction}
                    />
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

    onKeyPressHandler = (event: KeyboardEvent) => {
        const { feeds, onNextFeed, onPreviousFeed, sourceUuid, nextFeedLoader } = this.props
        if (!nextFeedLoader && feeds.length > 0 && event.code === "ArrowRight" || event.code === "KeyD") {
            onNextFeed(feeds[0], sourceUuid)
        } else if (feeds.length > 0 && event.code === "ArrowLeft" || event.code === "KeyQ" || event.code === "KeyA") {
            onPreviousFeed(sourceUuid)
        }
    }
}

const mapStateToProps = (state: GlobalState, props?: { match?: RouterMatch<Params> }): StateProps & Params => {
    const sourceUuid = props && props.match && props.match.params && props.match.params.sourceUuid
    return {
        sourceUuid,
        feeds: state.mindStream.feeds,
        loading: state.mindStream.loading,
        nextFeedLoader: state.mindStream.nextFeedLoader
    }
}

const mapDispatchToProps = (dispatch: Dispatch<GlobalState>): DispatchProps => {
    return {
        loadUnreadedFeeds: () => dispatch(MindStreamActions.loadUnreadedFeeds()),
        loadUnreadedFeedsBySource: (sourceUuid: string) => dispatch(MindStreamActions.loadUnreadedFeedsBySource(sourceUuid)),
        onReaction: (feed, reaction, sourceUuid?: string) => () => dispatch(MindStreamActions.readFeed(feed, reaction, sourceUuid)),
        onNextFeed: (feed, sourceUuid: string | undefined) => dispatch(MindStreamActions.nextFeed(feed, sourceUuid)),
        onPreviousFeed: (sourceUuid: string | undefined) => dispatch(MindStreamActions.previousFeed(sourceUuid)),
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(MindStreamContainer)
