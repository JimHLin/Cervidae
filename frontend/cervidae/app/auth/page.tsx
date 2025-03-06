import { Suspense } from 'react';

async function signIn(formData: FormData) {
    'use server'
    const email = formData.get('email')
    const password = formData.get('password')
    console.log(email, password)
}

export default function LoginPage(){
    console.log('test');
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <div className="flex flex-col items-center justify-center h-screen gap-2">
                <h1 className="text-4xl">Login</h1>
                <form className="flex flex-col items-center justify-center gap-2">
                    <div className="grid grid-cols-2 grid-flow-row gap-2">
                        <label htmlFor="email">Email</label>
                        <input type="text" placeholder="Email" />
                        <label htmlFor="password">Password</label>
                        <input type="password" placeholder="Password" />
                    </div>
                    <button type="submit">Login</button>
                </form>
            </div>
        </Suspense>
    )
}