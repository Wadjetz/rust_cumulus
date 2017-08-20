import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { LoginState } from "./LoginReducer"
import * as LoginActions from "./LoginActions"
import { State } from "../Store"
import LoginForm from "./components/LoginForm"
import * as styles from "./components/Login.css"

interface Props extends State {
    onChange: (field: keyof LoginState) => (value: any) => void
    onSubmit: (email: string, password: string) => () => void
}
const LoginContainer = (props: Props) => {
    const { login, onChange, onSubmit } = props
    const { email, password, loading, error } = login
    return (
        <div className={styles.container}>
            <h2 className={styles.appName}>Cumulus</h2>
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

const mapDispatchToProps = (dispatch: Dispatch<State>) => {
    return {
        onChange: (field: string, value: string) => {
            dispatch(LoginActions.loginOnChange(field, value))
        },
        onSubmit: (email: string, password: string) => {
            dispatch(LoginActions.loginOnSubmit(email, password))
        }
    }
}

const mapStateToProps = (state: State) => state
export default connect(mapStateToProps, mapDispatchToProps)(LoginContainer)
