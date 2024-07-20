import {CharBox} from "../../component/CharBox.tsx";
import {useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

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

export function Devicecode() {

    const [devicecode, setDevicecode] = useState<DevicecodePayload | null>(null)
    const [message, setMessage] = useState("Fetching device code...")
    const [lock, setLock] = useState(true)

    const exchange = useRef<number | null>(null);
    const update = useRef<number | null>(null);

    const cleanup = () => {
        if (exchange.current) {
            clearTimeout(exchange.current)
        }

        if (update.current){
            clearTimeout(update.current)
        }

    }

    const refresh_token = () =>{
        setLock(true)
        setMessage("Refreshing code...")
        cleanup()
        invoke<DevicecodePayload>("refresh").then((data)=>{

            update.current = setTimeout(()=>{
                refresh_token()
            },data.expiring_in.secs * 1000)

            setMessage("Waiting user to complete auth flow......")
            setDevicecode(data)
            exchange_loop()
            setLock(false)
        }).catch(console.error)
    }

    const xbox_live = () => {
        setMessage("Fetching XBOX Live data...")
        invoke("xbox_live").then(()=> {
            console.log("XBOX LiveDone")
            xbox_security()
        }).catch(console.error)
    }

    const xbox_security = () => {
        setMessage("Fetching XBOX Security data...")
        invoke("xbox_security").then(()=> {
            console.log("XBOX Security Done")
        }).catch(console.error)
    }

    const exchange_loop = () => {
        setMessage("Waiting user to complete auth flow......")
        invoke<ExchangePayload>("exchange").then((data) => {
            if (data.action === "Success") {
                xbox_live()
            } else
                exchange.current = setTimeout(() => {
                    exchange_loop()
                }, data.second * 1000)
        }).catch(console.error)
    }

    useEffect(() => {

        invoke<DevicecodePayload>("devicecode").then((data) => {


            update.current = setTimeout(()=>{
                refresh_token() // refresh in data.expiring_in.secs
            },data.expiring_in.secs * 1000)

            setLock(false)
            setDevicecode(data)
            exchange_loop()
            console.log(data.expiring_in.secs * 1000)

        }).catch(console.error)

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
            <div onClick={refresh_token}>{lock?refresh_disable:refresh}</div>
        </div>
        <CharBox chars={devicecode?.code} enable={!lock}/>
        <div className="flex flex-row px-2 gap-2">
            <span className="loading loading-spinner loading-xs"></span>
            <h1 className="">{message}</h1>
        </div>
        <div className="flex flex-row-reverse p-2">
            <button disabled={lock} className="btn outline outline-1 w-full bg-base-100">Open Link in browser</button>
        </div>

    </div>
}