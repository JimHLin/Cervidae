'use client'
import {useQuery} from 'urql'
import { useParams } from 'next/navigation'
import Review from '@/ui/review'
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
    return (
        <div>
            <h1>Reviews</h1>
            {result.data?.userReviews.map((review: any) => (
                <Review key={review.deer.id} review={review} deerId={review.deer.id} reload={() => {}} editReview={() => {}} />
            ))}
        </div>
    )
}