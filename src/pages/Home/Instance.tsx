import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";

interface InstanceProp {
    img: string,
    text: string
    instanceId: string
}


interface Props {
    children?: React.ReactNode,
}


export function InstanceList (props: Props) {
    return (
        <div className="flex flex-wrap gap-x-4 gap-y-2 p-4">
            {props.children}
        </div>
    )
}

export function InstanceItem(props:InstanceProp) {

    const navigate = useNavigate();

    const doubleClickToInstance = () => {
        navigate("/instance/"+props.instanceId);
    }

    const style = {
        backgroundImage:`url("${props.img}")`,
        backgroundSize: "cover"
    }

    return <div className="flex flex-col duration-200" onDoubleClick={doubleClickToInstance}>
        <div className="w-40 h-20 rounded-md duration-200 relative">
            {/*<img src={props.img} className="w-40 h-20 rounded-md object-cover" title={props.text}/>*/}
            <div className="w-full h-full items-center rounded-md absolute" style={style}/>
            <div className="w-full h-full items-center rounded-md absolute bg-base-100 bg-opacity-0 opacity-0 hover:opacity-100 hover:bg-opacity-50 duration-200 flex">
                <button className="m-auto btn btn-ghost btn-sm" onClick={()=>invoke("instance_downloads",{"id":props.instanceId})}>Launch</button> {/*fix later*/}
            </div>
        </div>
        <div className="text-center text-md  w-40 overflow-hidden truncate">
        {props.text}
        </div>
    </div>
}