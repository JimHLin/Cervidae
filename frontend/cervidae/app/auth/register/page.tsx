"use client"
import { Suspense } from 'react';
import { useMutation, gql } from 'urql';

const registerString = gql`
    mutation Register($input: CreateUserInput!) {
        register(input: $input) {
            id
            name
            email
            password
        }
    }
`;


const register = async function register(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault(); // Prevent the default form submission behavior
    const [registerResult, executeRegister] = useMutation(registerString);
    const formData = new FormData(event.currentTarget);
    const email = formData.get('email');
    const name = formData.get('name');
    const password = formData.get('password');
    const registerInput = {
        name: name as string,
        email: email as string,
        password: password as string
    }
    const response = await executeRegister({ input: registerInput });
    if (response.error) {
        console.log(response.error);
    }else{
    console.log('Registration successful:', response.data);
    }
};

export default function RegisterPage(){
    console.log('test');
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <div className="flex flex-col items-center justify-center h-screen gap-2">
                <h1 className="text-4xl">Login</h1>
                <form className="flex flex-col items-center justify-center gap-2" onSubmit={register}>
                    <div className="grid grid-cols-2 grid-flow-row gap-2">
                        <label htmlFor="email">Email</label>
                        <input type="text" placeholder="Email" />
                        <label htmlFor="name">Name</label>
                        <input type="text" placeholder="Name" />
                        <label htmlFor="password">Password</label>
                        <input type="password" placeholder="Password" />
                    </div>
                    <button type="submit">Register</button>
                </form>
            </div>
        </Suspense>
    )
}