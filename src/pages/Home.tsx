import React, {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

interface InstanceIDProp{
    id:string
}

interface InstanceProp {
    img: string,
    text: string
    instanceId: string
}

interface Props {
    children?: React.ReactNode,
}

function Progress(prop:InstanceIDProp){

    const [progress,setProgress] = useState<ProgressChange>({now: 0, total: 100})

    useEffect(()=>{

        const event = "progress_update:"+prop.id;
        const unlistenPromise = listen<ProgressChange>(event, (res)=> setProgress(res.payload));

        return () =>{ // clean up
            unlistenPromise.then((unlisten) => unlisten()).catch(console.error)
        }

    },[setProgress])


    return <progress className="progress w-24" value={progress.now} max={progress.total}></progress>

}

function StatusCover(prop: InstanceIDProp) {

    const [status,setStatus] = useState<InstanceStatusChange>({now: undefined, total: undefined, type: "Stopped"})

    useEffect(() => {
        let eventName = "instance_status_update:" + prop.id
        let unlistenPromise = listen<InstanceStatusChange>(eventName, (res) => setStatus(res.payload))

        return () => { // clean up
            unlistenPromise.then((unlisten) => unlisten()).catch(console.error)
        }
    },[setStatus])

    const preparing = <div
        className="w-full h-full items-center rounded-md absolute bg-base-100 bg-opacity-0 opacity-100 bg-opacity-70 duration-200 flex flex-col justify-center items-center duration-200">
        <div className="flex flex-row">
            <span className="flex loading loading-spinner loading-xs p-2"></span>
        </div>
        <div className="">preparing</div>
    </div>

    const running = <div
        className="w-full h-full items-center rounded-md absolute bg-base-100 bg-opacity-0 opacity-100 bg-opacity-70 duration-200 flex flex-col justify-center items-center">
        <div className="flex flex-row">
            <span className="flex loading loading-spinner loading-xs p-2"></span>
        </div>
        <div className="">running</div>
    </div>

    const stopeed = <div
        className="w-full h-full items-center rounded-md absolute bg-base-100 bg-opacity-0 opacity-0 hover:opacity-100 hover:bg-opacity-50 duration-200 flex flex-col justify-center items-center">
        <button className="btn btn-ghost btn-sm"
                onClick={() => invoke("launch_game", {id: prop.id})}>Launch
        </button>
    </div>

    const failed = <div
        className="w-full h-full items-center rounded-md absolute bg-base-100 bg-opacity-0 opacity-0 hover:opacity-100 hover:bg-opacity-50 duration-200 flex flex-col justify-center items-center border-2 border-error">
        <button className="btn btn-ghost btn-sm"
                onClick={() => invoke("launch_game", {id: prop.id})}>Try Launch
        </button>
    </div>

    const downloading = <div
        className="w-full h-full items-center rounded-md absolute bg-base-100 bg-opacity-0 opacity-100 bg-opacity-70 duration-200 flex flex-col justify-center items-center">
        <div className="">download</div>
        <Progress id={prop.id}/>
    </div>

    const checking = <div
        className="w-full h-full items-center rounded-md absolute bg-base-100 bg-opacity-0 opacity-100 bg-opacity-70 duration-200 flex flex-col justify-center items-center">
        <div className="">checking</div>
        <Progress id={prop.id}/>
    </div>

    const match = (type:InstanceStatusChange)=>{
        switch (type.type){
            case "Checking": return checking
            case "Downloading": return downloading
            case "Failed":return failed
            case "Preparing":return preparing
            case "Running":return running
            case "Stopped":return stopeed
        }
    }

    return match(status)

}

function InstanceItem(props: InstanceProp) {

    const navigate = useNavigate();

    const doubleClickToInstance = () => {
        navigate("/instance/" + props.instanceId);
    }

    const style = {
        backgroundImage: `url("${props.img}")`,
        backgroundSize: "cover"
    }

    return <div className="flex flex-col duration-200" onDoubleClick={doubleClickToInstance}>
        <div className="w-40 h-20 rounded-md duration-200 relative">
            {/*<img src={props.img} className="w-40 h-20 rounded-md object-cover" title={props.text}/>*/}
            <div className="w-full h-full items-center rounded-md absolute" style={style}/>
            <StatusCover id={props.instanceId}/>
        </div>
        <div className="text-center text-md  w-40 overflow-hidden truncate">
        {props.text}
        </div>
    </div>
}

function InstanceList (props: Props) {
    return (
        <div className="flex flex-wrap gap-x-4 gap-y-2 p-4">
            {props.children}
        </div>
    )
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
        invoke<Array<InstanceInfo>>("list_instance").then((res)=>{
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