"use client"
import { Suspense } from 'react';
import { useMutation, gql } from 'urql';

const registerString = gql`
    mutation Register($input: CreateUserInput!) {
        createUser(input: $input) {
            id
            name
            email
            password
        }
    }
`;


export default function RegisterPage(){
    const [registerResult, executeRegister] = useMutation(registerString);

    const register = async function register(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault(); // Prevent the default form submission behavior
        
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

    return (
        <Suspense fallback={<div>Loading...</div>}>
            <div className="flex flex-col items-center justify-center h-screen gap-2">
                <h1 className="text-4xl">Register</h1>
                <form className="flex flex-col items-center justify-center gap-2" onSubmit={register}>
                    <div className="grid grid-cols-2 grid-flow-row gap-2">
                        <label htmlFor="email">Email</label>
                        <input type="text" autoComplete="none" placeholder="Email" name="email" className="border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2"/>
                        <label htmlFor="name">Name</label>
                        <input type="text" autoComplete="none" placeholder="Name" name="name" className="border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2"/>
                        <label htmlFor="password">Password</label>
                        <input type="password" autoComplete="none" placeholder="Password" name="password" className="border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2"/>
                    </div>
                    <button type="submit">Register</button>
                </form>
            </div>
        </Suspense>
    )
}