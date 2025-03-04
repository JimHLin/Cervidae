'use client'
import { gql, useQuery, useMutation } from "urql";
import { useEffect, useState, useCallback } from "react";
import Comment from "@/ui/comment";

export default function DeerPage({ params }: { params: Promise<{ id: string }> }) {
    const [deerId, setDeerId] = useState<string | null>(null);

    useEffect(() => {
        params.then(resolvedParams => {
            setDeerId(resolvedParams.id);
        });
    }, [params]);

    const query = gql`
    query query ($id: String!) {
      deer(id: $id) {
        id
        name
        description
        imageUrl
        killCount
      }
    }
    `;

    const commentsQuery = gql`
    query commentsQuery ($id: String!) {
      deerComments(id: $id) {
      id
        user{
          name
        }
        parent{
          content
        }
        content
        createdAt
        updatedAt
      }
    }
    `;

    const createCommentMutation = gql`
    mutation createCommentMutation ($input: createCommentInput!) {
      createComment(input: $input) {
        createdAt
        content
        parent{
          name
        }
        user{
          namename
        }
      }
    }
    `;

    const [result, reexecuteQuery] = useQuery({
        query: query,
        variables: { id: deerId },
        pause: !deerId, // Pause the query until deerId is set
    });

    const [commentsResult, reexecuteCommentsQuery] = useQuery({
        query: commentsQuery,
        variables: { id: deerId },
        pause: !deerId, // Pause the query until deerId is set
    });

    const [createCommentResult, executeCreateCommentMutation] = useMutation(createCommentMutation);

    const submit = useCallback(() => {
      executeCreateCommentMutation({
        input: { deerId: deerId, content: "test", userId: "fabfe0da-9a94-46d3-b380-73cf71246c0c", parentId: null }
      })
    }, [executeCreateCommentMutation, deerId])

    const { data, fetching, error } = result;
    if (fetching) return <p>Loading...</p>;
    if (error) return <p>Oh no... {error.message}</p>;
    const { data: commentsData, fetching: commentsFetching, error: commentsError } = commentsResult;
    return (
        <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
            <h1>{data?.deer.name}</h1>
            <img src={data?.deer.imageUrl} alt="Deer" onError={(e) => {
                e.currentTarget.src = "https://i.postimg.cc/L69Q7Xzf/defaultdeer.webp";
            }} width="auto" height="auto" className="w-full h-40 object-scale-down bg-green-900" />
            <p>{data?.deer.description}</p>
            <p>Deer Kill Count: {data?.deer.killCount}</p>
            <div className="flex flex-col gap-4 w-full">
              <h2 className="text-2xl font-bold">Comments</h2>
              <textarea className="w-full h-20 border-2 border-gray-300 dark:bg-gray-900 rounded-md p-2" placeholder="Add a comment" />
              <button className="bg-green-500 text-white px-4 py-2 rounded-md" onClick={submit}>Add Comment</button>
              <div className="flex flex-col gap-2 w-full mt-4">
                  {commentsData?.deerComments.map((comment: any) => (
                      <Comment key={comment.id} comment={comment} />
                  ))}
              </div>
            </div>
        </div>
    );
}