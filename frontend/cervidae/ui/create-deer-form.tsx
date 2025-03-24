'use client'
import { useState } from "react";
import Test from "./test";
import Bloody from "@/public/bloody.png";
import GenericInput from "./generic-input";
import { useAuth } from "@/ui/auth-provider";
import { useMutation, gql } from "urql";
import { useRouter } from "next/navigation";

export default function CreateDeerForm() {
  const [name, setName] = useState("");
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const [killCount, setKillCount] = useState(0);
  const router = useRouter();
  const { userId, isAdmin } = useAuth();
  const [editorContent, setEditorContent] = useState("");
  const [createDeer, executeCreateDeer] = useMutation(gql`
    mutation CreateDeer($input: CreateDeerInput!) {
      createDeer(input: $input) {
        id
        name
        description
        imageUrl
        killCount
      }
    }
  `);
  
  const handleCreateDeer = async () => {
    const input = await executeCreateDeer({
        input: {
            userId: userId,
            name: name,
            description: editorContent,
            killCount: killCount,
            imageUrl: imageUrl
        }
    })
    if (input.error) {
      console.error(input.error);
    }else{
      router.push('/');
    }
  };

  return (
    <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
        <div className="flex flex-row gap-2 items-center">
            <label htmlFor="name" className="min-w-16">Name</label>
            <GenericInput value={name} onChange={(value) => setName(value)} />
        </div>
      <div className="flex justify-center items-center w-full h-40 object-scale-down bg-green-900">
        <input type="file" onChange={(e) => setImageUrl(e.target.value)} />
      </div>
      <div className="flex flex-row gap-2 items-center">
        <div className="min-w-16 flex justify-center items-center">
          <img className="w-4 h-4" src={Bloody.src} alt="Bloody" />
        </div>
        <input className="border-2 border-gray-300 rounded-md p-2  dark:bg-gray-800 dark:text-white" type="number" value={killCount} onChange={(e) => setKillCount(Number(e.target.value))} />
      </div>
      <div className="rounded-lg p-2">
        <Test setValue={setEditorContent} />
      </div>
      <button onClick={handleCreateDeer}>Create Deer</button>
  </div>
  );
}