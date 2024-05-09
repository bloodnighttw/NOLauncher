import React, {useEffect, useState} from "react";
import {Link} from "react-router-dom";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

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
    expires_in: number,
    interval: number,
    device_code: string,
}

interface LoginButtonProps {
    details: Verify|null,
}

function Center(props: PC) {
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

function Grid(props: PC) {
    let len = React.Children.count(props.children)
    let css = "gap-8 p-4 grid grid-cols-" + ((len <= 4) ? len : 4).toString();
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


function LoginButton(props: LoginButtonProps) {

    const handleClick = () => {
        invoke("msa_auth_open_browser", {invokeMessage: JSON.stringify(props.details)}).then((res: any) => {
            console.log(res)
        })
    };

    return (
        <div>
            <button className="h-8 text-sm font-semibold rounded-md shadow-md" onClick={handleClick}>Open In Browser
            </button>
        </div>
    );
}

function StepParent(prop: PC) {
    return (
        <ol className="relative w-10/12 text-gray-500 border-s border-gray-200 dark:border-gray-700 dark:text-gray-400">
            {prop.children}
        </ol>
    );
}

interface StepProps {
    condition: boolean,
    svg: React.ReactNode,
    children: React.ReactNode,
}

function StepChild(prop: StepProps) {
    const done = "absolute flex items-center justify-center w-8 h-8 bg-green-200 rounded-full -start-4 ring-4 ring-white dark:ring-zinc-700 dark:bg-green-900";
    const notDone = "absolute flex items-center justify-center w-8 h-8 bg-gray-100 rounded-full -start-4 ring-4 ring-white dark:ring-zinc-700 dark:bg-zinc-900";
    return (
        <li className="mb-10 ms-6 px-2">
            <span
                className={prop.condition ? done : notDone}>
                {prop.svg}
            </span>
            <div className="px-2 text-left">
                {prop.children}
            </div>
        </li>
    );

}

const code = (
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
         className="w-3.5 h-3.5 text-gray-500 dark:text-gray-400"
         viewBox="0 0 16 16">
        <path
            d="M2.873 11.297V4.142H1.699L0 5.379v1.137l1.64-1.18h.06v5.961zm3.213-5.09v-.063c0-.618.44-1.169 1.196-1.169.676 0 1.174.44 1.174 1.106 0 .624-.42 1.101-.807 1.526L4.99 10.553v.744h4.78v-.99H6.643v-.069L8.41 8.252c.65-.724 1.237-1.332 1.237-2.27C9.646 4.849 8.723 4 7.308 4c-1.573 0-2.36 1.064-2.36 2.15v.057zm6.559 1.883h.786c.823 0 1.374.481 1.379 1.179.01.707-.55 1.216-1.421 1.21-.77-.005-1.326-.419-1.379-.953h-1.095c.042 1.053.938 1.918 2.464 1.918 1.478 0 2.642-.839 2.62-2.144-.02-1.143-.922-1.651-1.551-1.714v-.063c.535-.09 1.347-.66 1.326-1.678-.026-1.053-.933-1.855-2.359-1.845-1.5.005-2.317.88-2.348 1.898h1.116c.032-.498.498-.944 1.206-.944.703 0 1.206.435 1.206 1.07.005.64-.504 1.106-1.2 1.106h-.75z"/>
    </svg>
)

const account = (
    <svg className="w-3.5 h-3.5 text-gray-500 dark:text-gray-400" aria-hidden="true"
         xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 16">
        <path
            d="M18 0H2a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2ZM6.5 3a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5ZM3.014 13.021l.157-.625A3.427 3.427 0 0 1 6.5 9.571a3.426 3.426 0 0 1 3.322 2.805l.159.622-6.967.023ZM16 12h-3a1 1 0 0 1 0-2h3a1 1 0 0 1 0 2Zm0-3h-3a1 1 0 1 1 0-2h3a1 1 0 1 1 0 2Zm0-3h-3a1 1 0 1 1 0-2h3a1 1 0 1 1 0 2Z"/>
    </svg>
)

const xbox = (
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="bi bi-dpad-fill"
         viewBox="0 0 16 16">
        <path
            d="M6.5 0A1.5 1.5 0 0 0 5 1.5v3a.5.5 0 0 1-.5.5h-3A1.5 1.5 0 0 0 0 6.5v3A1.5 1.5 0 0 0 1.5 11h3a.5.5 0 0 1 .5.5v3A1.5 1.5 0 0 0 6.5 16h3a1.5 1.5 0 0 0 1.5-1.5v-3a.5.5 0 0 1 .5-.5h3A1.5 1.5 0 0 0 16 9.5v-3A1.5 1.5 0 0 0 14.5 5h-3a.5.5 0 0 1-.5-.5v-3A1.5 1.5 0 0 0 9.5 0zm1.288 2.34a.25.25 0 0 1 .424 0l.799 1.278A.25.25 0 0 1 8.799 4H7.201a.25.25 0 0 1-.212-.382zm0 11.32-.799-1.277A.25.25 0 0 1 7.201 12H8.8a.25.25 0 0 1 .212.383l-.799 1.278a.25.25 0 0 1-.424 0Zm-4.17-4.65-1.279-.798a.25.25 0 0 1 0-.424l1.279-.799A.25.25 0 0 1 4 7.201V8.8a.25.25 0 0 1-.382.212Zm10.043-.798-1.278.799A.25.25 0 0 1 12 8.799V7.2a.25.25 0 0 1 .383-.212l1.278.799a.25.25 0 0 1 0 .424Z"/>
    </svg>
)


const Done = (
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
         className="bi bi-check-circle-fill" viewBox="0 0 16 16">
        <path
            d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0m-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z"/>
    </svg>
)

interface Payload{
    message: string
}

function Loading(props: PC) {
    return (
        <div className="flex flex-row">
            <div className="pr-2">
                <svg aria-hidden="true"
                     className="inline w-4 h-4 text-gray-200 animate-spin dark:text-gray-600 fill-gray-600 dark:fill-gray-300"
                     viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                        fill="currentColor"/>
                    <path
                        d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                        fill="currentFill"/>
                </svg>
                <span className="sr-only">Loading...</span>
            </div>

            {props.children}
        </div>
    );
}


export function Auth() {
    const [verfied, setVerified] = useState<Verify | null>(null)
    const [allDone, setAllDone] = useState(false)
    const [state, setState] = useState("")

    useEffect(() => {
        invoke("msa_auth_init").then((res: any) => {
            console.log(res)
            let json = JSON.parse(res)
            setVerified(json as Verify)
            invoke("msa_auth_exchange", {invokeMessage: res}).then((res: any) => {
                console.log(res)
                setAllDone(true)
            })
        })
    }, [setVerified, setAllDone]);


    useEffect(() => {
        console.log('hi')
        const event = async () => {
            await listen<Payload>("mc_login", (event: any) => {
                let payload = event.payload;
                console.log(payload.message)
                setState(payload.message)
            })
        }
        event()
    })

    // https://flowbite.com/docs/components/stepper/https://flowbite.com/docs/components/stepper/
    return (
        <Center>
            <div>
                <StepParent>
                    <StepChild condition={verfied != null} svg={account}>
                        <h3 className="font-bold">Generating Device Auth Code</h3>
                        <p> {verfied == null ? <Loading><p>please waiting......</p></Loading> : "the code is " + verfied.user_code} </p>
                    </StepChild>
                    <StepChild condition={(state !== "" || allDone)} svg={code}>
                        <h3 className="font-bold">Enter the code</h3>
                        <p>Click the button down below to auth with Microsoft!</p>
                        <p>Open {verfied?.verification_uri}</p>
                        <p>in browser and enter code {verfied?.user_code}</p>
                        {verfied == null || allDone ? "" : <LoginButton details={verfied}/>}
                    </StepChild>
                    <StepChild condition={allDone} svg={xbox}>
                        <h3 className="font-bold">Fetching your data</h3>
                        {
                            allDone ? "" : state == "" ? "" : <Loading>{state}</Loading>
                        }
                    </StepChild>
                    <StepChild condition={allDone} svg={Done}>
                        <h3 className="font-bold">You are now log in!</h3>
                    </StepChild>
                </StepParent>
            </div>

        </Center>

    );
}


export function Login() {
    const [user, _] = useState<[]>([])

    return (
        <Center>
            {(user.length > 0 ?
                    have_account :
                    no_account
            )}
            <Grid>
                <LoginCard/>
            </Grid>
        </Center>

    );
}