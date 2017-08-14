import { Epic } from "redux-observable"
import * as router from "../router"
import {
    LOGIN_ON_SUBMIT, LOGIN_ON_SUBMIT_SUCCESS, loginOnSubmitSuccess, loginOnSubmitError
} from "./LoginActions"
import { State } from "../Store"
import * as Api from "../Api"

export const loginEpic: Epic<any, State> = (action$, state) => action$.ofType(LOGIN_ON_SUBMIT)
    .mergeMap(action =>
        Api.login(action.email, action.password)
            .then(loginOnSubmitSuccess)
            .catch(loginOnSubmitError)
    )

export const loginSuccessEpic: Epic<any, State> = (action$) => action$.ofType(LOGIN_ON_SUBMIT_SUCCESS)
    .map(() => {
        router.replace("/")
        return { type: "LOGIN_SUCCESS_REDIRECT" }
    })
