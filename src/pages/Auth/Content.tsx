import {CharBox} from "../../component/CharBox.tsx";
import {useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {open} from '@tauri-apps/plugin-shell'
import {writeText} from "@tauri-apps/plugin-clipboard-manager";

// #[derive(Debug,Clone,Serialize)]
// pub struct DevicecodeInfo{
//     url:String,
//     code:String,
//     expiring_in:Duration
// }
interface Duration{
    secs: number,
    nanos: number
}


interface DevicecodePayload {
    url: string,
    code: string,
    expiring_in: Duration
}

interface ExchangePayload {
    action: "Pending" | "Success",
    second: number
}

enum Status{
    Unlock,
    Lock,
    Error,
    Done
}

interface Error{
    status: "error" | "CountryBan" | "NeedAdultVerification" | "AddToFamily",
    error: string
}

const refresh =
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
                     stroke="currentColor" className="size-5 cursor-pointer text-base-content active:rotate-180 duration-100 transform-gpu">
        <path strokeLinecap="round" strokeLinejoin="round"
            d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
    </svg>

const refresh_disable =
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
         stroke="currentColor" className="size-5 text-base-content/50">
        <path strokeLinecap="round" strokeLinejoin="round"
              d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
    </svg>

export function Content() {

    const [devicecode, setDevicecode] = useState<DevicecodePayload | null>(null)
    const [message, setMessage] = useState("Fetching device code...")
    const [status, setStatus] = useState(Status.Lock)

    const exchange = useRef<number | null>(null); // the id of setTimeout that check device code status
    const update = useRef<number | null>(null); // the id of setTimeout that device code will expire
    const lock = useRef(false)

    const open_browser = () => {
        open(devicecode!!.url).catch(console.error)
        writeText(devicecode!!.code).catch(console.error)
    }


    const cleanup = () => {
        if (exchange.current) {
            clearTimeout(exchange.current)
            exchange.current = null
        }

        if (update.current){
            clearTimeout(update.current)
            update.current = null
        }
    }

    const handle_error = (error:Error,message:string) => {
        cleanup()
        setStatus(Status.Error)
        console.error(error.error)

        switch (error.status){
            case "NeedAdultVerification": setMessage("You need to adult verification to login!"); break;
            case "CountryBan": setMessage("Your country is banned from login!"); break;
            case "AddToFamily": setMessage("You need to add to family to login!"); break;
            default: setMessage(message)
        }

    }

    const delay = (ms:number) => new Promise(res => setTimeout(res, ms));

    const refresh_token = async () =>{

        cleanup()
        setStatus(Status.Lock)
        setMessage("Refreshing code...")
        // if we run exchange and refresh at the same time, we will get deadlock.
        // this is a temporary solution to prevent deadlock
        while (lock.current){
            await delay(100)
        } // wait until the lock is released
        lock.current = true
        invoke<DevicecodePayload>("refresh").then((data)=>{
            setStatus(Status.Unlock)
            update.current = setTimeout(()=>{
                refresh_token()
            },data.expiring_in.secs * 1000)
            setMessage("Waiting user to complete auth flow......")
            setDevicecode(data)
            exchange.current = setTimeout(() => {
                exchange_loop()
            }, 5 * 1000)
            lock.current = false
        }).catch((error)=> {
            lock.current=false
            handle_error(error, "Error while refresh the devicecode token!")
        })
    }

    const exchange_loop = async () => {

        while (lock.current){
            await delay(100)
        } // wait until the lock is released

        lock.current = true
        setMessage("Waiting user to complete auth flow......")
        invoke<ExchangePayload>("exchange").then((data) => {
            lock.current = false
            if (data.action === "Success")
                xbox_live()
            else
                exchange.current = setTimeout(() => {
                    exchange_loop()
                }, data.second * 1000)
        }).catch((error)=> {
            handle_error(error, "Error when exchange devicecode data!")
            lock.current = false
        })
    }

    const xbox_live = () => {
        setStatus(Status.Lock)
        setMessage("Fetching XBOX Live data...")
        invoke("xbox_live").then(()=> {
            console.log("XBOX LiveDone")
            xbox_security()
        }).catch((err)=>handle_error(err,"Error when fetching XBOX Live data!"))
    }

    const xbox_security = () => {
        setStatus(Status.Lock)
        setMessage("Fetching XBOX Security data...")
        invoke("xbox_security").then(()=> {
            console.log("XBOX Security Done")
            account_check()
        }).catch((err)=>handle_error(err,"Error when fetching XBOX Security data!"))
    }

    const account_check = ()=>{
        setStatus(Status.Lock)
        setMessage("Fetching Your Account Data...")
        invoke("account").then(()=> {
            setStatus(Status.Done)
            setMessage("Done! You can close this window now!")
            console.log("XBOX Security Done")
        }).catch((err)=>handle_error(err,"Error when fetching your account data!"))
    }

    useEffect(() => {

        invoke<DevicecodePayload>("devicecode").then((data) => {

            update.current = setTimeout(()=>{
                refresh_token().catch(console.error) // refresh in data.expiring_in.secs
            },data.expiring_in.secs * 1000)

            setStatus(Status.Unlock)
            setDevicecode(data)
            exchange.current = setTimeout(() => {
                exchange_loop().catch(console.error)
            }, 5 * 1000)

        }).catch((err)=>handle_error(err,"Error while fetching devicecode"))

        return () => { // cleanup timeout function
            cleanup()
        }

    }, [setMessage])

    return <div>
        <h1 className="w-full px-2 text-4xl"> How to login </h1>
        <ol className="list-inside list-decimal w-[28rem] px-2 py-6">
            <li>Click login code to copy.</li>
            <li>Open <a className="cursor-pointer text-primary">{devicecode?.url}</a> in your browser or click the
                button down below to open in browser.
            </li>
            <li>Paste code in web and follow the step Microsoft tell you!</li>
            <li>Once you are done, nolauncher will obtain the permission to fetch your minecraft data!</li>
        </ol>
        <div className="flex flex-row gap-2 px-2 items-center">
            <h1 className="text-2xl">Your login code:</h1>
            <div className="grow"></div>

            {status == Status.Lock ?
                <div>{refresh_disable}</div> :
                <div onClick={() => refresh_token()}>{refresh}</div>
            }
        </div>
        <CharBox chars={devicecode?.code} enable={!status}/>
        <div className="flex flex-row px-2 gap-2">
            {status == Status.Lock ||  status == Status.Unlock ?<span className="loading loading-spinner loading-xs"></span>:null}
            <h1 className={status == Status.Error?"text-error":""}>{message}</h1>
        </div>
        <div className="flex flex-row-reverse p-2">
            <button disabled={status!=Status.Unlock} onClick={open_browser} className="btn outline outline-1 w-full bg-base-100">Copy code and Open Link in browser</button>
        </div>

    </div>
}