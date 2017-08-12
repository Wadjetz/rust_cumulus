import * as React from "react"

interface Props {
    loading: boolean
    newSourceUrl: string
    onChange: (field: string, value: string) => void
    onSubmit: (sourceUrl: string) => void
}

export default class AddSourceForm extends React.Component<Props, {}> {
    render() {
        const { newSourceUrl } = this.props
        console.log("AddSourceForm.render", newSourceUrl)
        return (
            <div>
                <label>Add source</label>
                <input
                    type="text"
                    value={newSourceUrl}
                    onChange={this.onChangeHandler("newSourceUrl")}
                />
                <button onClick={this.onSubmitHandler}>Add</button>
            </div>
        )
    }

    onChangeHandler = (field: string) => (value: React.FormEvent<HTMLInputElement>) => {
        const { onChange } = this.props
        onChange(field, (value.target as any).value)
    }

    onSubmitHandler = () => {
        const { newSourceUrl, loading, onSubmit } = this.props
        if (!loading) {
            onSubmit(newSourceUrl)
        }
    }
}



