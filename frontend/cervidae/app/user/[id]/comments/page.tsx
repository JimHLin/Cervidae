'use client'
import {useQuery} from 'urql'
import { useParams } from 'next/navigation'
import Comment from '@/ui/comment'
const commentsQueryString = `
    query Comments($id: ID!) {
        userComments(id: $id) {
            id
            parent{
                id
                content
            }
            content
            createdAt
            updatedAt
            user{
                id
                name
            }
        }
    }
`
export default function Comments() {
    const params = useParams();
    const id = params.id;
    const [result, reexecuteQuery] = useQuery({
        query: commentsQueryString,
        variables: {id}
    })
    return (
        <div>
            <h1>Comments</h1>
            {result.data?.userComments.map((comment: any) => (
                <Comment key={comment.id} comment={comment} reload={() => {}} />
            ))}
        </div>
    )
}