import { Epic } from "redux-observable"
import { Action } from "redux"
import * as router from "../router"
import {
    SIGNUP_ON_SUBMIT, signupOnSubmitSuccess, signupOnSubmitError
} from "./SignupActions"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const signupEpic: Epic<any, GlobalState> = (action$, state) => action$.ofType("SIGNUP_ON_SUBMIT")
    .mergeMap((action: SIGNUP_ON_SUBMIT) =>
        Api.signup(action.login, action.email, action.password)
            .then(signupOnSubmitSuccess)
            .catch(signupOnSubmitError)
    )

export const signupSuccessEpic: Epic<Action, GlobalState> = (action$) => action$.ofType("SIGNUP_ON_SUBMIT_SUCCESS")
    .map(() => {
        router.replace("/")
        return { type: "SIGNUP_SUCCESS_REDIRECT" }
    })
