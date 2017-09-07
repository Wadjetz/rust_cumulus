import { Epic } from "redux-observable"
import { Action } from "redux"
import * as router from "../router"
import {
    SIGNUP_ON_SUBMIT, SIGNUP_ON_SUBMIT_SUCCESS, signupOnSubmitSuccess, signupOnSubmitError
} from "./SignupActions"
import { State } from "../Store"
import * as Api from "../Api"

export const signupEpic: Epic<any, State> = (action$, state) => action$.ofType(SIGNUP_ON_SUBMIT)
    .mergeMap(action =>
        Api.signup(action.login, action.email, action.password)
            .then(signupOnSubmitSuccess)
            .catch(signupOnSubmitError)
    )

export const signupSuccessEpic: Epic<Action, State> = (action$) => action$.ofType(SIGNUP_ON_SUBMIT_SUCCESS)
    .map(() => {
        router.replace("/")
        return { type: "SIGNUP_SUCCESS_REDIRECT" }
    })