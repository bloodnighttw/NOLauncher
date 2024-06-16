import React from "react";

interface Props {
    children?: React.ReactNode,
}

interface DynamicGridProps {
    children?: React.ReactNode,
    len: number,
}

interface InstanceProp {
    img: string,
    text: string
}

export function CenterView(props: Props) {
    return (
        <div className="px-2 py-1 flex flex-col h-full justify-center items-center overflow-y-auto ">
            {props.children}
        </div>
    );
}

export function DynamicGrid(props: DynamicGridProps) {
    if(props.len == 1){
        return <div className="gap-8 p-4 grid grid-cols-1">{props.children}</div>
    } else if(props.len == 2){
        return <div className="gap-8 p-4 grid grid-cols-2">{props.children}</div>
    } else if(props.len == 3){
        return <div className="gap-8 p-4 grid grid-cols-3">{props.children}</div>
    }
    return <div className={"gap-8 p-4 grid grid-cols-4"+props.len}>{props.children}</div>
}

export function InstanceItem(props:InstanceProp) {
    return <div className="flex flex-col p-4">
        <div className="w-24 h-24 rounded-md">
            <img src={props.img} className="w-20 h-20 mx-2 rounded-md object-cover"></img>
        </div>
        <div className="text-center text-md object-full w-24 h-12 text-ellipsis overflow-hidden">
            {props.text}
        </div>
    </div>

}

export function InstanceList(props: Props) {
    return (
        <div className="flex flex-wrap gap-4 p-4">
            {props.children}
        </div>
    )
}