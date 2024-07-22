import { useDispatch, useSelector } from "react-redux"
import { RootState } from "../../state-hook/store";
import { useEffect, useRef } from "react";
import { closeSidePanel } from "../../state-hook/state/side-panel/accountListSlice";

export default function AccountPanel() {
    
    const open = useSelector((state:RootState) => state.accountPanel.open);
    const item = useRef<HTMLDivElement>(null)
    const dispatch = useDispatch()

    useEffect(() => {
        let handler = (e:any) =>{
            console.log("close now!");
            if(!item.current?.contains(e.target)){
                dispatch(closeSidePanel())
            }
        }

        setTimeout(() => {
            document.addEventListener("click", handler)
        },1)

        return () => {
            document.removeEventListener("click", handler)
        }
    })
    
    return ( open?
        <div className="absolute bg-error w-96 bottom-1 left-0 mx-20 rounded my-4 p-2 flex flex-col-reverse shadow-lg delay-1000 animate-fade" ref={item}>
            <div>123</div>
            <div>123</div>
            <div>123</div>
            <div>1232222222222222</div>
        </div>:null
    )

}            
            
            
