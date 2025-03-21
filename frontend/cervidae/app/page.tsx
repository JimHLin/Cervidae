'use client'
import DeerCard from "@/ui/deer-card";
import { gql, useQuery} from "urql";
import { useAuth } from "@/ui/auth-provider";
import { useState, useCallback } from "react";
import Switch from "@/ui/switch";
import Link from "next/link";
export default function Page(){

  const pendingQuery = gql`
    query {
      deerPending {
        id
        name
        description
        imageUrl
        killCount
      }
    }
  `;
  const entriesPerPage = 2;
  const { isAuthenticated, isAdmin } = useAuth();
  const [seePending, setSeePending] = useState(false);
  const [pendingResult, pendingExecuteQuery] = useQuery({query: pendingQuery});
  const testQuery = gql`
    query ($first: Int, $after: String, $last: Int, $before: String) {
        deerConnections(first: $first, after: $after, last: $last, before: $before) {
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
    if (testResult?.data?.deerConnections[0].pageInfo.hasNextPage) {
      const lastEdge = testResult.data.deerConnections[0].edges[testResult.data.deerConnections[0].edges.length - 1];
      setAfter(lastEdge?.node.id || null);
      setBefore(null);
      setDirection("forward");
      setCurrentPage(currentPage + 1);
    }
  };

  const handlePrevious = () => {
    if (testResult?.data?.deerConnections[0].pageInfo.hasPreviousPage) {
      const firstEdge = testResult.data.deerConnections[0].edges[0];
      setBefore(firstEdge?.node.id || null);
      setAfter(null);
      setDirection("backward");
      setCurrentPage(currentPage - 1);
    }
  };
  const { data, fetching, error } = testResult;
  const items = data?.deerConnections[0].edges.map((edge: any) => edge.node);
  const totalPages = Math.ceil(data?.deerConnections[0].pageInfo.totalCount / entriesPerPage);
  console.log(items);
  
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