import * as React from "react"
import * as styles from "./IconButton.css"
import LoaderIcon from "./icons/LoaderIcon"
import AddIcon from "./icons/AddIcon"

type IconButtonType = "Add"

interface Props {
    loading?: boolean
    disable?: boolean
    type: IconButtonType
    onClick(): void
}

export default class IconButton extends React.PureComponent<Props> {
    static defaultProps = {
        loading: false,
        disable: false,
    }

    render() {
        const { loading } = this.props
        return (
            <div className={styles.iconButton} onClick={this.onClickHandler}>
                {loading
                    ? <LoaderIcon className={styles.loaderSvg} width={24} height={24} color={"#4A90E2"} />
                    : this.renderIcon()}
            </div>
        )
    }

    renderIcon = () => {
        switch (this.props.type) {
            case "Add": return <AddIcon className={styles.icon} />
            default: return <AddIcon className={styles.icon} />
        }
    }

    onClickHandler = () => {
        const { loading, disable, onClick } = this.props
        if (!loading || !disable) {
            onClick()
        }
    }
}
