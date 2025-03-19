'use client'
import Link from 'next/link';
import { useAuth } from './auth-provider';
import Profile from '@/public/profile.svg';
import { redirect } from 'next/navigation';

export default function Header() {
    const { isAuthenticated, login, logout, userId } = useAuth();
    return (
        <div className="flex flex-col items-center justify-center h-10 bg-green-800 fixed w-full z-50">
          <Link href="/" className="text-white font-serif text-2xl tracking-wider">Cervidae</Link>
          <div className="flex flex-row gap-4 absolute right-5 mr-4">
            {!isAuthenticated && (
                <>
                    <Link href="/auth" className="hover:text-gray-300 hover:shadow-md hover:shadow-black rounded-md px-2 py-1">Sign In</Link>
                    <Link href="/auth/register" className="hover:text-gray-300 hover:shadow-md hover:shadow-black rounded-md px-2 py-1">Sign Up</Link>
                </>
            )}
            {isAuthenticated && (
                <div className="flex flex-row items-center">
                    <Link href={`/user/${userId}/profile`} className="hover:text-gray-300 hover:shadow-md hover:shadow-black rounded-md px-2 py-1">
                        <img src={Profile.src} alt="Profile" width={20} height={20} className="cursor-pointer invert" />
                    </Link>
                    <button className="hover:text-gray-300 hover:shadow-md hover:shadow-black rounded-md px-2 py-1" onClick={logout}>Logout</button>
                </div>
            )}
          </div>
        </div>
    )
}
