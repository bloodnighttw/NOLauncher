import React, {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

export default function useCommand<T>(commandName:string):[T|null,React.Dispatch<React.SetStateAction<T|null>>]{

    const [status, setStatus] = useState<T|null>(null);

    useEffect(() => {
        invoke<T>(commandName).then((e)=>{
            setStatus(e)
        }).catch(console.error)
    }, [setStatus])

    return [status,setStatus];

}