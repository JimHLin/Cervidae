'use client'
import Link from 'next/link';
import { useAuth } from './auth-provider';
export default function Header() {
    const { isAuthenticated, login, logout } = useAuth();
    
    return (
        <div className="flex flex-col items-center justify-center h-10 bg-green-800 fixed w-full z-50">
          <Link href="/" className="text-white font-serif text-2xl tracking-wider">Cervidae</Link>
          <div className="flex flex-row gap-4 absolute right-0">
            {isAuthenticated ? (
              <button onClick={logout}>Logout</button>
            ) : (
              <button onClick={login}>Login</button>
            )}
          </div>
        </div>
    )
}
