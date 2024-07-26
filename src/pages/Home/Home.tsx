import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { InstanceItem,InstanceList } from "./Instance";

interface InstanceInfo {
    id: string,
    name:string,
    basePath: string,
}


function AddInstance() {
    const navigate = useNavigate();

    const doubleClickToCreate = () => {
        navigate("/create");
    }

    return (
        <div className="flex flex-col active:scale-95 duration-200" onDoubleClick={doubleClickToCreate}>
            <div className="w-40 h-20 rounded-md">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
                     stroke="currentColor" className="w-40 h-16 duration-200 text-gray-900">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15"/>
                </svg>
            </div>
            <div className="text-center text-md object-full w-40 h-12 text-ellipsis overflow-hidden leading-4">
                New Instance
            </div>
        </div>
    );
}

export function Home() {

    const [instances,setInstances] = useState<Array<InstanceInfo>>([]);

    useEffect( ()=>{
        invoke<Array<InstanceInfo>>("instance_list").then((res)=>{
            setInstances(res)
        }).catch(console.error)

    },[setInstances])

    return (
        <div>
            <InstanceList>

                {instances.map((i) =>(
                    <InstanceItem
                        img="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQLqPCLMpN2yRL9noYNEuddweIC-Spud6jIuA&s"
                        text={i.name}
                        instanceId={i.id}
                    />
                ))}
                <AddInstance/>
            </InstanceList>
        </div>
    );
}