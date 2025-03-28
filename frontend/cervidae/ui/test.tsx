'use client'
import { useState } from 'react'
import { Color } from '@tiptap/extension-color'
import ListItem from '@tiptap/extension-list-item'
import TextStyle from '@tiptap/extension-text-style'
import { EditorProvider, useCurrentEditor } from '@tiptap/react'
import StarterKit from '@tiptap/starter-kit'
import React from 'react'
import bold from '../public/bold.svg'
import italic from '../public/italic.svg'
import strike from '../public/strike.svg'
import code from '../public/code.svg'
import clear from '../public/clear.svg'
import paragraph from '../public/paragraph.svg'
import h1 from '../public/heading1.svg'
import h2 from '../public/heading2.svg'
import h3 from '../public/heading3.svg'
import bulletList from '../public/bullet-list.svg'
import orderedList from '../public/ordered-list.svg'
import blockquote from '../public/quote.svg'
import horizontalRule from '../public/horizontal-rule.svg'
const MenuBar = () => {
  const { editor } = useCurrentEditor()

  if (!editor) {
    return null
  }
  const [color, setColor] = useState('#000000')
  return (
    <div className="control-group">
      <div className="button-group">
        <button
          onClick={() => editor.chain().focus().toggleBold().run()}
          disabled={
            !editor.can()
              .chain()
              .focus()
              .toggleBold()
              .run()
          }
          className={editor.isActive('bold') ? 'is-active' : ''}
          title="Bold"
        >
          <img src={bold.src} alt="bold" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleItalic().run()}
          disabled={
            !editor.can()
              .chain()
              .focus()
              .toggleItalic()
              .run()
          }
          className={editor.isActive('italic') ? 'is-active' : ''}
          title="Italic"
        >
          <img src={italic.src} alt="italic" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleStrike().run()}
          disabled={
            !editor.can()
              .chain()
              .focus()
              .toggleStrike()
              .run()
          }
          className={editor.isActive('strike') ? 'is-active' : ''}
          title="Strike"
        >
          <img src={strike.src} alt="strike" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleCode().run()}
          disabled={
            !editor.can()
              .chain()
              .focus()
              .toggleCode()
              .run()
          }
          className={editor.isActive('code') ? 'is-active' : ''}
          title="Code"
        >
          <img src={code.src} alt="code" className='w-4 h-4 dark:invert'/>
        </button>
        <button onClick={() => editor.chain().focus().unsetAllMarks().run()} title="Clear marks">
          <img src={clear.src} alt="clear" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().setParagraph().run()}
          className={editor.isActive('paragraph') ? 'is-active' : ''}
          title="Paragraph"
        >
          <img src={paragraph.src} alt="paragraph" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
          className={editor.isActive('heading', { level: 1 }) ? 'is-active' : ''}
          title="Heading 1"
        >
          <img src={h1.src} alt="h1" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
          className={editor.isActive('heading', { level: 2 }) ? 'is-active' : ''}
          title="Heading 2"
        >
          <img src={h2.src} alt="h2" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleHeading({ level: 3 }).run()}
          className={editor.isActive('heading', { level: 3 }) ? 'is-active' : ''}
          title="Heading 3"
        >
          <img src={h3.src} alt="h3" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleBulletList().run()}
          className={editor.isActive('bulletList') ? 'is-active' : ''}
          title="Bullet list"
        >
          <img src={bulletList.src} alt="bullet list" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleOrderedList().run()}
          className={editor.isActive('orderedList') ? 'is-active' : ''}
          title="Ordered list"
        >
          <img src={orderedList.src} alt="ordered list" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().toggleBlockquote().run()}
          className={editor.isActive('blockquote') ? 'is-active' : ''}
          title="Blockquote"
        >
          <img src={blockquote.src} alt="blockquote" className='w-4 h-4 dark:invert'/>
        </button>
        <button onClick={() => editor.chain().focus().setHorizontalRule().run()} title="Horizontal rule">
          <img src={horizontalRule.src} alt="horizontal rule" className='w-4 h-4 dark:invert'/>
        </button>
        <button
          onClick={() => editor.chain().focus().setColor(color).run()}
          className={editor.isActive('textStyle', { color: color }) ? 'is-active' : ''}
        >
          Color
        </button>
        <input type="color" value={color} onChange={(e) => {setColor(e.target.value);}} />
      </div>
    </div>
  )
}

const extensions = [
  Color.configure({ types: [TextStyle.name, ListItem.name] }),
  TextStyle.configure({  }),
  StarterKit.configure({
    bulletList: {
      keepMarks: true,
      keepAttributes: false, // TODO : Making this as `false` becase marks are not preserved when I try to preserve attrs, awaiting a bit of help
    },
    orderedList: {
      keepMarks: true,
      keepAttributes: false, // TODO : Making this as `false` becase marks are not preserved when I try to preserve attrs, awaiting a bit of help
    },
  }),
]

const content = ``

export default (props: {setValue: (value: string) => void}) => {
  return (
    <EditorProvider slotBefore={<MenuBar />} extensions={extensions} content={content} immediatelyRender={false} onUpdate={({ editor }) => {
      props.setValue(editor.getHTML())
    }}></EditorProvider>
  )
}