import {useEffect, useState} from "react";
import {CenterView, DynamicGrid} from "../component/Compose.tsx";
import {Link, useNavigate, useParams} from "react-router-dom";
import {invoke} from "@tauri-apps/api/core";

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

interface LoginCardProps {
    image?: string;
    url?: string;
    key?: string;
}

export function LoginCard(props: LoginCardProps) {
    return (
        <Link
            to={props.url || "/auth"}
            className="w-20 h-20 bg-gray-200 dark:bg-zinc-700 rounded-xl"
        >
            {props.image == null ?
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                     className="w-full h-full text" viewBox="0 0 16 16">
                    <path
                        d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
                </svg>

                :
                <img src={props.image} alt="user" className="w-full h-full rounded-xl"/>
            }
        </Link>
    );
}

export function Login() {
    const [user, setUser] = useState<Array<Profile>>([])

    useEffect(() => {
        invoke("get_users").then((res) => {
            setUser(JSON.parse(res as string) as Array<Profile>)
        })
    },[setUser])

    return (
        <CenterView>
            {(user.length > 0 ?
                    have_account :
                    no_account
            )}
            <DynamicGrid len={user.length+1}>
                {user.map((value, _) => (
                    <LoginCard key={value.id} image={"https://crafatar.com/avatars/"+value.id} url={"/login/"+value.id}/>
                ))}
                <LoginCard/>
            </DynamicGrid>
        </CenterView>
    );
}

export function UserProfile() {

    const { id } = useParams();
    let navigate = useNavigate();

    let click = () => {
        invoke("set_current_user",{id: id}).catch(console.error)
        navigate("/login")
    }

    return (
        <CenterView>
            <h1>{id}</h1>
            <div>
                <button className="btn btn-sm shadow-none" onClick={click}>
                    switch to this account
                </button>
            </div>
        </CenterView>
    );
}