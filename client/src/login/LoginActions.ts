export const LOGIN_ON_CHANGE = "LOGIN_ON_CHANGE"
export const LOGIN_ON_SUBMIT = "LOGIN_ON_SUBMIT"
export const LOGIN_ON_SUBMIT_SUCCESS = "LOGIN_ON_SUBMIT_SUCCESS"
export const LOGIN_ON_SUBMIT_ERROR = "LOGIN_ON_SUBMIT_ERROR"

export function loginOnChange(field: string, value: string) {
    return {
        type: LOGIN_ON_CHANGE,
        field,
        value
    }
}

export function loginOnSubmit() {
    return {
        type: LOGIN_ON_SUBMIT,
    }
}

export function loginOnSubmitSuccess(token: string) {
    return {
        type: LOGIN_ON_SUBMIT_SUCCESS,
        token
    }
}

export function loginOnSubmitError(error: any) {
    return {
        type: LOGIN_ON_SUBMIT_ERROR,
        error
    }
}
