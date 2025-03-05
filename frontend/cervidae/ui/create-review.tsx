import { setServers } from "dns";
import { useState, useCallback } from "react";
import { useMutation, gql } from "urql";
const createReviewMutation = gql`
    mutation createReviewMutation($input: CreateReviewInput!) {
        createReview(input: $input) {
            title
        }
    }
`;


export default function CreateReview(props: { show: boolean, setShow: (show: boolean) => void, deerId: string | null}) 
{
    const [createReviewResult, executeCreateReviewMutation] = useMutation(createReviewMutation);
    const [title, setTitle] = useState("");
    const [body, setBody] = useState("");
    const [dangerLevel, setDangerLevel] = useState("");
    const [createReviewError, setCreateReviewError] = useState("");
    const [createReviewSuccess, setCreateReviewSuccess] = useState("");
    const submit = async () => {
        console.log(title);
        console.log(body);
        console.log(dangerLevel);
        if(title && body && dangerLevel) {
            let dangerLevelInt = parseInt(dangerLevel);
            if(isNaN(dangerLevelInt)) {
                setCreateReviewError("Danger level must be a number");
                return;
            }
            const test = await executeCreateReviewMutation({ input: { cervidaeId: props.deerId, title: title, body: body, dangerLevel: dangerLevelInt, userId: "fabfe0da-9a94-46d3-b380-73cf71246c0f"} })
            if(test.error) {
                setCreateReviewError(test.error.message);
            } else {
                setCreateReviewSuccess("Review created successfully");
                props.setShow(false);
            }
        }else{
            setCreateReviewError("Please fill in all fields");
        }
    };
    return props.show ? (
        <div className="absolute w-screen h-screen flex justify-center items-center" onClick={() => props.setShow(false)}>
            <div className=" z-40 rounded-md bg-gray-800 p-4" onClick={(e) => e.stopPropagation()}>
                <form className="flex flex-col gap-2">
                    <label htmlFor="title">Title</label>
                    <textarea className="w-full border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2" autoFocus name="title" value={title} onChange={(e) => setTitle(e.target.value)}></textarea>
                    <label htmlFor="body">Body</label>
                    <textarea className="w-full border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2" name="body" value={body} onChange={(e) => setBody(e.target.value)}></textarea>
                    <label htmlFor="dangerLevel">Danger Level</label>
                    <input min={0} max={10} className="w-full border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2" type="number" name="dangerLevel" value={dangerLevel} onChange={(e) => setDangerLevel(e.target.value)}></input>
                {createReviewError && <p>{createReviewError}</p>}
            {createReviewSuccess && <p>{createReviewSuccess}</p>}
                <button type="button" onClick={submit}>Submit</button>
            </form>
        </div>
        </div>
    ) : null;
}