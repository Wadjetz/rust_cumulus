import { AppAction } from "./AppActions"

export interface AppState {
    isMenuOpen: boolean
}

const initState: AppState = {
    isMenuOpen: false,
}

const AppReducer = (state: AppState = initState, action: AppAction) => {
    switch (action.type) {
        case "MENU_TOGGLE": return { ...state, isMenuOpen: action.isMenuOpen }
        default: return state
    }
}

export default AppReducer
