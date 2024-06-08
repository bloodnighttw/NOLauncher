import {open} from "@tauri-apps/api/shell";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {CenterView} from "../component/Compose.tsx";
import {StepChild, StepParent} from "../component/Step.tsx";
import {Loading} from "../component/Animation.tsx";

interface Verify {
    verification_uri: string,
    user_code: string,
    expires_in: Date,
}

interface Status{
    status: "success" | "error",
    description: string | null
}

interface LoginButtonProps {
    details: Verify|null,
}

function LoginButton(props: LoginButtonProps) {

    const handleClick = async () => {
        await open(props.details?.verification_uri as string);
    };

    return (
        <div>
            <button className="h-8 text-sm font-semibold rounded-md shadow-md" onClick={handleClick}>Open Browser
            </button>
        </div>
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

interface AuthCodeProps {
    code: string
}

function AuthCode(props:AuthCodeProps){

    return (
        <div className="flex flex-row my-2">
            <p className="font-bold text-4xl"> {props.code} </p>
        </div>
    )
}

export function Auth() {
    const [verified, setVerified] = useState<Verify | null>(null)
    const [description, setDescription] = useState<Status | null>(null)
    const [all, setAll] = useState<boolean | null>(null)

    useEffect(() => {

        let works = async () => {
            let verifiedJson = JSON.parse(await invoke("devicecode_init")) as Verify
            setVerified(verifiedJson)
            let exchangeJson = JSON.parse(await invoke("devicecode_exchange", {code: verifiedJson.user_code})) as Status
            setDescription(exchangeJson)
            let xboxLiveJson = JSON.parse(await invoke("xbox_live_auth", {token: exchangeJson.description})) as Status
            setDescription(xboxLiveJson)
            let xboxXSTSJson = JSON.parse(await invoke("xbox_xsts_auth", {token: xboxLiveJson.description})) as Status
            setDescription(xboxXSTSJson)
            let minecraftTokenJson = JSON.parse(await invoke("minecraft_token", {token: xboxXSTSJson.description})) as Status
            setDescription(minecraftTokenJson)
            let minecraftProfileJson = JSON.parse(await invoke("minecraft_profile", {token: minecraftTokenJson.description})) as Status
            setDescription(minecraftProfileJson)
            setAll(minecraftProfileJson.status === "success")
        }

        works().then(r => console.log(r)).catch(_ => setAll(false))

    }, [setVerified,setDescription, setAll])

    // https://flowbite.com/docs/components/stepper/https://flowbite.com/docs/components/stepper/
    return (
        <CenterView>
            <div>
                <StepParent>
                    <StepChild condition={verified != null} svg={account}>
                        <h3 className="font-bold">Generating Device Auth Code</h3>
                        <p> {verified == null ? <Loading><p>please waiting......</p></Loading> : <AuthCode code={verified.user_code}/> }</p>
                    </StepChild>
                    <StepChild condition={description != null} svg={code} error={all === false}>
                        <h3 className="font-bold">Enter the code</h3>
                        <p>Open {verified?.verification_uri}</p>
                        <p>in browser and enter code {verified?.user_code}</p>
                        {verified == null || description != null ? "" : <LoginButton details={verified}/>}
                    </StepChild>
                    <StepChild condition={all === true} svg={xbox} error={all === false}>
                        <h3 className="font-bold">Fetching your data</h3>
                        {
                            all || description == null ? "" : <Loading>{description?.description}</Loading>
                        }

                    </StepChild>
                    <StepChild condition={all === true} svg={Done} error={all === false}>
                        <h3 className="font-bold">You are now log in!</h3>
                    </StepChild>
                </StepParent>
            </div>

        </CenterView>

    );
}