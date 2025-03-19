'use client'
import {useQuery} from 'urql'
import { useParams } from 'next/navigation'
import Comment from '@/ui/comment'
import Link from 'next/link'

const commentsQueryString = `
    query Comments($id: UuidScalar!) {
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
            deer{
                id
                name
            }
        }
    }
`

const userQueryString = `
    query User($id: UuidScalar!) {
        user(id: $id) {
            name
        }
    }
`
export default function Comments() {
    const params = useParams();
    const id = params.id;
    const [result, reexecuteQuery] = useQuery({
        query: commentsQueryString,
        variables: {id}
    });
    const [userResult, reexecuteUserQuery] = useQuery({
        query: userQueryString,
        variables: {id}
    });
    return (
        <div>
            {userResult.data?.user.name ?
            <h2 className="text-2xl font-bold">{userResult.data?.user.name}'s Comments:</h2>
            :
            <h2 className="text-2xl font-bold">Comments:</h2>
            }
            <div className="flex flex-col gap-3 pt-4">
            {result.data?.userComments.sort((a: any, b: any) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()).map((comment: any) => (
                <div key={comment.id}>
                    <label className="text-md">On <Link href={`/deer/${comment.deer.id}`} className="text-blue-500 hover:underline">{comment.deer.name}</Link></label>
                    <Comment comment={comment} reload={() => {}} setParentComment={() => {}} />
                </div>
            ))}
            </div>
        </div>
    )
}