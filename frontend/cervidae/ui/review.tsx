import DangerRating from "./danger-rating";
import Image from "next/image";
import { useState, useRef, useEffect } from "react";
import { useMutation } from "urql";
const deleteReviewString = `
    mutation deleteReviewMutation($input: UpdateReviewInput!) {
        deleteReview(input: $input)
    }
`;

export default function Review(props: {review: any, deerId: string|null, reload: () => void, editReview: (review: any|null) => void}){
    const [showOptions, setShowOptions] = useState(false);
    const optionsRef = useRef<HTMLDivElement>(null);
    const [deleteResult, executeDelete] = useMutation(deleteReviewString);
    const deleteReview = async () => {
        let res = await executeDelete({input: {cervidaeId: props.deerId, userId: props.review.user.id}});
        if(res.error) {
            console.log(res.error);
        } else {
            props.reload();
        }
    }
    const handleClickOutside = (event: MouseEvent) => {
        if (optionsRef.current && !optionsRef.current.contains(event.target as Node)) {
            setShowOptions(false);
        }
    };
    useEffect(() => {
        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, [handleClickOutside]);



    return (
        <div className="flex flex-col w-ful bg-orange-900 p-4 gap-4 max-w-64 flex-shrink-0">
            <div className="flex flex-row gap-2 justify-between items-baseline">
                <p className="text-xl font-bold">{props.review.title}</p>
                <div className="text-xs text-gray-50 text-right relative">
                    <button onClick={() => setShowOptions(!showOptions)}>
                        <Image width={16} height={16} src="/options_vert.svg" alt="options" className="w-4 h-4 dark:invert hover:cursor-pointer hover:scale-110 transition-all duration-300 select-none"/>
                    </button>
                    {showOptions && 
                    <div ref={optionsRef} className="z-20 flex flex-col justify-start text-left absolute top-0 right-0 dark:bg-orange-800 border border-gray-300 rounded-s">
                        <button className="text-xs dark:text-gray-50 p-1 hover:bg-orange-900 hover:cursor-pointer transition-all duration-300"
                        onClick={() => props.editReview(props.review)}
                        >Edit</button>
                        <button className="text-xs dark:text-gray-50 border-t border-gray-300 p-1 hover:bg-orange-900 hover:cursor-pointer transition-all duration-300"
                        onClick={() => deleteReview()}
                        >Delete</button>
                    </div>
                    }
                </div>
            </div>
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