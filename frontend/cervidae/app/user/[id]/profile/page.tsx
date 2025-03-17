'use client';
import { useParams } from "next/navigation";
import { useQuery } from "urql";
import edit  from "@/public/edit.svg"
import { useAuth } from "@/ui/auth-provider";
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
export default function Page() {
    const { userId } = useAuth();
    if(!userId) {
        return <div>Unauthorized: Please login to view this page</div>
    }
    const { id } = useParams();
    const [result, reexecuteQuery] = useQuery({
        query: UserQuery,
        variables: { id },
    });
    return (
        <div className="mt-5">
            <div className="flex justify-end">
                <img src={edit.src} alt="edit" className="w-7 h-7 invert mr-4 cursor-pointer hover:bg-gray-400 rounded-md p-1" />
            </div>
            <div className="grid grid-cols-2 gap-4 p-4  w-10/12 mx-auto">
                <label htmlFor="name">Name: </label>
                <h1>{result.data?.user.name}</h1>
                <label htmlFor="email">Email: </label>
                <p>{result.data?.user.email}</p>
                <label>Created At: </label>
                <p>{result.data?.user.createdAt}</p>
                <label>Updated At:</label>
                <p>{result.data?.user.updatedAt}</p>
            </div>
            <div className="flex justify-center">
                <button className="bg-green-700 text-white px-4 py-2 rounded-md mt-4">Click here to reset password</button>
            </div>
        </div>
    )
}