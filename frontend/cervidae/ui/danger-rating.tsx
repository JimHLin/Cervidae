export default function DangerRating(props: {rating: any}){
    return (
        <div className="flex flex-row items-center gap-1 justify-center">
            {Array.from({length: 10}).map((_, index) => {
                if(props.rating >= index + 1){
                    return (
                        <div key={index} className="rounded-full bg-yellow-500 w-2 h-2"
                            style={{
                        clipPath: 'polygon(50% 0, calc(50%*(1 + sin(.4turn))) calc(50%*(1 - cos(.4turn))), calc(50%*(1 - sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 + sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 - sin(.4turn))) calc(50%*(1 - cos(.4turn)))'
                    }}
                        ></div>
                    )
                }else if(props.rating > index){
                    let percentage = Math.round((index + 1 - props.rating) * 100);
                    return(
                        <div key={index} className="rounded-full w-2 h-2"
                            style={{
                                clipPath: 'polygon(50% 0, calc(50%*(1 + sin(.4turn))) calc(50%*(1 - cos(.4turn))), calc(50%*(1 - sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 + sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 - sin(.4turn))) calc(50%*(1 - cos(.4turn)))',
                                background: `linear-gradient(to right, #eab308 ${100 - percentage}%, #6b7280 ${percentage}%)`
                            }}></div>
                    )
                }else{
                    return (
                        <div key={index} className="rounded-full bg-gray-500 w-2 h-2" style={{
                            clipPath: 'polygon(50% 0, calc(50%*(1 + sin(.4turn))) calc(50%*(1 - cos(.4turn))), calc(50%*(1 - sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 + sin(.2turn))) calc(50%*(1 - cos(.2turn))), calc(50%*(1 - sin(.4turn))) calc(50%*(1 - cos(.4turn)))'
                        }}></div>
                    )
                }
            })}
        </div>
    )
}