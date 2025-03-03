export default function Comment(props: {comment: any}){
    return (
        <div className="flex flex-col">
            <div className="flex flex-row items-baseline justify-between">
                <p className="text-xs pl-2 pr-2 pt-1 pb-1 rounded-t dark:bg-gray-700">{props.comment.user.name}</p>
                <p className="text-xs">{props.comment.createdAt}</p>
            </div>
            <div className="w-full bg-gray-100 p-2 rounded-b dark:bg-gray-600">
                <p>{props.comment.content}</p>
            </div>
        </div>
    )
}