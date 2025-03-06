import { setServers } from "dns";
import { useState, useEffect } from "react";
import { useMutation, gql } from "urql";
const createReviewMutation = gql`
    mutation createReviewMutation($input: CreateReviewInput!) {
        createReview(input: $input) {
            title
        }
    }
`;
const updateReviewMutation = gql`
    mutation updateReviewMutation($input: UpdateReviewInput!) {
        updateReview(input: $input) {
            title
        }
    }
`;


export default function CreateReview(props: { show: boolean, setShow: (show: boolean) => void, deerId: string | null, review: any|null, setReview: (review: any|null) => void}) 
{
    const [createReviewResult, executeCreateReviewMutation] = useMutation(createReviewMutation);
    const [updateReviewResult, executeUpdateReviewMutation] = useMutation(updateReviewMutation);
    const [title, setTitle] = useState("");
    const [body, setBody] = useState("");
    const [dangerLevel, setDangerLevel] = useState("");
    const [submissionError, setSubmissionError] = useState("");
    const submit = async () => {
        if(title && body && dangerLevel) {
            let dangerLevelInt = parseInt(dangerLevel);
            if(isNaN(dangerLevelInt)) {
                setSubmissionError("Danger level must be a number");
                return;
            }
            const test = props.review ?  await executeUpdateReviewMutation({ input: { cervidaeId: props.deerId, title: title,
                body: body, dangerLevel: dangerLevelInt, userId: props.review.user.id} }): 
            await executeCreateReviewMutation({ input: { cervidaeId: props.deerId, title: title,
                body: body, dangerLevel: dangerLevelInt, userId: "fabfe0da-9a94-46d3-b380-73cf71246c0c"} })
                
            if(test.error) {
                setSubmissionError(test.error.message);
            } else {
                props.setShow(false);
                props.setReview(null);
            }
        }else{
            setSubmissionError("Please fill in all fields");
        }
    };
    useEffect(() => {
        if(props.review) {
            setTitle(props.review.title);
            setBody(props.review.body);
            setDangerLevel(props.review.dangerLevel);
        }else{
            setTitle("");
            setBody("");
            setDangerLevel("");
        }
    }, [props.review]);
    return props.show ? (
        <div className="absolute w-screen h-screen flex justify-center items-center" onClick={() => {props.setShow(false); props.setReview(null);}}>
            <div className=" z-40 rounded-md bg-gray-800 p-4" onClick={(e) => e.stopPropagation()}>
                <form className="flex flex-col gap-2">
                    <label htmlFor="title">Title</label>
                    <textarea className="w-full border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2" autoFocus name="title" value={title} onChange={(e) => setTitle(e.target.value)}></textarea>
                    <label htmlFor="body">Body</label>
                    <textarea className="w-full border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2" name="body" value={body} onChange={(e) => setBody(e.target.value)}></textarea>
                    <label htmlFor="dangerLevel">Danger Level</label>
                    <input min={0} max={10} className="w-full border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2" type="number" name="dangerLevel" value={dangerLevel} onChange={(e) => setDangerLevel(e.target.value)}></input>
                {submissionError && <p>{submissionError}</p>}
                <button type="button" onClick={submit}>Submit</button>
            </form>
        </div>
        </div>
    ) : null;
}