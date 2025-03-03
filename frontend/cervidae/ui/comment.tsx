export default function Comment(props: {comment: any}){
    return (
        <div className="flex flex-col gap-4">
            <div className="flex flex-row gap-2 items-baseline">
                <p className="text-sm max-w-10 overflow-hidden text-ellipsis whitespace-nowrap">{props.comment.userId}</p>
                <p>says: </p>
            </div>
        <div className="w-full bg-gray-100 p-4 rounded-md dark:bg-gray-700">
            <p>{props.comment.content}</p>
        </div>
        </div>
    )
}