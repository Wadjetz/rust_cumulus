import { SignupAction } from "./SignupActions"
import { ApiError } from "../Api"

export interface SignupState {
    login: string
    email: string
    password: string
    loading: boolean
    error?: ApiError
}

const initState: SignupState = {
    login: "",
    email: "",
    password: "",
    loading: false,
    error: undefined,
}

const SignupReducer = (state: SignupState = initState, action: SignupAction) => {
    switch (action.type) {
        case "SIGNUP_ON_CHANGE": return { ...state, [action.field]: action.value }
        case "SIGNUP_ON_SUBMIT": return { ...state, loading: true, error: undefined }
        case "SIGNUP_ON_SUBMIT_SUCCESS": return { ...state, loading: false, login: "", email: "", password: "" }
        case "SIGNUP_ON_SUBMIT_ERROR": return { ...state, error: action.error, loading: false }
        default: return state
    }
}

export default SignupReducer
