import * as React from "react"
import * as styles from "./GhostButton.css"
import LoaderIcon from "./icons/LoaderIcon"
import BaseButton from "./BaseButton"

interface Props {
    label: string
    loading?: boolean
    disable?: boolean
    onClick(): void
}

export default class GhostButton extends React.PureComponent<Props> {
    static defaultProps = {
        loading: false,
        disable: false,
    }

    render() {
        const { label, loading, disable, onClick } = this.props
        return (
            <BaseButton
                className={styles.ghostButton}
                loading={loading}
                disable={disable}
                onClick={onClick}
                renderLoader={this.renderLoader}
            >
                <div className={styles.label}>{label}</div>
            </BaseButton>
        )
    }

    renderLoader = () => <LoaderIcon width={34} height={34} color={"#4A90E2"} />
}
