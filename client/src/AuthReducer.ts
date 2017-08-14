import { LOGIN_ON_SUBMIT_SUCCESS } from "./login/LoginActions"

export interface AuthState {
    token?: string
}

const initState: AuthState = {
    token: undefined,
}

const AuthReducer = (state: AuthState = initState, action: any) => {
    switch (action.type) {
        case LOGIN_ON_SUBMIT_SUCCESS: return { ...state, token: action.token }
        default: return state
    }
}

export default AuthReducer
