'use client'
import { gql, useQuery } from "urql";
import { useEffect, useState } from "react";
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
        userId
        content
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

    const { data, fetching, error } = result;
    if (fetching) return <p>Loading...</p>;
    if (error) return <p>Oh no... {error.message}</p>;
    const { data: commentsData, fetching: commentsFetching, error: commentsError } = commentsResult;
    return (
        <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
            <h1>{data?.deer.name}</h1>
            <img src={data?.deer.imageUrl} alt="Deer" className="w-full h-40 object-scale-down" />
            <p>{data?.deer.description}</p>
            <p>Deer Kill Count: {data?.deer.killCount}</p>
            {commentsData?.deerComments.map((comment: any) => (
                <Comment key={comment.id} comment={comment} />
            ))}
        </div>
    );
}