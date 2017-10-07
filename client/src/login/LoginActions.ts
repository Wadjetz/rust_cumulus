export type LOGIN_ON_CHANGE = {
    type: "LOGIN_ON_CHANGE"
    field: string
    value: string
}
export function loginOnChange(field: string, value: string) {
    return { type: "LOGIN_ON_CHANGE", field, value }
}

export type LOGIN_ON_SUBMIT = {
    type: "LOGIN_ON_SUBMIT"
    email: string
    password: string
}
export function loginOnSubmit(email: string, password: string) {
    return { type: "LOGIN_ON_SUBMIT", email, password }
}

export type LOGIN_ON_SUBMIT_SUCCESS = {
    type: "LOGIN_ON_SUBMIT_SUCCESS"
}
export function loginOnSubmitSuccess() {
    return { type: "LOGIN_ON_SUBMIT_SUCCESS" }
}

export type LOGIN_ON_SUBMIT_ERROR = {
    type: "LOGIN_ON_SUBMIT_ERROR"
    error: any
}
export function loginOnSubmitError(error: any) {
    return { type: "LOGIN_ON_SUBMIT_ERROR", error }
}

export type LoginAction = LOGIN_ON_CHANGE | LOGIN_ON_SUBMIT | LOGIN_ON_SUBMIT_SUCCESS | LOGIN_ON_SUBMIT_ERROR
