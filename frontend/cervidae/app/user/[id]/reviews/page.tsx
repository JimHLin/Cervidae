'use client'
import {useQuery} from 'urql'
import { useParams } from 'next/navigation'
import Review from '@/ui/review'
import { useState } from 'react'
import CreateReview from '@/ui/create-review'
import Link from 'next/link'

const reviewsQueryString = `
    query Reviews($id: ID!) {
        userReviews(id: $id) {
            deer{
                id
                name
            }
            user{
                id
                name
            }
            dangerLevel
            title
            body
            createdAt
            updatedAt
        }
    }
`;
const getUserQueryString = `
    query User($id: ID!) {
        user(id: $id) {
            name
        }
    }
`;
export default function Reviews() {
    const params = useParams();
    const id = params.id;
    const [result, reexecuteQuery] = useQuery({
        query: reviewsQueryString,
        variables: {id}
    })
    const [userResult, reexecuteUserQuery] = useQuery({
        query: getUserQueryString,
        variables: {id}
    })
    const [show, setShow] = useState(false);
    const [review, setReview] = useState(null);
    const [editedReview, setEditedReview] = useState(null);
    const [editedDeerId, setEditedDeerId] = useState(null);
    const editReview = (review: any) => {
        setEditedReview(review);
        setEditedDeerId(review.deer.id);
        setShow(true);
    }
    return (
        <div>
            {show && (
                <CreateReview show={show} setShow={setShow} deerId={editedDeerId} review={editedReview} setReview={setEditedReview} />
            )}
            {userResult.data?.user.name ?
            <h2 className="text-2xl font-bold">{userResult.data?.user.name}'s Reviews:</h2>
            :
            <h2 className="text-2xl font-bold">Comments:</h2>
            }
            <div className="flex flex-row gap-3 pt-4 flex-wrap justify-evenly">
            {result.data?.userReviews.map((review: any) => (
                <div key={review.deer.id}>
                    <label className="text-md">On <Link href={`/deer/${review.deer.id}`} className="text-blue-500 hover:underline">{review.deer.name}</Link></label>
                    <Review review={review} deerId={review.deer.id} 
                    reload={() => {reexecuteQuery({ requestPolicy: 'network-only' });}} editReview={(() => editReview(review))} />
                </div>
            ))}
            </div>
        </div>
    )
}