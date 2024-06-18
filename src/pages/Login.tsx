import {useEffect, useState} from "react";
import {CenterView} from "../component/Compose.tsx";
import {Link, useNavigate, useParams} from "react-router-dom";
import {invoke} from "@tauri-apps/api/core";

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

const setting =
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
         className="size-6">
        <path strokeLinecap="round" strokeLinejoin="round"
              d="M6 13.5V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 9.75V10.5"/>
    </svg>

const logout =
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
         stroke-width="2" stroke-linecap="round" stroke-linejoin="round" className="size-6 text-error">
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
        <polyline points="16 17 21 12 16 7"/>
        <line x1="21" x2="9" y1="12" y2="12"/>
    </svg>

export function Login() {
    const [user, setUser] = useState<Array<Profile>>([])

    useEffect(() => {
        invoke("get_users").then((res) => {
            setUser(JSON.parse(res as string) as Array<Profile>)
        })
    }, [setUser])

    return (
        <CenterView>
            <div className="overflow-x-auto bg-base-100 rounded-md p-1.5 shadow-md flex-col">
                <table className="table">

                    {user.map((profile, _) => (

                        <tr className="bg-base-200 rounded-md">
                            <th><img className="w-6 h-6 rounded-sm" src={"https://crafatar.com/avatars/" + profile.id}/>
                            </th>
                            <td>{profile.name}</td>
                            <td className="text-right">{setting}</td>
                            <td className="text-right">{logout}</td>
                        </tr>


                    ))}


                </table>

                <div className="items-center text-center p-1.5">
                    <Link
                        key={3}
                        to="/auth"
                        className="text-center"
                    >
                        Add Account Here

                    </Link>
                </div>

            </div>
        </CenterView>
    );
}

export function UserProfile() {

    const {id} = useParams();
    let navigate = useNavigate();

    let click = () => {
        invoke("set_current_user", {id: id}).catch(console.error)
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