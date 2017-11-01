export type SignupAction =
    SIGNUP_ON_CHANGE |
    SIGNUP_ON_SUBMIT |
    SIGNUP_ON_SUBMIT_SUCCESS |
    SIGNUP_ON_SUBMIT_ERROR

export type SIGNUP_ON_CHANGE = { type: "SIGNUP_ON_CHANGE", field: string, value: string }
export const signupOnChange = (field: string, value: string): SIGNUP_ON_CHANGE => ({ type: "SIGNUP_ON_CHANGE", field, value })

export type SIGNUP_ON_SUBMIT = { type: "SIGNUP_ON_SUBMIT", login: string, email: string, password: string}
export const signupOnSubmit = (login: string, email: string, password: string): SIGNUP_ON_SUBMIT => {
    return { type: "SIGNUP_ON_SUBMIT", login, email, password }
}

export type SIGNUP_ON_SUBMIT_SUCCESS = { type: "SIGNUP_ON_SUBMIT_SUCCESS" }
export const signupOnSubmitSuccess = (): SIGNUP_ON_SUBMIT_SUCCESS => ({ type: "SIGNUP_ON_SUBMIT_SUCCESS" })

export type SIGNUP_ON_SUBMIT_ERROR = { type: "SIGNUP_ON_SUBMIT_ERROR", error: any }
export const signupOnSubmitError = (error: any): SIGNUP_ON_SUBMIT_ERROR => ({ type: "SIGNUP_ON_SUBMIT_ERROR", error })
