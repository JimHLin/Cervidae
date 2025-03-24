export default function GenericInput(props: {value: string, onChange: (value: string) => void}, name?: string) {
    return (
        <input className="border-2 border-gray-300 rounded-md p-2 dark:bg-gray-800 dark:text-white" type="text" value={props.value} onChange={(e) => props.onChange(e.target.value)} />
    )
}