import * as React from "react"

interface Props {
    color?: string
    width?: number
    height?: number
}

export default class MenuIcon extends React.PureComponent<Props> {
    render() {
        const { color = "#000000", width = 24, height = 24 } = this.props
        return (
            <svg fill={color} width={width} height={height} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M0 0h24v24H0z" fill="none"/>
                <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
            </svg>
        )
    }
}