import SideBar from "./component/SideBar.tsx";
import "./index.css";
import {Route, Routes} from "react-router-dom";
import {Home} from "./pages/Home.tsx";
import {Server} from "./pages/Server.tsx";
import {ModList} from "./pages/ModList.tsx";
import {Settings} from "./pages/Settings.tsx";
import {Login, UserProfile} from "./pages/Login.tsx";
import {Auth} from "./pages/Auth.tsx";
import React from "react";
import {Create} from "./pages/Create.tsx";

interface ContentProps {
    children?: React.ReactNode; // 👈️ for demo purposes
}


export function Content(props: ContentProps) {
    return (
        <div className="flex flex-col h-screen w-full">
            <div
                data-tauri-drag-region={true}
                className="h-8 bg-base-200 w-full flex flex-row sticky"
            >
            </div>
            <div className="w-full h-full overflow-y-auto bg-base-200">
                {props.children}
            </div>
        </div>
    );
}

export default function App() {
    return (
        <div className="flex flex-row">
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
        </div>

    );
}
