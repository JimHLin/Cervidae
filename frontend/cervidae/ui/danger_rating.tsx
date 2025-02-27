export default function DangerRating(){
    return (
        <div className="flex flex-row items-center gap-1 justify-center">
            {Array.from({length: 10}).map((_, index) => (
                <div key={index} className="rounded-full bg-yellow-500 w-2 h-2"
                    style={{
                        clipPath: 'polygon(50% 0, calc(50%*(1 + sin(.4turn))) calc(50%*(1 - cos(.4turn))), calc(50%*(1 - sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 + sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 - sin(.4turn))) calc(50%*(1 - cos(.4turn)))'
                    }}
                ></div>
            ))}
        </div>
    )
}