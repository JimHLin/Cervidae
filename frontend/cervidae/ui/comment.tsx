
import { gql } from "urql";
import { useCallback, useState, useRef } from "react";
import { useMutation } from "urql";
import { useAuth } from "./auth-provider";
import reply from "@/public/reply.svg";

export default function Comment(props: {comment: any, reload: () => void}){
    const { isAuthenticated, userId, isAdmin } = useAuth();
    const deleteCommentMutation = gql`
    mutation deleteCommentMutation($id: String!) {
        deleteComment(id: $id)
    }
    `;
    const editCommentMutation = gql`
    mutation updateCommentMutation($input: UpdateCommentInput!) {
        updateComment(input: $input){
            id
            content
        }
    }
    `;

    const commentRef = useRef<HTMLTextAreaElement | null>(null);
    const [deleteCommentResult, executeDeleteCommentMutation] = useMutation(deleteCommentMutation);
    const [updateCommentResult, executeUpdateCommentMutation] = useMutation(editCommentMutation);
    const deleteComment = useCallback(async () => {
        console.log(props.comment);
        const result = await executeDeleteCommentMutation({
            id: props.comment.id
        });
        if(result.error) {
            setActionError(result.error.message);
        }else{
            setActionError(null);
        }
        props.reload();
    }, [executeDeleteCommentMutation, props]);

    const updateComment = useCallback(async () => {
        console.log(props.comment);
        const result = await executeUpdateCommentMutation({
            input: {
                id: props.comment.id,
                content: commentRef.current?.value
            }
        });
        if(result.error) {
            setActionError(result.error.message);
        }else{
            setActionError(null);
            setIsEditing(false);
            props.reload();
        }
    }, []);
    const [isEditing, setIsEditing] = useState(false);
    const [actionError, setActionError] = useState<string | null>(null);
    return (
        <div className="flex flex-col">
            <div className="flex flex-row items-baseline justify-between">
                <p className="text-xs pl-2 pr-2 pt-1 pb-1 rounded-t dark:bg-gray-700">{props.comment.user.name}</p>
                {(isAdmin || (userId && userId == props.comment.user.id)) &&
                <div className="flex flex-row items-center gap-2">
                    <p className="text-xs">{props.comment.createdAt}</p>
                    {isEditing ? (
                        <button className="text-xs text-blue-400 hover:underline cursor-pointer select-none" onClick={updateComment}>Save</button>
                    ) : (
                        <a className="text-xs text-blue-400 hover:underline cursor-pointer select-none" onClick={() => setIsEditing(true)}>Edit</a>
                    )}
                    <a className="text-xs text-blue-400 hover:underline cursor-pointer select-none" onClick={deleteComment}>Delete</a>
                </div>
                }
            </div>
            <div className="w-full bg-gray-100 p-2 rounded-b dark:bg-gray-600">
                {props.comment.parent && (
                    <div className="ml-2 mr-2 rounded-md bg-gray-900 p-1 mb-2">
                        <p>{props.comment.parent?.content}</p>
                    </div>
                )}
                {isEditing ? (
                    <textarea ref={commentRef} className="w-full h-20 border-2 border-gray-300 dark:bg-gray-900 rounded-md p-2" />
                ) : (
                    <p>{props.comment.content}</p>
                )}
                {actionError && <p className="text-red-500">{actionError}</p>}
                <div className="flex flex-row justify-between">
                    {props.comment.editedAt != props.comment.createdAt ? (
                        <p className="text-xs dark: text-gray-300 opacity-90">Edited at: {props.comment.updatedAt}</p>
                    ) : (
                        <p></p>
                    )}
                    <button className="hover:bg-gray-200 dark:hover:bg-gray-800 rounded-md">
                        <img src={reply.src} alt="reply" className="w-4 h-4 float-right" />
                    </button>
                </div>
                
            </div>
        </div>
    )
}