export default function Switch(props: {onChange: (value: boolean) => void, value: boolean}) {
    return (
        <div className="flex flex-row gap-4">
            <label htmlFor="viewPending">View Pending</label>
            <div className={`relative w-10 h-5 bg-gray-300 rounded-full cursor-pointer transition-all duration-200 ${props.value ? "bg-green-600" : "bg-gray-300"}`} onClick={() => props.onChange(!props.value)}>
                <input name="viewPending" type="checkbox" className="hidden" checked={props.value} onChange={(e) => {
                    props.onChange(e.target.checked);
                }} />
                <div className={`absolute top-0 left-0 w-5 h-5 bg-white rounded-full transition-all duration-200 ${props.value ? "translate-x-full" : "translate-x-0"}`}></div>
            </div>
        </div>
    )
}
