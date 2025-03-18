'use client'
import {useQuery} from 'urql'
import { useParams } from 'next/navigation'
import Review from '@/ui/review'
import { useState } from 'react'
import CreateReview from '@/ui/create-review'

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
`
export default function Reviews() {
    const params = useParams();
    const id = params.id;
    const [result, reexecuteQuery] = useQuery({
        query: reviewsQueryString,
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
            <h1>Reviews</h1>
            {result.data?.userReviews.map((review: any) => (
                <Review key={review.deer.id} review={review} deerId={review.deer.id} 
                reload={() => {reexecuteQuery({ requestPolicy: 'network-only' });}} editReview={(() => editReview(review))} />
            ))}
            
        </div>
    )
}