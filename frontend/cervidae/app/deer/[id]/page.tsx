'use client'
import { gql, useQuery, useMutation } from "urql";
import { useEffect, useState, useCallback } from "react";
import Comment from "@/ui/comment";
import Review from "@/ui/review";
import CreateReview from "@/ui/create-review";

export default function DeerPage({ params }: { params: Promise<{ id: string }> }) {
    const [deerId, setDeerId] = useState<string | null>(null);

    useEffect(() => {
        params.then(resolvedParams => {
            setDeerId(resolvedParams.id);
        });
    }, [params]);
    
    const [createCommentError, setCreateCommentError] = useState<string | null>(null);
    const [showCreateReview, setShowCreateReview] = useState<boolean>(false);
    const [commentValue, setCommentValue] = useState<string>("");
    const [review, setReview] = useState<any|null>(null);
    const query = gql`
    query query ($id: String!) {
      deer(id: $id) {
        id
        name
        description
        imageUrl
        killCount
        reviews{
          dangerLevel
          title
          body
          createdAt
          updatedAt
          user{
            id
            name
          }
        }
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
    mutation createCommentMutation ($input: CreateCommentInput!) {
      createComment(input: $input) {
        id
        content
        user{
          name
        }
        parent{
          id
          content
        }
        createdAt
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

    const submit = useCallback(async () => {
      if(commentValue) {
        const test = await executeCreateCommentMutation({
          input: { cervidaeId: deerId, content: commentValue, userId: "fabfe0da-9a94-46d3-b380-73cf71246c0c", parentId: null }
        })
        if(test.error) {
          console.log(test.error);
          setCreateCommentError(test.error.message);
        }else{
          setCreateCommentError(null);
        }
      }
    }, [executeCreateCommentMutation, deerId])

    const reloadComments = useCallback(() => {
      console.log('reexecuting comments query');
      reexecuteCommentsQuery({ requestPolicy: 'network-only' });
    }, [reexecuteCommentsQuery]);

    const populateReviewForm = async (review: any|null) => {
      setReview(review);
      if(review) {
        setShowCreateReview(true);
      }
    }

    const { data, fetching, error } = result;
    if (fetching) return <p>Loading...</p>;
    if (error) return <p>Oh no... {error.message}</p>;
    const { data: commentsData, fetching: commentsFetching, error: commentsError } = commentsResult;
    return (
        <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
          <CreateReview show={showCreateReview} setShow={setShowCreateReview} deerId={deerId} review={review} setReview={setReview}/>
            <h1>{data?.deer.name}</h1>
            <img src={data?.deer.imageUrl} alt="Deer" onError={(e) => {
                e.currentTarget.src = "https://i.postimg.cc/L69Q7Xzf/defaultdeer.webp";
            }} width="auto" height="auto" className="w-full h-40 object-scale-down bg-green-900" />
            <p>{data?.deer.description}</p>
            <p>Deer Kill Count: {data?.deer.killCount}</p>
            <div className="flex flex-row gap-4 w-full relative overflow-auto">
              {data?.deer.reviews.map((review: any) => (
                <Review key={review.user.id} review={review} deerId={deerId} reload={() => {reexecuteQuery({ requestPolicy: 'network-only' });}}
                 editReview={populateReviewForm}/>
              ))}
            </div>
            <div className="w-full relative">
              <button className="z-10 bg-green-500 bg-opacity-50 text-opacity-50 text-white px-4 py-2 rounded-full absolute bottom-10 right-1
              hover:bg-green-500 hover:text-white hover:bg-opacity-100 hover:text-opacity-100" onClick={() => setShowCreateReview(true)}>+</button>
            </div>
            <div className="flex flex-col gap-4 w-full">
              <h2 className="text-2xl font-bold">Comments</h2>
              <textarea value={commentValue} onChange={(e) => setCommentValue(e.target.value)} className="w-full h-20 border-2 border-gray-300 dark:bg-gray-900 rounded-md p-2" placeholder="Add a comment" />
                {createCommentError && <p className="text-red-500">{createCommentError}</p>}
                <button className="bg-green-500 text-white px-4 py-2 rounded-md" onClick={submit}>Add Comment</button>
              <div className="flex flex-col gap-2 w-full mt-4">
                  {commentsData?.deerComments.map((comment: any) => (
                      <Comment key={comment.id} comment={comment} reload={reloadComments}/>
                  ))}
              </div>
            </div>
        </div>
    );
}