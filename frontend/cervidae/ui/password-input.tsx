import eyeOpen from '@/public/eye_open.svg'
import eyeClosed from '@/public/eye_closed.svg'
import { useState } from 'react'
export default function PasswordInput(props: { value: string, onChange: (value: string) => void, autoFocus?: boolean }){
    const [show, setShow] = useState(false);
    return (
        <div className="relative">
            <div className="flex items-center justify-between border-2 dark:border-gray-300 dark:bg-gray-900 rounded-md p-2 gap-2">
            <input autoComplete="none" className="border-b-2 dark:border-gray-500 bg-transparent" autoFocus={props.autoFocus} type={show ? "text" : "password"} name="password" value={props.value} onChange={(e) => props.onChange(e.target.value)}/>
                <button type="button" className="" onClick={() => setShow(!show)}>
                    <img src={show ? eyeOpen.src : eyeClosed.src} alt="eye" width={20} height={20} className="dark:invert"/>
                </button>
            </div>
        </div>
    )
}