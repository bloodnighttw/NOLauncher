import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import {useState} from "react";


interface Props{
    chars:string|null|undefined
    onClick?: ()=>void
    err?:boolean
        enable:boolean
}

export function CharBox(props:Props){

    const chars = props.chars?.split("")
    const [click,setClick] = useState(false)
    const [show,setShow] = useState(false);

    const disabled = "text-3xl bg-base-100 rounded w-12 h-16 text-center shadow outline outline-1 content-center outline-gray-300 duration-200 opacity-50"
    const notClicked = "text-3xl bg-base-100 rounded w-12 h-16 text-center shadow outline outline-1 content-center active:scale-90 duration-200 "
    const clicked = "text-3xl bg-base-100 rounded w-12 h-16 text-center shadow outline outline-1 content-center active:scale-90 duration-200 outline-green-500"
    const error = "text-3xl bg-base-100 rounded w-12 h-16 text-center shadow outline outline-1 content-center active:scale-90 duration-200 outline-red-500"


    return <div className="relative">
        <div className="grid grid-cols-8 gap-2 p-2 select-none cursor-pointer" onClick={() => {
            if(props.chars && props.enable) {

                if(!show){
                    setTimeout(() => {
                        setShow(false)
                    }, 1000)
                }

                if(!click) {
                    setTimeout(()=>{
                        setClick(false)
                    },2000)
                }

                setClick(true)
                setShow(true)

                writeText(props.chars!!).catch(console.error)
                props.onClick?.()
            }
        }}>
            {
                !chars || !props.enable ? (chars ? chars!! : "        ".split("")).map((e,index)=> (
                    <div
                        key={index}
                        className={disabled}>
                        {e}
                    </div>
                )) :
                props.err ? chars!!.map((e,index) => ( // webkit2gtk only support this syntax
                    <div
                        key={index}
                        className={error}>
                        {e}
                    </div>
                )) :
                click ? chars!!.map((e,index) => ( // webkit2gtk only support this syntax
                    <div
                        key={index}
                        className={clicked}>
                        {e}
                    </div>
                )) : chars!!.map((e,index) => (
                    <div
                        key={index}
                        className={notClicked}>
                        {e}
                    </div>
                ))
            }
        </div>

        {show ?
            <div
                className={"absolute left-1/2 -translate-x-1/2 bg-base-100 p-2 rounded text-center"}>
                Copy!
            </div> : null

        }


    </div>

}