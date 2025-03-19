import { useState } from 'react'
import PasswordInput from './password-input'
export default function ResetPassword(props: {setShow: (show: boolean) => void, resetPassword: (input: any) => any, id: string}) {
    const [password, setPassword] = useState("");
    const [newPassword, setNewPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [error, setError] = useState("");
    const [matching, setMatching] = useState(true);

    const submit = async () => {
        if(newPassword !== confirmPassword) {
            setMatching(false);
            return;
        }
        let res = await props.resetPassword({
            input: {
                id: props.id,
                currentPassword: password,
                newPassword: newPassword,
            }
        })
        if(res.error) {
            setError(res.error);
        } else {
            setMatching(true);
            props.setShow(false);
        }
    }
    return (
        <div className="fixed w-screen flex justify-center items-center" onClick={() => {props.setShow(false);}}>
            <div className=" z-40 rounded-md bg-gray-800 p-4" onClick={(e) => e.stopPropagation()}>
                <form className="flex flex-col gap-2">
                    <label htmlFor="title">Current Password</label>
                    <PasswordInput value={password} onChange={(value) => setPassword(value)} autoFocus={true} />
                    <label htmlFor="newPassword">New Password</label>
                    <PasswordInput value={newPassword} onChange={(value) => setNewPassword(value)} />
                    <label htmlFor="confirmPassword">Confirm New Password</label>
                    <PasswordInput value={confirmPassword} onChange={(value) => setConfirmPassword(value)} />
                {error && <p>{error}</p>}
                {!matching && <p className="text-red-400 text-sm">Passwords do not match</p>}
                <button type="button" onClick={submit}>Submit</button>
            </form>
        </div>
        </div>
    )
}