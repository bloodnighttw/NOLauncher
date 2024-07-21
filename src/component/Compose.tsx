import React from "react";

interface Props {
    children?: React.ReactNode,
}

export function CenterView(props: Props) {
    return (
        <div className="px-2 py-1 flex flex-col h-full justify-center items-center overflow-y-auto">
            {props.children}
        </div>
    );
}

