export const LOGIN_ON_CHANGE = "LOGIN_ON_CHANGE"
export const loginOnChange = (field: string, value: string) => ({ type: LOGIN_ON_CHANGE, field, value })

export const LOGIN_ON_SUBMIT = "LOGIN_ON_SUBMIT"
export const loginOnSubmit = (email: string, password: string) => ({ type: LOGIN_ON_SUBMIT, email, password })

export const LOGIN_ON_SUBMIT_SUCCESS = "LOGIN_ON_SUBMIT_SUCCESS"
export const loginOnSubmitSuccess = (token: string) => ({ type: LOGIN_ON_SUBMIT_SUCCESS, token })

export const LOGIN_ON_SUBMIT_ERROR = "LOGIN_ON_SUBMIT_ERROR"
export const loginOnSubmitError = (error: any) => ({ type: LOGIN_ON_SUBMIT_ERROR, error })
