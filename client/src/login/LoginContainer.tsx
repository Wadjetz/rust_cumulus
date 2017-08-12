import * as React from "react"
import { connect, Dispatch } from "react-redux"
import * as Api from "../Api"
import { LoginState } from "./LoginReducer"
import * as LoginActions from "./LoginActions"
import { State } from "../Store"
import LoginForm from "./LoginForm"

interface Props extends State {
    onChange: (field: keyof LoginState) => (value: any) => void
    onSubmit: (email: string, password: string) => () => void
}
const LoginContainer = (props: Props) => {
    const { login, onChange, onSubmit } = props
    const { email, password, loading } = login
    return <LoginForm
        email={email}
        password={password}
        loading={loading}
        onChange={onChange}
        onSubmit={onSubmit}
    />
}

const mapStateToProps = (state: State) => {
    return {
        ...state
    }
}

const mapDispatchToProps = (dispatch: Dispatch<State>) => {
    return {
        onChange: (field: string, value: string) => {
            dispatch(LoginActions.loginOnChange(field, value))
        },
        onSubmit: (email: string, password: string) => {
            // TODO validate data
            dispatch(LoginActions.loginOnSubmit())
            Api.login(email, password).then(response => {
                dispatch(LoginActions.loginOnSubmitSuccess(response.login))
            }).catch(error => {
                dispatch(LoginActions.loginOnSubmitError(error))
            })
        }
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(LoginContainer)
