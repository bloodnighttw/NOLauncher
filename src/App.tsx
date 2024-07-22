import SideBar from "./component/Sidebar/SideBar.tsx";
import "./index.css";
import {Route, Routes} from "react-router-dom";
import {Home} from "./pages/Home.tsx";
import {Server} from "./pages/Server.tsx";
import {ModList} from "./pages/ModList.tsx";
import {Settings} from "./pages/Settings.tsx";
import {Login, UserProfile} from "./pages/Login.tsx";
import {Auth} from "./pages/Auth/Auth.tsx";
import React from "react";
import {Create} from "./pages/Create.tsx";
import AccountPanel from "./component/Sidebar/AccountPanel.tsx";

interface ContentProps {
    children?: React.ReactNode; // 👈️ for demo purposes
}


export function Content(props: ContentProps) {
    return (
        <div className="w-full h-screen overflow-y-auto bg-base-200">
            {props.children}
        </div>
    );
}

export default function App() {
    return (
        <div className="flex flex-row relative">
            <SideBar/>
            <Content>
                <Routes>
                    <Route path="/" element={<Home/>}/>
                    <Route path="/create" element={<Create/>}/>
                    <Route path="/server" element={<Server/>}/>
                    <Route path="/modlist" element={<ModList/>}/>
                    <Route path="/settings" element={<Settings/>}/>
                    <Route path="/login">
                        <Route path="" element={<Login/>}/>
                        <Route path=":id" element={<UserProfile/>}/>
                    </Route>
                    <Route path="/auth" element={<Auth/>}/>
                </Routes>
            </Content>
            <AccountPanel/>
        </div>

    );
}
