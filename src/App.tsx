import SideBar from "./SideBar";
import "./index.css";

interface ContentProps {
  children?: React.ReactNode; // 👈️ for demo purposes
}

export function Content(props: ContentProps) {
  return (
    <div className="flex flex-col w-full">
      <div
        data-tauri-drag-region
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
        <SideBar />
        <Content>
          <h1 className="text-2xl font-bold px-4">Coming soon ......</h1>
          <p className="text-gray-500 px-4 text-center">
            The application is currently under construct,you can follow use on 
            <a href="https://github.com/bloodnighttw/nolauncher">github!</a>
          </p>
        </Content>
      </div>
    </>
  );
}
