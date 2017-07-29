import * as React from "react"

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
                <div>
                    <label>Email</label>
                </div>
                <div>
                    <input type="email" value={email} onChange={this.onChangeHandler("email")} />
                </div>

                <div>
                    <label>Password</label>
                </div>
                <div>
                    <input type="password" value={password} onChange={this.onChangeHandler("password")} />
                </div>
                <div>
                    <button onClick={this.onSubmitHandler}>Login</button>
                    <div>{loading ? "loading" : ""}</div>
                </div>
            </div>
        )
    }

    onChangeHandler = (field: string) => (value: React.FormEvent<HTMLInputElement>) => {
        const { onChange } = this.props
        onChange(field, (value.target as any).value)
    }

    onSubmitHandler = () => {
        const { email, password, loading, onSubmit } = this.props
        if (!loading) {
            onSubmit(email, password)
        }
    }
}
