import * as React from "react"
import * as styles from "./Header.css"
import * as Api from "../Api"

export default class Header extends React.Component<{}, {}> {
    render() {
        return (
            <div className={styles.header}>
                <a className={styles.item} href="#/">MindStream</a>
                <a className={styles.item} href="#/feeds">Feeds</a>
                <a className={styles.item} href="#/sources">Sources</a>
                <div className={styles.item} onClick={Api.logout}>Logout</div>
            </div>
        )
    }
}
