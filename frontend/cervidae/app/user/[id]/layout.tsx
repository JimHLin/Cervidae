'use client'
import {useState} from 'react'

export default function Layout({ children }: { children: React.ReactNode }) {
    const [active, setActive] = useState(0);
    return (
        <div>
            <div className="flex flex-row w-full justify-evenly bg-green-900 shadow-inner shadow-black flex-1">
                <button onClick={() => setActive(0)} className={`flex-1 p-1  ${active === 0 ? 'bg-green-950 shadow-inner shadow-black' : 'hover:bg-green-800 hover:shadow-inner hover:shadow-black'}`}>Profile</button>
                <button onClick={() => setActive(1)} className={`flex-1 p-1  ${active === 1 ? 'bg-green-950 shadow-inner shadow-black' : 'hover:bg-green-800 hover:shadow-inner hover:shadow-black'}`}>Reviews</button>
                <button onClick={() => setActive(2)} className={`flex-1 p-1  ${active === 2 ? 'bg-green-950 shadow-inner shadow-black' : 'hover:bg-green-800 hover:shadow-inner hover:shadow-black'}`}>Comments</button>
            </div>
            {children}
        </div>
    )
}