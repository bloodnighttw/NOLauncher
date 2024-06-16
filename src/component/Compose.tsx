import React from "react";

interface Props {
    children?: React.ReactNode,
}

interface DynamicGridProps {
    children?: React.ReactNode,
    len: number,
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

