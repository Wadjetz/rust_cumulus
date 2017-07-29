import * as React from "react"
import * as styles from "./Header.css"

export default class Header extends React.Component<{}, {}> {
    render() {
        return (
            <div className={styles.header}>
                <a className={styles.item} href="#/">Feeds</a>
                <a className={styles.item} href="#/login">Login</a>
            </div>
        )
    }
}
