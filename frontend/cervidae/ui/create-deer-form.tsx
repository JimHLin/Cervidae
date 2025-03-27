'use client'
import { useState } from "react";
import Test from "./test";
import Bloody from "@/public/bloody.png";
import GenericInput from "./generic-input";
import { useAuth } from "@/ui/auth-provider";
import { useMutation, gql } from "urql";
import { useRouter } from "next/navigation";
import striptags from "striptags";
const createDeerMutation = gql`
mutation CreateDeer($input: CreateDeerInput!) {
  createDeer(input: $input) {
    id
    name
    description
    imageUrl
    killCount
  }
}
`;

const getUploadUrlMutation = gql`
mutation GetUploadUrl($contentType: String!) {
  getUploadUrl(contentType: $contentType)
}
`;

export default function CreateDeerForm() {
  const [name, setName] = useState("");
  const [file, setFile] = useState<{file: File, src: string} | null>(null);
  const [killCount, setKillCount] = useState(0);
  const router = useRouter();
  const { userId, isAdmin } = useAuth();
  const [editorContent, setEditorContent] = useState("");
  const [createDeer, executeCreateDeer] = useMutation(createDeerMutation);
  const [getUploadUrl, executeGetUploadUrl] = useMutation(getUploadUrlMutation);
  
  const handleCreateDeer = async () => {
    let uploadedImageUrl = null;
    if(striptags(editorContent).length === 0) {
      alert("Please enter a description");
      return;
    }
    if (file) {
      let res = await executeGetUploadUrl({
        contentType: file.file.type
      });
  
      if (res.data?.getUploadUrl) {
        uploadedImageUrl = await uploadToS3(file.file, res.data.getUploadUrl);
        console.log("Uploaded URL:", uploadedImageUrl);
      } else {
        console.error("Upload failed", res.error);
        return;
      }
    }
  
    const input = {
      userId: userId,
      name: name,
      description: editorContent,
      killCount: killCount,
      imageUrl: uploadedImageUrl,
    };
  
    console.log("Payload:", input);
  
    let createRes = await executeCreateDeer({input: input});
    if (createRes.error) {
      console.error("Mutation failed:", createRes.error);
    } else {
      //router.push('/');
    }
  };
  

  const uploadToS3 = async (file: File, uploadUrl: string) => {
    const response = await fetch(uploadUrl, {
      method: "PUT",
      body: file,
      headers: {
        "Content-Type": file.type,
      },
    });
    if (!response.ok) {
      throw new Error("Upload failed");
    }
    console.log(uploadUrl.split("?")[0]);
    return uploadUrl.split("?")[0]; // Get the file URL without query params
  };
  

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        setFile({file: file, src: e.target?.result as string});
      };
      reader.readAsDataURL(file);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
        <div className="flex flex-row gap-2 items-center">
            <label htmlFor="name" className="min-w-16">Name</label>
            <GenericInput value={name} onChange={(value) => setName(value)} />
        </div>
      <div className="flex justify-center items-center w-full h-40 object-scale-down bg-green-900">
        <input type="file" onChange={handleFileChange} accept="image/*"/>
        <img width={50} height={50} src={file?.src ?? "https://i.postimg.cc/L69Q7Xzf/defaultdeer.webp"} alt="Deer" />
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