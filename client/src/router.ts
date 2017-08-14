import createHashHistory from "history/createHashHistory"
export const history = createHashHistory()

export function replace(path: string, params?: any) {
    history.replace(path, params)
}
