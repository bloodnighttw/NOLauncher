import { useState } from "react";
import { NewInstance } from "./NewInstance";

export function Create() {

    const [tabIndex, setTabIndex] = useState(0)
    const [name,setName] = useState<string>("")


    const tab = "tab duration-200";
    const tabActive = "tab tab-active duration-200";
    const tabs = [
        {long_name: "Create", component: <NewInstance name={name}/>},
        {long_name: "Import", component: null},
    ];


    return (
        <div className="p-2">
            <div className="w-full flex flex-row gap-2 p-2">
                <div className="w-24 h-20">
                    <img
                        className="w-20 h-20 rounded-md object-cover"
                        src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQLqPCLMpN2yRL9noYNEuddweIC-Spud6jIuA&s"/>
                </div>
                <div className="flex flex-col gap-2 w-full h-20">
                    <input type="text" placeholder="Type Instance Name Here "
                           className="input input-xs input-bordered w-full h-full"
                           onChange={(e) => {
                               setName(e.target.value);
                           }}
                    />

                    <select disabled className="select select-bordered w-full select-sm h-full ">
                        <option disabled selected>Tag is Coming soon......</option>
                    </select>
                </div>


            </div>
            <div className="h-full space-y-2 p-2">
                <div role="tablist" className="tabs tabs-bordered">
                    {tabs.map((i, index) => (
                        <div className={index == tabIndex ? tabActive : tab}
                             onClick={() => setTabIndex(index)}>{i.long_name}</div>
                    ))}
                </div>

                {tabs.map((i, index) => (
                    index == tabIndex ? i.component : null
                ))}

            </div>
        </div>
    )
}