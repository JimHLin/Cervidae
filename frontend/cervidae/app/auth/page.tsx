'use client'
import { Suspense, useState } from 'react';
import { useMutation, gql } from 'urql';
import { useAuth } from '@/ui/auth-provider';
import { redirect } from 'next/navigation';
import PasswordInput from '@/ui/password-input';

const loginString = gql`
    mutation Login($input: LoginInput!) {
        login(input: $input)
    }
`;

export default function LoginPage(){
    const [loginResult, executeLogin] = useMutation(loginString);
    const { isAuthenticated, login, logout, validate } = useAuth(); 
    const [password, setPassword] = useState('');
    if(isAuthenticated){
        redirect('/');
    }
    async function signIn(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);
        const email = formData.get('email');
        const loginInput = {
            email: email as string,
            password: password as string
        }
        const response = await executeLogin({ input: loginInput });
        if (response.error) {
            console.log(response.error);
        }else{
            console.log('Login successful:', response.data);
            validate();
        }
    }
    return (
            <Suspense fallback={<div>Loading...</div>}>
                <div className="flex flex-col items-center justify-center h-screen gap-2">
                    <h1 className="text-4xl">Login</h1>
                <form className="flex flex-col items-center justify-center gap-2" onSubmit={signIn}>
                    <div className="grid grid-cols-2 grid-flow-row gap-2">
                        <label htmlFor="email">Email</label>
                        <input type="text" placeholder="Email" autoComplete='on' className="border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2" name="email"/>
                        <label htmlFor="password">Password</label>
                        <PasswordInput value={password} onChange={(value) => setPassword(value)} />
                    </div>
                    <button type="submit">Login</button>
                </form>
            </div>
        </Suspense>
    )
}