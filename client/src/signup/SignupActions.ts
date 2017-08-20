export const SIGNUP_ON_CHANGE = "SIGNUP_ON_CHANGE"
export const signupOnChange = (field: string, value: string) => ({ type: SIGNUP_ON_CHANGE, field, value })

export const SIGNUP_ON_SUBMIT = "SIGNUP_ON_SUBMIT"
export const signupOnSubmit = (login: string, email: string, password: string) => ({ type: SIGNUP_ON_SUBMIT, login, email, password })

export const SIGNUP_ON_SUBMIT_SUCCESS = "SIGNUP_ON_SUBMIT_SUCCESS"
export const signupOnSubmitSuccess = () => ({ type: SIGNUP_ON_SUBMIT_SUCCESS })

export const SIGNUP_ON_SUBMIT_ERROR = "SIGNUP_ON_SUBMIT_ERROR"
export const signupOnSubmitError = (error: any) => ({ type: SIGNUP_ON_SUBMIT_ERROR, error })
