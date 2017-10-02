import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { SignupState } from "./SignupReducer"
import * as SignupActions from "./SignupActions"
import { GlobalState } from "../app/AppState"
import SignupForm from "./components/SignupForm"
import * as styles from "./components/Signup.css"

interface Props extends GlobalState {
    onChange: (field: keyof SignupState) => void
    onSubmit: (login: string, email: string, password: string) => () => void
}
const SignupContainer = (props: Props) => {
    const { signup, onChange, onSubmit } = props
    const { login, email, password, loading, error } = signup
    return (
        <div className={styles.container}>
            <h2 className={styles.appName}>Cumulus</h2>
            <SignupForm
                login={login}
                email={email}
                password={password}
                loading={loading}
                error={error}
                onChange={onChange}
                onSubmit={onSubmit}
            />
            <a href="#/login">Login</a>
        </div>
    )
}

const mapDispatchToProps = (dispatch: Dispatch<GlobalState>) => {
    return {
        onChange: (field: string, value: string) => {
            dispatch(SignupActions.signupOnChange(field, value))
        },
        onSubmit: (login: string, email: string, password: string) => {
            dispatch(SignupActions.signupOnSubmit(login, email, password))
        }
    }
}

const mapStateToProps = (state: GlobalState) => state
export default connect(mapStateToProps, mapDispatchToProps)(SignupContainer)
