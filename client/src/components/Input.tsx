import * as React from "react"
import * as styles from "./Input.css"

interface Props {
    label: string
    value: string
    error?: string
    type: string
    onChange(value: string): void
}

export default class Input extends React.PureComponent<Props> {
    render() {
        const { label, type, error, value } = this.props
        return (
            <div className={styles.container}>
                <label className={styles.label}>{label}</label>
                <input className={styles.input} type={type} value={value} onChange={this.onChangeHandler} />
                <span>{error}</span>
            </div>
        )
    }

    onChangeHandler = (event: React.FormEvent<HTMLInputElement>) => this.props.onChange(event.currentTarget.value)
}
