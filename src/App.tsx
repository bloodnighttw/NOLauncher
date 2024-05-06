import SideBar from "./SideBar";
import "./index.css";
import {Route, Routes} from "react-router-dom";
import {Home} from "./Home.tsx";
import {Server} from "./Server.tsx";
import {ModList} from "./ModList.tsx";

interface ContentProps {
    children?: React.ReactNode; // 👈️ for demo purposes
}

export function Content(props: ContentProps) {
    return (
        <div className="flex flex-col w-full">
            <div
                data-tauri-drag-region={true}
                className="h-8 bg-gray-100 w-full flex flex-row"
            >
            </div>
            {props.children}
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
                    </Routes>
                </Content>
            </div>

        </>
    );
}
