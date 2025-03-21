'use client'
import { useState } from "react";
import Tiptap from "./tiptap";
import Test from "./test";

export default function CreateDeerForm() {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [imageUrl, setImageUrl] = useState("");
  const [killCount, setKillCount] = useState(0);

  const handleCreateDeer = () => {
    console.log(name, description, imageUrl, killCount);
  };

  return (
    <div className="flex flex-col items-center justify-center w-10/12 m-auto pt-16 gap-5">
        <div className="flex flex-row gap-2">
            <label htmlFor="name">Name</label>
            <input type="text" value={name} onChange={(e) => setName(e.target.value)} />
        </div>
      <div className="flex justify-center items-center w-full h-40 object-scale-down bg-green-900">
        <input type="file" onChange={(e) => setImageUrl(e.target.value)} />
      </div>
      <input type="text" value={description} onChange={(e) => setDescription(e.target.value)} />
      <input type="number" value={killCount} onChange={(e) => setKillCount(Number(e.target.value))} />
      <div className="rounded-lg p-2">
        <Test />
        <Tiptap />
      </div>
      <button onClick={handleCreateDeer}>Create Deer</button>
  </div>
  );
}