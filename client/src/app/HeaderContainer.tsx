import * as React from "react"
import { connect, Dispatch } from "react-redux"
import Header from "../components/Header"
import { GlobalState } from "../app/AppState"
import * as AppActions from "./AppActions"

interface DispatchProps {
    menuToggle(isMenuOpen: boolean): void
}

interface StateProps {
    isMenuOpen: boolean
}

type Props = StateProps & DispatchProps

class HeaderContainer extends React.PureComponent<Props> {
    render() {
        const { isMenuOpen, menuToggle } = this.props
        return <Header isMenuOpen={isMenuOpen} onMenuToggle={menuToggle} />
    }
}

const mapStateToProps = (state: GlobalState): StateProps => {
    return {
        isMenuOpen: state.app.isMenuOpen,
    }
}

const mapDispatchToProps = (dispatch: Dispatch<GlobalState>): DispatchProps => {
    return {
        menuToggle: (isMenuOpen) => dispatch(AppActions.menuToggle(isMenuOpen))
    }
}

export default connect(mapStateToProps, mapDispatchToProps)(HeaderContainer)
