import DangerRating from "./danger_rating";

export default function Review(props: {review: any}){
    return (
        <div className="flex flex-col w-ful bg-orange-900 p-4 gap-4 max-w-64 flex-shrink-0">
            <h2 className="text-2xl font-bold">{props.review.title}</h2>
            <div className="flex">
                <DangerRating rating={props.review.dangerLevel}/>
            </div>
            <p className="text-sm text-gray-50">
                {props.review.body.slice(0, 100)}
                {props.review.body.length > 100 && '...'}
            </p>
            {props.review.body.length > 100 &&
            <p className="text-xs text-gray-50 text-right">
                <a href={`/deer/${props.review.cervidaeId}`}>Read More</a>
            </p>
            }
            <p className="text-xs text-gray-50 text-right">
                - {props.review.user.name}
            </p>
        </div>
    )
}