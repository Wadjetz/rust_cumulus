import * as React from "react"
import { connect, Dispatch } from "react-redux"
import { SignupState } from "./SignupReducer"
import * as SignupActions from "./SignupActions"
import { GlobalState } from "../app/AppState"
import SignupForm from "./components/SignupForm"
import * as styles from "./components/Signup.css"

interface State {
    signup: SignupState
}

interface DispatchProps {
    onChange(field: string, value: string): void
    onSubmit(login: string, email: string, password: string): void
}

type Props = State & DispatchProps

const SignupContainer = (props: Props) => {
    const { signup, onChange, onSubmit } = props
    const { login, email, password, loading, error } = signup
    return (
        <div className={styles.container}>
            <h2 className={styles.appName}>Mindstream</h2>
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

const mapDispatchToProps = (dispatch: Dispatch<GlobalState>): DispatchProps => {
    return {
        onChange: (field, value) =>
            dispatch(SignupActions.signupOnChange(field, value)),
        onSubmit: (login, email, password) =>
            dispatch(SignupActions.signupOnSubmit(login, email, password)),
    }
}

const mapStateToProps = (state: GlobalState): State => {
    return {
        signup: state.signup
    }
}
export default connect(mapStateToProps, mapDispatchToProps)(SignupContainer)
