import React, {useState} from "react";

interface LoginCardProps {
    image?: string;
}

interface LoginCardGridProps {
    children?: React.ReactNode,
}

function LoginCard(props:LoginCardProps) {
    return (
        <div className="w-24 h-24 bg-gray-200 dark:bg-zinc-700 rounded-xl">
            {props.image == null ?
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                     className="w-full h-full" viewBox="0 0 16 16">
                    <path
                        d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
                </svg>

                :
                <img src={props.image} alt="user" className="w-full h-full"/>
            }
        </div>
    );
}

function LoginCardGrid(props:LoginCardGridProps) {
    let len = (props.children as React.ReactNode[]).length
    let css = "gap-8 p-4 grid grid-cols-" + ((len >=4) ? 4 : len);
    return (
        <div className={css}>
            {props.children}
        </div>
    );
}


export function Login() {
    const [user, setUser] = useState<[]>([])

    return (
        <div className="px-2 py-1 flex flex-col h-full justify-center items-center overflow-y-auto ">
            <h1 className="text-4xl">You doesn't have any account</h1>
            <p className="text-xl">add one down below</p>
            <LoginCardGrid>
                <LoginCard/>
            </LoginCardGrid>
        </div>
    );
}