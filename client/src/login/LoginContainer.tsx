import * as React from "react"
import { connect, Dispatch } from "react-redux"
import * as LoginActions from "./LoginActions"
import { GlobalState } from "../app/AppState"
import LoginForm from "./components/LoginForm"
import * as styles from "./components/Login.css"

interface Props extends GlobalState {
    onChange(field: string): (value: string) => void
    onSubmit(email: string, password: string): () => void
}
const LoginContainer = (props: Props) => {
    const { login, onChange, onSubmit } = props
    const { email, password, loading, error } = login
    return (
        <div className={styles.container}>
            <h2 className={styles.appName}>Mindstream</h2>
            <LoginForm
                email={email}
                password={password}
                loading={loading}
                error={error}
                onChange={onChange}
                onSubmit={onSubmit}
            />
            <a href="#/signup">Signup</a>
        </div>
    )
}

const mapDispatchToProps = (dispatch: Dispatch<GlobalState>) => {
    return {
        onChange: (field: string, value: string) => {
            dispatch(LoginActions.loginOnChange(field, value))
        },
        onSubmit: (email: string, password: string) => {
            dispatch(LoginActions.loginOnSubmit(email, password))
        }
    }
}

const mapStateToProps = (state: GlobalState) => state
export default connect(mapStateToProps, mapDispatchToProps)(LoginContainer)
