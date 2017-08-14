import * as React from "react"
import * as styles from "./Input.css"

interface Props {
    label: string
    value: string
    type: string
    onChange: (value: string) => void
}

export default class Input extends React.Component<Props, {}> {
    render() {
        const { label, type, value } = this.props
        return (
            <div className={styles.container}>
                <div>
                    <label>{label}</label>
                </div>
                <div>
                    <input type={type} value={value} onChange={this.onChangeHandler} />
                </div>
            </div>
        )
    }

    onChangeHandler = (value: React.FormEvent<HTMLInputElement>) => {
        this.props.onChange((value.target as any).value)
    }
}
