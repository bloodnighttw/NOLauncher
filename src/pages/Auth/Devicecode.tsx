import useCommand from "../../hook/useCommand.ts";
import {CharBox} from "../../component/CharBox.tsx";
import {useState} from "react";

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

export function Devicecode(){

    const [devicecode,_] = useCommand<DevicecodePayload>("devicecode")
    const [message, setMessage] = useState("roger say that u are 2486!")

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
        <CharBox chars={devicecode?.code} enable={false}/>
        <div className="flex flex-row px-2 gap-2">
            <span className="loading loading-spinner loading-xs"></span>
            <h1 className="">{message}</h1>
        </div>
        <div className="flex flex-row-reverse p-2">
        <button disabled className="btn outline outline-1 w-full bg-base-100">Open Link in browser</button>
        </div>


    </div>
}