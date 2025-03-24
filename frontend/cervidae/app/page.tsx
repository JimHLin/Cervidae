'use client'
import DeerCard from "@/ui/deer-card";
import { gql, useQuery} from "urql";
import { useAuth } from "@/ui/auth-provider";
import { useState, useCallback } from "react";
import Switch from "@/ui/switch";
import Link from "next/link";
export default function Page(){
  const entriesPerPage = 2;
  const { isAuthenticated, isAdmin } = useAuth();
  const [seePending, setSeePending] = useState(false);
  const testQuery = gql`
    query ($first: Int, $after: String, $last: Int, $before: String) {
        ${seePending ? "deerPendingConnections" : "deerConnections"}(first: $first, after: $after, last: $last, before: $before) {
          edges{
            node{
              id
              name
              imageUrl
              description
              killCount
            }
          }
          pageInfo{
            hasNextPage
            hasPreviousPage
            totalCount
          }
        }
      }
    `;

  const [after, setAfter] = useState<string | null>(null);
  const [before, setBefore] = useState<string | null>(null);
  const [direction, setDirection] = useState<"forward" | "backward">("forward");
  const [currentPage, setCurrentPage] = useState(1);
  const [testResult, testExecuteQuery] = useQuery({
    query: testQuery,
    variables: direction === "forward"
      ? { first: entriesPerPage, after }
      : { last: entriesPerPage, before },
  });

  const handleNext = () => {
    if (pageInfo?.hasNextPage) {
      const lastEdge = items[items.length - 1];
      setAfter(lastEdge.id || null);
      setBefore(null);
      setDirection("forward");
      setCurrentPage(currentPage + 1);
    }
  };

  const handlePrevious = () => {
    if (pageInfo?.hasPreviousPage) {
      const firstEdge = items[0];
      setBefore(firstEdge.id || null);
      setAfter(null);
      setDirection("backward");
      setCurrentPage(currentPage - 1);
    }
  };
  const { data, fetching, error } = testResult;
  const dataToUse = seePending ? data?.deerPendingConnections : data?.deerConnections;
  const pageInfo = fetching ? null : dataToUse.length > 0 ? dataToUse[0].pageInfo : null;
  const items = fetching ? [] : dataToUse.length > 0 ? dataToUse[0].edges.map((edge: any) => edge.node) : [];
  const totalPages = fetching ? 0 : dataToUse.length > 0 ? Math.ceil(dataToUse[0].pageInfo.totalCount / entriesPerPage) : 0;
  
  if (fetching) return <p>Loading...</p>;
  if (error) return <p>Oh no... {error.message}</p>;
  return (
    <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
      {isAdmin && (
        <div className="flex flex-row justify-center items-center gap-4">
          <Link href="/deer/create">Create Deer</Link>
          <Switch onChange={setSeePending} value={seePending} />
        </div>
      )}
      <p className="text-xl text-gray-500">Terrifying creatures stalk these lands</p>
      <div className="flex flex-row gap-4 flex-wrap justify-evenly align-bottom transition-all duration-500">
        {items.map((deer: any) => (
          <DeerCard deer={deer} key={deer.id} />
        ))}
      </div>
        <div className="flex flex-row justify-center items-center gap-4">
          <button onClick={handlePrevious} disabled={fetching || currentPage === 1} className="bg-blue-500 text-white p-2 rounded-md disabled:bg-gray-500">
            Previous
          </button>
          <p className="text-gray-500">{currentPage} of {totalPages}</p>
          <button onClick={handleNext} disabled={fetching || currentPage === totalPages} className="bg-blue-500 text-white p-2 rounded-md disabled:bg-gray-500">
            Next
          </button>
        </div>
    </div>
  )
}