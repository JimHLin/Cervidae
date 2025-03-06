import { Suspense } from 'react';

export default function LoginPage(){
    console.log('test');
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <div className="flex flex-col items-center justify-center h-screen">
                <h1 className="text-4xl">Login</h1>
                <form className="flex flex-col items-center justify-center gap-2">
                    <input type="text" placeholder="Username" />
                    <input type="password" placeholder="Password" />
                    <button type="submit">Login</button>
                </form>
            </div>
        </Suspense>
    )
}