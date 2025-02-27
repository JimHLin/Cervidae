'use client'
import DeerCard from "@/ui/deer-card";
import { gql, useQuery} from "urql";

export default function Page(){
    const query = gql`
    query {
      deerAll {
        id
        name
        description
        imageUrl
        killCount
      }
    }
  `;
console.log(query);
  const [result, reexecuteQuery] = useQuery({query: query});
  const { data, fetching, error } = result;


  if (fetching) return <p>Loading...</p>;
  if (error) return <p>Oh no... {error.message}</p>;
  return (
    <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
      <p className="text-xl text-gray-500">Terrifying creature stalk these lands</p>
      <div className="flex flex-row gap-4 flex-wrap justify-evenly align-bottom">
        {data?.deerAll.map((deer: any) => (
          <DeerCard deer={deer} />
        ))}
      </div>
    </div>
  )
}