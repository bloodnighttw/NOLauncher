import React from "react";
import {useNavigate} from "react-router-dom";



interface InstanceProp {
    img: string,
    text: string
    instanceId: string
}

interface Props {
    children?: React.ReactNode,
}

function InstanceItem(props:InstanceProp) {

    const navigate = useNavigate();

    const doubleClickToInstance = () => {
        navigate("/instance/"+props.instanceId);
    }

    return <div className="flex flex-col p-2 select-all" onDoubleClick={doubleClickToInstance}>
        <div className="w-20 h-20 mx-2 rounded-md">
            <img src={props.img} className="w-20 h-20 rounded-md object-cover"></img>
        </div>
        <div className="text-center text-md object-full w-24 h-12 text-ellipsis overflow-hidden leading-4">
            {props.text}
        </div>
    </div>
}

function InstanceList (props: Props) {
    return (
        <div className="flex flex-wrap gap-4 p-4">
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
        <div className="flex flex-col p-2 select-all" onDoubleClick={doubleClickToCreate}>
            <div className="w-20 h-20 mx-2 rounded-md">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
                     stroke="currentColor" className="w-16 h-16 m-2 duration-200 text-gray-600 hover:text-gray-900">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15"/>
                </svg>
            </div>
            <div className="text-center text-md object-full w-24 h-12 text-ellipsis overflow-hidden leading-4">
                Add Instance
            </div>
        </div>
    );
}

export function Home() {

    return (
        <div>
        <InstanceList>
                <InstanceItem
                    img="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQLqPCLMpN2yRL9noYNEuddweIC-Spud6jIuA&s"
                    text="never gonna give u up, never gonna let u down"
                    instanceId="0xz4da"
                />
                <AddInstance/>
            </InstanceList>
        </div>
    );
}