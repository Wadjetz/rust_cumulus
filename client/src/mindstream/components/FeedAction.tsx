import * as React from "react"
import * as styles from "./FeedAction.css"
import BaseButton from "../../components/BaseButton"
import LoaderIcon from "../../components/icons/LoaderIcon"
import classNames from "../../utils/classNames"

interface FeedActionProps {
    name: string
    loading: boolean
    className: string
    onClick(): void
}

export default class FeedAction extends React.PureComponent<FeedActionProps> {
    render() {
        const { name, loading, className, onClick } = this.props
        const classes = classNames({
            [styles.action]: true,
            [className]: true
        })
        return (
            <BaseButton className={classes} loading={loading} onClick={onClick} renderLoader={this.renderLoader}>
                {name}
            </BaseButton>
        )
    }

    renderLoader = () => <LoaderIcon width={34} height={34} color={"#4A90E2"} />
}
