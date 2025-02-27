import '@/app/globals.css';
import DangerRating from './danger_rating';
import Bloody from '@/public/bloody.png'

export default function DeerCard(deer: {deer: any}){
  return (
    <div className="flex flex-col justify-center align-middle p-2 dark:bg-gray-800 border-2 border-green-900  rounded-lg w-64 gap-1">
        <div className="flex flex-row justify-center items-center">
            <img src={deer.deer.imageUrl} alt="Deer" onError={(e) => {
                e.currentTarget.src = "https://i.postimg.cc/L69Q7Xzf/defaultdeer.webp";
            }} width="auto" height="auto" className="w-full h-40 object-scale-down" />
        </div>
        <p className="text-2xl font-bold text-center">{deer.deer.name}</p>
        <div className="flex flex-row justify-center gap-2 items-center">
            <img className="w-4 h-4" src={Bloody.src} alt="Bloody" />
            <p>10</p>
        </div>
        <DangerRating />
        <p className="text-sm text-center text-gray-500 dark:text-gray-400 overflow-hidden text-ellipsis whitespace-nowrap">
            Deer Description that is very long and takes up a lot of space. There is even more text that will never end, followed by even more text that will never ever end.
        </p>
    </div>
  )
}