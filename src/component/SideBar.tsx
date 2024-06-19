import "../index.css";
import {Link, useLocation} from "react-router-dom";
import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";

const homeSVG = (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        strokeWidth="1.5"
        stroke="currentColor"
        className="w-6 h-6"
    >
        <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
        />
    </svg>
);

const serverSVG = (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        strokeWidth={1.5}
        stroke="currentColor"
        className="w-6 h-6"
    >
        <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M21.75 17.25v-.228a4.5 4.5 0 0 0-.12-1.03l-2.268-9.64a3.375 3.375 0 0 0-3.285-2.602H7.923a3.375 3.375 0 0 0-3.285 2.602l-2.268 9.64a4.5 4.5 0 0 0-.12 1.03v.228m19.5 0a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3m19.5 0a3 3 0 0 0-3-3H5.25a3 3 0 0 0-3 3m16.5 0h.008v.008h-.008v-.008Zm-3 0h.008v.008h-.008v-.008Z"
        />
    </svg>
);

const modListSVG = (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        strokeWidth={1.5}
        stroke="currentColor"
        className="w-6 h-6"
    >
        <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 0 1 0 3.75H5.625a1.875 1.875 0 0 1 0-3.75Z"
        />
    </svg>
);

interface UUIDProps {
    id: string
}

export function UserImage(props: UUIDProps) {
    return (
        <img
            className="object-cover w-9 h-9 p-0 rounded-md"
            src={"https://crafatar.com/avatars/" + props.id}
            alt=""
        />
    )

}

const settingSVG = (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        strokeWidth="1.5"
        stroke="currentColor"
        className="w-6 h-6"
    >
        <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z"
        />
        <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
        />
    </svg>
)

const noaccount = (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        strokeWidth={1.5}
        stroke="currentColor"
        className="w-6 h-6">
        <path strokeLinecap="round" strokeLinejoin="round"
              d="M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z"/>
    </svg>
)

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

const newAccont =
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
         className="size-6">
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"/>
    </svg>


const btnList = [
    {icon: homeSVG, link: "/"},
    {icon: serverSVG, link: "/server"},
    {icon: modListSVG, link: "/modlist"},
]

function identifyLink(args: any) {
    if (args.pathname.startsWith('/login') || args.pathname.startsWith('/auth')) return 4;
    if (args.pathname.startsWith('/settings')) return 3;
    if (args.pathname.startsWith('/modlist')) return 2;
    if (args.pathname.startsWith('/server')) return 1;
    return 0; // '/home' & '/instance/{id}'
}

export default function SideBar() {
    let location = useLocation();
    let [user, setUser] = useState<UUIDPayload | null>(null);
    let [menu, setMenu] = useState<boolean>(false);
    const [users, setUsers] = useState<Array<Profile>>([])


    let work = async () => {
        await listen<UUIDPayload>("change_user", (event) => {
            setUser(event.payload);
        });
    }

    useEffect(() => {
        work().catch(console.error);
        invoke("get_current_user").then((res) => {
            setUser({
                uuid: res as string
            });
            console.log(res)
        })
    }, [setUser])

    const selected = "p-1.5 bg-base-300 rounded-md transition-transform duration-200";
    const notSelect = "p-1.5 hover:bg-base-300 duration-200 rounded-md active:scale-90";

    const show = "dropdown-open dropdown dropdown-right dropdown-end absolute "
    const notShow = "dropdown dropdown-right dropdown-end absolute invisible"

    const userNotSelect = "bg-base-200 rounded-md flex p-3 cursor-pointer gap-4"
    const userSelect = "hover:bg-base-200 rounded-md flex p-3 duration-200 cursor-pointer gap-4"


    return (

        <aside data-tauri-drag-region={true}
               className="flex flex-col items-center w-20 h-screen py-4 overflow-y-auto bg-base-100 overflow-hidden gap-4">
            <div className="flex flex-col flex-1 gap-4" data-tauri-drag-region={true}>
                {btnList.map((btn, index) => (


                    <Link
                        key={index}
                        to={btn.link}
                        className={index == identifyLink(location) ? selected : notSelect}
                    >
                        {btn.icon}
                    </Link>

                ))}
            </div>

            <div className="flex flex-col">


                <Link
                    key={3}
                    to="/settings"
                    className={3 == identifyLink(location) ? selected : notSelect}
                >
                    {settingSVG}
                </Link>

            </div>

            <div className="flex flex-col">
                <div
                    className="p-1.5 duration-200 rounded-md"
                    onClick={() => {
                        invoke("get_users").then((res) => {
                            setUsers(JSON.parse(res as string) as Array<Profile>)
                        }).catch(console.error)
                        setMenu(true)
                    }}
                    onMouseLeave={() => setMenu(false)}
                >

                    {
                        user == null ? noaccount
                            : <UserImage id={user?.uuid}/>
                    }


                    <div className={menu ? show : notShow}>
                        <div tabindex="0"
                             class="dropdown-content z-[1000] shadow-lg bg-base-100 rounded-md w-96">
                            <div className="max-h-80 overflow-y-auto p-3">
                                <div
                                    className="bg-base-100 flex-col space-y-1.5">

                                    {users.map((profile, _) => (

                                        <div className={user?.uuid == profile.id ? userNotSelect : userSelect}>
                                            <div className="flex-1"
                                                 onClick={() => invoke("set_current_user", {id: profile.id}).catch(console.error)}
                                            ><img className="w-6 h-6 rounded-sm" src={"https://crafatar.com/avatars/" + profile.id}/>
                                            </div>
                                            <div className="grow"
                                                 onClick={() => invoke("set_current_user", {id: profile.id}).catch(console.error)}>{profile.name}</div>

                                            <div
                                                className="text-right flex-none active:scale-90 duration-200">
                                                <Link
                                                    to={"/login/" + profile.id}
                                                >
                                                    {setting}

                                                </Link>

                                            </div>
                                            <div
                                                className="text-right flex-none active:scale-90 duration-200"
                                                onClick={() => invoke("logout_user", {id: profile.id}).catch(console.error)}
                                            >{logout}</div>
                                        </div>


                                    ))}


                                    <Link className={userSelect + " justify-center active:scale-90"}
                                          to="/auth"
                                    >
                                        <div
                                            className="text-left flex-none active:scale-90 duration-200">{newAccont}</div>
                                    </Link>


                                </div>


                            </div>
                        </div>
                    </div>

                </div>
            </div>
        </aside>
    );
}
