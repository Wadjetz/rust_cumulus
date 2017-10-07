import { Epic } from "redux-observable"
import { Action } from "redux"
import * as router from "../router"
import { loginOnSubmitSuccess, loginOnSubmitError } from "./LoginActions"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const loginEpic: Epic<any, GlobalState> = (action$, state) => action$.ofType("LOGIN_ON_SUBMIT")
    .mergeMap(action =>
        Api.login(action.email, action.password)
            .then(loginOnSubmitSuccess)
            .catch(loginOnSubmitError)
    )

export const loginSuccessEpic: Epic<Action, GlobalState> = (action$) => action$.ofType("LOGIN_ON_SUBMIT_SUCCESS")
    .map(() => {
        router.replace("/")
        return { type: "LOGIN_SUCCESS_REDIRECT" }
    })

export const loginErrorEpic: Epic<any, GlobalState> = (action$, state) =>
    action$.filter(action => !!action.errors && action.error.errors.find((e: any) => e.message === "invalid token"))
        .map(action => {
            router.replace("/login")
            return { type: "LOGIN_ERROR_REDIRECT" }
        })
