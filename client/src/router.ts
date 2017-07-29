import createHashHistory from "history/createHashHistory"
export const history = createHashHistory()

export function push(path: string, params?: any) {
    history.replace(path, params)
}
