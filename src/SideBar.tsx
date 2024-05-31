import "./index.css";
import {Link, useLocation} from "react-router-dom";
import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";

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

const userImage = (
    <img
        className="object-cover w-9 h-9 rounded-md"
        src="https://avatars.githubusercontent.com/u/44264182?s=460&u=b59e580f37ab7e6a3979ab8a6df1f12ba6588069&v=4"
        alt=""
    />
)

const consoleSVG = (
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
            d="m6.75 7.5 3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0 0 21 18V6a2.25 2.25 0 0 0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 6v12a2.25 2.25 0 0 0 2.25 2.25Z"
        />
    </svg>
)

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
    let [_user, setUser] = useState<UUIDPayload | null>(null);

    let work = async () => {
        await listen<UUIDPayload>("change_user", (event) => {
            setUser(event.payload);
        });

    }

    useEffect(() => {
        work();
    })


    return (
        <>
            <aside
                className="flex flex-col items-center w-20 h-screen py-8 overflow-y-auto bg-white border-r dark:bg-zinc-900 dark:border-zinc-700 sticky">
                <nav className="flex flex-col flex-1 space-y-6">
                    {btnList.map((btn, index) => (
                        (identifyLink(location) === index
                            ? <Link
                                key={index}
                                to={btn.link}
                                className={"p-1.5 text-gray-700 focus:outline-nones  duration-200 rounded-lg dark:text-gray-200 dark:bg-gray-800 bg-gray-100"}
                            >
                                {btn.icon}
                            </Link>
                            : <Link
                                key={index}
                                to={btn.link}
                                className={"p-1.5 text-gray-700 focus:outline-nones  duration-200 rounded-lg dark:text-gray-200 dark:hover:bg-gray-800 hover:bg-gray-100"}
                            >
                                {btn.icon}
                            </Link>
                        )

                    ))}

                    <hr/>

                    <Link to="#">
                        {userImage}
                    </Link>

                    <Link to="#">
                        {userImage}
                    </Link>
                </nav>

                <div className="flex flex-col space-y-6 sticky">
                    <Link
                        to="#"
                        className="p-1.5 text-gray-700 focus:outline-nones transition-colors duration-200 rounded-lg dark:text-gray-200 dark:hover:bg-gray-800 hover:bg-gray-100"
                    >
                        {consoleSVG}
                    </Link>

                    {
                        (identifyLink(location) === 3
                                ? <Link
                                    key={3}
                                    to="/settings"
                                    className={"p-1.5 text-gray-700 focus:outline-nones  duration-200 rounded-lg dark:text-gray-200 dark:bg-gray-800 bg-gray-100"}
                                >
                                    {settingSVG}
                                </Link>
                                : <Link
                                    key={3}
                                    to="/settings"
                                    className={"p-1.5 text-gray-700 focus:outline-nones  duration-200 rounded-lg dark:text-gray-200 dark:hover:bg-gray-800 hover:bg-gray-100"}
                                >
                                    {settingSVG}
                                </Link>
                        )

                    }

                    <Link to="/login">
                        {userImage}
                    </Link>
                </div>
            </aside>
        </>
    );
}
