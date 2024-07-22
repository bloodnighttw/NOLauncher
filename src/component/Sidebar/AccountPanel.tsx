import { useDispatch, useSelector } from "react-redux"
import { RootState } from "../../state-hook/store";
import { useEffect, useRef } from "react";
import { closeSidePanel } from "../../state-hook/state/side-panel/accountListSlice";
import { Account, logoutAccount, switchAccount } from "../../state-hook/state/account/accountSlice";
import { useNavigate } from "react-router-dom";

const logout = 
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
  <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0 0 13.5 3h-6a2.25 2.25 0 0 0-2.25 2.25v13.5A2.25 2.25 0 0 0 7.5 21h6a2.25 2.25 0 0 0 2.25-2.25V15m3 0 3-3m0 0-3-3m3 3H9" />
</svg>

const settings=
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
  <path strokeLinecap="round" strokeLinejoin="round" d="M6 13.5V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 9.75V10.5" />
</svg>

const new_account = 
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
  <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
</svg>




export default function AccountPanel() {
    
    const nav= useNavigate()
    const open = useSelector((state:RootState) => state.accountPanel.open)
    const accounts = useSelector((state:RootState) => state.account.accounts)
    const me = useSelector((state:RootState) => state.account.userNow)
    const item = useRef<HTMLDivElement>(null)
    const dispatch = useDispatch()

    useEffect(() => {


        let handler = (e:any) =>{
            if(open && !item.current?.contains(e.target)){
                console.log(item.current)
                dispatch(closeSidePanel())
            }
        }

        setTimeout(() => document.addEventListener("click", handler),1)

        return () => {
            document.removeEventListener("click", handler)
        }
    })

    const switch_account = (account:Account) =>{
        dispatch(switchAccount(account.id))
        dispatch(closeSidePanel())
    }
    
    return ( open?
        <div className="absolute bg-base-100 w-96 bottom-1 left-0 mx-20 rounded my-2 p-2 flex flex-col-reverse shadow-lg delay-1000 animate-fade transform-gpu gap-1" ref={item}>

            <div className="flex p-2 hover:bg-base-300/75 duration-200 rounded cursor-pointer" onClick={()=>nav('auth')}>
                <div className="m-auto justify-center">
                    {new_account}
                </div>
            </div>  

            {accounts.map((account,index) => (
                <div className={"flex gap-4 p-2 hover:bg-base-300/75 duration-200 rounded "+(me == account.id ? "bg-base-200 outline-1" :"")} key={index}>
                    <img className="w-8 h-8 bg-base-100 rounded flex-none cursor-pointer" src={"https://crafatar.com/avatars/" + account.id} onClick={()=>switch_account(account)}/>
                    <div className="my-auto flex-grow cursor-pointer" onClick={()=>switch_account(account)}>{account.name}</div>
                    <div className="my-auto h-8 w-8 p-1 active:scale-90 duration-200 cursor-pointer" onClick={()=>nav("login/"+account.id)}>{settings}</div>
                    <div className="my-auto h-8 w-8 p-1 active:scale-90 duration-200 text-red-500 cursor-pointer" onClick={()=>dispatch(logoutAccount(account.id))}>{logout}</div>
                </div>  
            ))}


        </div>:null
    )

}            
            
            
