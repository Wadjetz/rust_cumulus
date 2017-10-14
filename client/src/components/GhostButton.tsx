import * as React from "react"
import * as styles from "./GhostButton.css"
import LoaderIcon from "./icons/LoaderIcon"

interface Props {
    label: string
    loading?: boolean
    disable?: boolean
    onClick: () => void
}

export default class GhostButton extends React.Component<Props, {}> {
    static defaultProps = {
        loading: false,
        disable: false,
    }

    render() {
        const { label, loading } = this.props
        return (
            <div className={styles.ghostButton} onClick={this.onClickHandler}>
                {loading
                    ? <LoaderIcon className={styles.loaderSvg} width={34} height={34} color={"#4A90E2"} />
                    : <div className={styles.label}>{label}</div>}
            </div>
        )
    }

    onClickHandler = () => {
        const { loading, disable, onClick } = this.props
        if (!loading || !disable) {
            onClick()
        }
    }
}
