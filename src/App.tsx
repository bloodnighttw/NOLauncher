import SideBar from "./SideBar";
import "./index.css";
import {Route, Routes} from "react-router-dom";
import {Home} from "./Home.tsx";
import {Server} from "./Server.tsx";
import {ModList} from "./ModList.tsx";
import React from "react";
import {Settings} from "./Settings.tsx";
import {Auth, Login} from "./Login.tsx";

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
                        <Route path="/login/auth" element={<Auth/>}/>
                    </Routes>
                </Content>
            </div>

        </>
    );
}
