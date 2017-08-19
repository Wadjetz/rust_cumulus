import * as React from "react"
import * as styles from "./LoginForm.css"
import Input from "../../components/Input"
import GhostButton from "../../components/GhostButton"
import { ApiError } from "../../Api"

interface Props {
    email: string
    password: string
    loading: boolean
    error?: ApiError
    onChange: (field: string, value: string) => void
    onSubmit: (email: string, password: string) => void
}

export default class LoginForm extends React.Component<Props, {}> {
    render() {
        const { email, password, loading } = this.props
        return (
            <div className={styles.container}>
                <Input
                    label="Email"
                    value={email}
                    onChange={this.onChangeHandler("email")}
                    type="email"
                />

                <Input
                    label="Password"
                    value={password}
                    onChange={this.onChangeHandler("password")}
                    type="password"
                />

                <GhostButton
                    label="Login"
                    loading={loading}
                    onClick={this.onSubmitHandler}
                />

                {this.renderError()}
            </div>
        )
    }

    renderError = () => {
        const { error } = this.props
        return (
            <div className={styles.errorContainer}>
                {error ? <div className={styles.errorMessage}>{error.errors.map(e => e.message).join(", ")}</div> : null }
            </div>
        )
    }

    onChangeHandler = (field: string) => (value: string) => {
        const { onChange } = this.props
        onChange(field, value)
    }

    onSubmitHandler = () => {
        const { email, password, loading, onSubmit } = this.props
        if (!loading) {
            onSubmit(email, password)
        }
    }
}
