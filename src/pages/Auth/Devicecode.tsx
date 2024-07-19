import {CharBox} from "../../component/CharBox.tsx";
import {useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

// #[derive(Debug,Clone,Serialize)]
// pub struct DevicecodeInfo{
//     url:String,
//     code:String,
//     expiring_in:Duration
// }

interface DevicecodePayload{
    url:string,
    code:string,
    expiring_in:string
}

interface ExchangePayload{
    action:"Pending" | "Success",
    second: number
}

export function Devicecode(){

    const [devicecode,setDevicecode] = useState<DevicecodePayload|null>(null)
    const [message, setMessage] = useState("Fetching device code...")

    const exchange = useRef<number | null>(null);

    const cleanup_exchange = ()=>{
        if(exchange.current){
            clearTimeout(exchange.current)
        }
    }

    useEffect(()=>{

        invoke<DevicecodePayload>("devicecode").then((data)=>{
            setDevicecode(data)
            exchange_loop()
        }).catch(console.error)

        const exchange_loop = ()=>{
            setMessage("Waiting user to complete auth flow......")
            invoke<ExchangePayload>("exchange").then((data)=>{
                if(data.action === "Success"){

                }else
                    exchange.current = setTimeout(()=>{
                        exchange_loop()
                    },data.second * 1000)
            }).catch(console.error)
        }

        return ()=>{ // cleanup timeout function
            cleanup_exchange()
        }

    },[setMessage])

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
        <h1 className="w-full px-2 text-xl">Your login code:</h1>
        <CharBox chars={devicecode?.code} enable={true}/>
        <div className="flex flex-row px-2 gap-2">
            <span className="loading loading-spinner loading-xs"></span>
            <h1 className="">{message}</h1>
        </div>
        <div className="flex flex-row-reverse p-2">
        <button className="btn outline outline-1 w-full bg-base-100">Open Link in browser</button>
        </div>


    </div>
}