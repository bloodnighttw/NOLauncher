import React, {useEffect, useState} from "react";
import {Link} from "react-router-dom";
import {invoke} from "@tauri-apps/api/tauri";

interface LoginCardProps {
    image?: string;
    url?: string;
}

interface PC {
    children?: React.ReactNode,
}

interface Verify {
    verification_uri: string,
    user_code: string,
}

function Center(props:PC) {
    return (
        <div className="px-2 py-1 flex flex-col h-full justify-center items-center overflow-y-auto ">
            {props.children}
        </div>
    );

}

function LoginCard(props: LoginCardProps) {
    return (
        <Link
            to={props.url || "/auth"}
            className="w-20 h-20 bg-gray-200 dark:bg-zinc-700 rounded-xl"
        >
            {props.image == null ?
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                     className="w-full h-full" viewBox="0 0 16 16">
                    <path
                        d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
                </svg>

                :
                <img src={props.image} alt="user" className="w-full h-full"/>
            }
        </Link>
    );
}

function Grid(props:PC) {
    let len = React.Children.count(props.children)
    let css = "gap-8 p-4 grid grid-cols-" +((len <=4) ? len : 4).toString();
    return (
        <div className={css}>
            {props.children}
        </div>
    );
}

const have_account = (<h1 className="text-4xl">Select icon to switch</h1>);
const no_account = (
    <div className="flex flex-col items-center justify-center">
        <h1 className="text-4xl">You don't have any account.</h1>
        <div className="flex flex-row">
            <p>press</p>
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                 className="" viewBox="0 0 16 16">
                <path
                    d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
            </svg>
            <p>
                button to login your first account.
            </p>
        </div>

    </div>
)

function Generating() {
    return (
        <div>
            <h1>Generating</h1>
        </div>
    );
}


interface InnerInformation{
    message:{
        verification_uri: string,
        user_code: string
    }
}

function LoginButton(props:InnerInformation) {

    const handleClick = () => {
        invoke("msa_auth_open_browser",{ invokeMessage: JSON.stringify(props.message) }).then((res:any) => {
            console.log(res)
    })};

    return (
        <div>
            <button className="font-semibold rounded-md shadow-md" onClick={handleClick}>OUO</button>
        </div>
    );
}

export function Auth() {
    const [verfied, setVerified] = useState<Verify | null>(null)

    useEffect(() => {
        invoke("msa_auth_init").then((res:any) => {
            console.log(res)
            let json = JSON.parse(res)
            setVerified(json as Verify)
        })
    }, [setVerified]);

    return (
        <Center>
            <div>
                {verfied == null ? <Generating/> : null}
                {verfied == null ?  null : <LoginButton message={verfied}/>}

                <h1>
                    {verfied == null ? verfied : verfied.verification_uri}
                </h1>
            </div>

        </Center>

    );
}


export function Login() {
    const [user, _] = useState<[]>([])

    return (
        <Center>
            {(user.length > 0 ?
                    have_account:
                    no_account
            )}
            <Grid>
                <LoginCard/>
            </Grid>
        </Center>

    );
}