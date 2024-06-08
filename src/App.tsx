import SideBar from "./component/SideBar.tsx";
import "./index.css";
import {Route, Routes} from "react-router-dom";
import {Home} from "./pages/Home.tsx";
import {Server} from "./pages/Server.tsx";
import {ModList} from "./pages/ModList.tsx";
import {Settings} from "./pages/Settings.tsx";
import {Login} from "./pages/Login.tsx";
import {Auth} from "./pages/Auth.tsx";
import React from "react";

interface ContentProps {
    children?: React.ReactNode; // 👈️ for demo purposes
}

export function Content(props: ContentProps) {
    return (
        <div className="flex flex-col h-screen w-full">
            <div
                data-tauri-drag-region={true}
                className="h-8 bg-gray-100 w-full flex flex-row sticky"
            >
            </div>
            <div className="w-full h-full overflow-y-auto">
                {props.children}
            </div>
        </div>
    );
}

export default function App() {
    return (
        <>
            <div className="flex flex-row">
                <SideBar/>
                <Content>
                    <Routes>
                        <Route path="/" element={<Home/>}/>
                        <Route path="/server" element={<Server/>}/>
                        <Route path="/modlist" element={<ModList/>}/>
                        <Route path="/settings" element={<Settings/>}/>
                        <Route path="/login" element={<Login/>}/>
                        <Route path="/auth" element={<Auth/>}/>
                    </Routes>
                </Content>
            </div>

        </>
    );
}
