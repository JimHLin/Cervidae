'use client'
import {useState} from 'react'
import { usePathname, redirect } from 'next/navigation'
import { useAuth } from '@/ui/auth-provider'

export default function Layout({ children }: { children: React.ReactNode }) {
    const pathname = usePathname();
    let patharray = pathname.split('/');
    const { userId } = useAuth();
    const [active, setActive] = useState(() => {
        switch (patharray[patharray.length - 1]) {
            case 'profile':
                return 0;
            case 'reviews':
                return 1;
            case 'comments':
                return 2;
            default:
                return 0;
        }
    });
    const switchtab = (tab: number) => {
        setActive(tab);
        switch (tab) {
            case 0:
                redirect(pathname.substring(0, pathname.lastIndexOf('/')) + '/profile');
            case 1:
                redirect(pathname.substring(0, pathname.lastIndexOf('/')) + '/reviews');
            case 2:
                redirect(pathname.substring(0, pathname.lastIndexOf('/')) + '/comments');
        }
    }
    return (
        <div>
            <div className="flex flex-row w-full justify-evenly bg-green-900 shadow-inner shadow-black flex-1">
                {!userId || userId == patharray[patharray.length - 2] && (
                <button onClick={() => {switchtab(0)}} className={`flex-1 p-1  ${active === 0 ? 'bg-green-950 shadow-inner shadow-black' : 'hover:bg-green-800 hover:shadow-inner hover:shadow-black'}`}>Profile</button>
                )}
                <button onClick={() => {switchtab(1)}} className={`flex-1 p-1  ${active === 1 ? 'bg-green-950 shadow-inner shadow-black' : 'hover:bg-green-800 hover:shadow-inner hover:shadow-black'}`}>Reviews</button>
                <button onClick={() => {switchtab(2)}} className={`flex-1 p-1  ${active === 2 ? 'bg-green-950 shadow-inner shadow-black' : 'hover:bg-green-800 hover:shadow-inner hover:shadow-black'}`}>Comments</button>
            </div>
            {children}
        </div>
    )
}