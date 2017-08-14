import * as React from "react"
import Input from "../components/Input"

interface Props {
    email: string
    password: string
    loading: boolean
    onChange: (field: string, value: string) => void
    onSubmit: (email: string, password: string) => void
}

export default class LoginForm extends React.Component<Props, {}> {
    render() {
        const { email, password, loading } = this.props
        return (
            <div>
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
                <div>
                    <button onClick={this.onSubmitHandler}>Login</button>
                    <div>{loading ? "loading" : ""}</div>
                </div>
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
