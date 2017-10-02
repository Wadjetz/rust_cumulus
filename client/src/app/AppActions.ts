export type MENU_TOGGLE = {
    type: "MENU_TOGGLE"
    isMenuOpen: boolean
}
export function menuToggle(isMenuOpen: boolean): MENU_TOGGLE {
    return { type: "MENU_TOGGLE", isMenuOpen }
}

export type AppAction = MENU_TOGGLE
