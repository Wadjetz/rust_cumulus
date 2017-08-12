import { Epic } from "redux-observable"

import {
    LOGIN_ON_SUBMIT, loginOnSubmitSuccess, loginOnSubmitError
} from "./LoginActions"
import { State } from "../Store"
import * as Api from "../Api"

export const loginEpic: Epic<any, State> = (action$, state) => action$.ofType(LOGIN_ON_SUBMIT)
    .mergeMap(action =>
        Api.login(action.email, action.password)
            .then(loginOnSubmitSuccess)
            .catch(loginOnSubmitError)
    )