'use client';
import { useParams } from "next/navigation";
import { useQuery } from "urql";

const UserQuery = `
    query User($id: String!) {
        user(id: $id) {
            id
            name
            email
            createdAt
        }
    }
`;
export default function Page() {
    const { id } = useParams();
    const [result, reexecuteQuery] = useQuery({
        query: UserQuery,
        variables: { id },
    });
    return (
        <div>
            <h1>{result.data?.user.name}</h1>
            <form>
                <input type="text" placeholder="Username" />
                <input type="text" placeholder="Email" />
                <input type="text" placeholder="Password" />
                <button type="submit">Submit</button>
            </form>
        </div>
    )
}