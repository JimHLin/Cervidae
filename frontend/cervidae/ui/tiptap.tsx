'use client'

import { useEditor, EditorContent, useCurrentEditor } from '@tiptap/react'

import StarterKit from '@tiptap/starter-kit'


const Tiptap = () => {
  const editor = useEditor({
    extensions: [StarterKit],
    content: '<p>Hello World! 🌎️</p>',
  })
  const current = useCurrentEditor();
  return <EditorContent editor={current ? current.editor : editor} />
}

export default Tiptap
