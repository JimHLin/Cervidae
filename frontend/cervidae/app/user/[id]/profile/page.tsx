'use client';
import { useParams } from "next/navigation";
import { useQuery, useMutation } from "urql";
import edit  from "@/public/edit.svg"
import { useAuth } from "@/ui/auth-provider";
import { useState, useEffect } from "react";
import ResetPassword from "@/ui/reset-password";
const UserQuery = `
    query User($id: String!) {
        user(id: $id) {
            id
            name
            email
            createdAt
            updatedAt
        }
    }
`;

const UserMutation = `
    mutation User($input: UpdateUserInput) {
        updateUser(input: $input) {
            id
            name
            email
            createdAt
            updatedAt
        }
    }
`;

const resetPasswordMutation = `
    mutation ResetPassword($input: ResetPasswordInput) {
        resetUserPassword(input: $input)
    }
`;

export default function Page() {
    const { id } = useParams();
    const { userId } = useAuth();
    const [result, reexecuteQuery] = useQuery({
        query: UserQuery,
        variables: { id },
    });
    const [name, setName] = useState(result.data?.user.name);
    const [email, setEmail] = useState(result.data?.user.email);
    useEffect(() => {
        setName(result.data?.user.name);
        setEmail(result.data?.user.email);
    }, [result.data]);

    const [mutationResult, executeMutation] = useMutation(UserMutation);
    const [resetPasswordResult, executeResetPassword] = useMutation(resetPasswordMutation);
    const [editing, setEditing] = useState(false);
    const [show, setShow] = useState(false);

    if(!userId) {
        return <div>Unauthorized: Please login to view this page</div>
    }

    const editUser = async () => {
        const result = await executeMutation({
            input: {
                id: id,
                name,
                email,
            },
        });
        console.log(result);
    }


    return (
        <div className="mt-5">
            {show && (
                <ResetPassword setShow={setShow} resetPassword={executeResetPassword} id={id as string}/>
            )}
            <div className="flex justify-end">
                {!editing && (
                    <img src={edit.src} alt="edit" className="w-7 h-7 invert mr-4 cursor-pointer hover:bg-gray-400 rounded-md p-1" onClick={() => setEditing(!editing)} />
                )}
            </div>
            <div className="grid grid-cols-2 gap-4 p-4  w-10/12 mx-auto">
                <label htmlFor="name">Name: </label>
                {editing ? (
                    <input type="text" className="border-2 border-gray-300 rounded-md p-2 dark:bg-gray-800 dark:text-white"
                     value={name} onChange={(e) => setName(e.target.value)} />
                ) : (
                    <h1>{result.data?.user.name}</h1>
                )}
                <label htmlFor="email">Email: </label>
                {editing ? (
                    <input type="text" className="border-2 border-gray-300 rounded-md p-2 dark:bg-gray-800 dark:text-white"
                     value={email} onChange={(e) => setEmail(e.target.value)} />
                ) : (
                    <p>{result.data?.user.email}</p>
                )}
                <label>Created At: </label>
                <p>{result.data?.user.createdAt}</p>
                <label>Updated At:</label>
                <p>{result.data?.user.updatedAt}</p>
            </div>
            {editing && (
                <div className="flex justify-center gap-10">
                    <button className="bg-green-700 text-white px-4 py-2 rounded-md mt-4" onClick={editUser}>Save</button>
                    <button className="bg-gray-700 text-white px-4 py-2 rounded-md mt-4" onClick={() => setEditing(!editing)}>Cancel</button>
                </div>
            )}
            {
                !editing && (
                    <div className="flex justify-center">
                        <button onClick={() => setShow(true)} className="bg-green-700 text-white px-4 py-2 rounded-md mt-4">Click here to reset password</button>
                    </div>
                )
            }
        </div>
    )
}