import * as React from "react"
import * as styles from "./GhostButton.css"

interface Props {
    label: string
    loading?: boolean
    disable?: boolean
    onClick: () => void
}

export default class GhostButton extends React.Component<Props, {}> {
    defaultProps = {
        loading: false,
        disable: false,
    }

    render() {
        const { label, loading } = this.props
        return (
            <div className={styles.container} onClick={this.onClickHandler}>
                {loading
                    ? <div className={styles.loading}>Loading</div>
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
