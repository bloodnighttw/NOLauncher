import {useEffect, useState} from "react";

function NewInstance(){
    const [version, setVersion] = useState<string>("Minecraft")
    const [showRelease, setRelease] = useState(true);
    const [showSnapshot, setSnapshot] = useState(false);
    const [beta, setBeta] = useState(false);
    const [alpha, setAlpha] = useState(false);
    const [experiment, setExperiment] = useState(false);

    const loader=[
        "Minecraft",
        "Fabric",
        "NeoForge",
        "Forge",
        "Quilt"
    ]

    const typeFilter = (type:string|null) =>{

        switch (type) {
            case "release":
                return showRelease;
            case "old_snapshot":
            case "snapshot":
                return showSnapshot;
            case "old_beta":
                return beta;
            case "old_alpha":
                return alpha;
            case "experiment":
                return experiment;
        }

        return false
    }

    const platform = "tab rounded-md duration-200 active:scale-90 hover:bg-base-200";
    const platformActive = "tab rounded-md duration-200 bg-base-200 active:scale-90";

    return <div className="py-1.5 w-full gap-2">
        <div className="label">
            <span className="label-text">Pick the platform you want to use:</span>
        </div>
        <div role="tablist" className="tabs bg-base-100 rounded-md p-0.5">
            {loader.map((name) => {
                return <div role="tab" className={version == name ? platformActive : platform}
                            onClick={() => setVersion(name)}>{name}</div>
            })}
        </div>
        <div className="label">
            <span className="label-text">Select Minecraft Version</span>
        </div>

        <div className="flex flex-wrap gap-2">
            <div className="w-40 px-2">
                <label className="label cursor-pointer">
                    <span className="label-text">Release</span>
                    <input type="checkbox" className="toggle" defaultChecked onChange={(e) => {
                        setRelease(e.target.checked)
                    }}/>
                </label>
            </div>
            <div className="w-40 px-2">
                <label className="label cursor-pointer">
                    <span className="label-text">Snapshot</span>
                    <input type="checkbox" className="toggle" onChange={(e) => {
                        setSnapshot(e.target.checked)
                    }}/>
                </label>
            </div>
            <div className="w-40 px-2">
                <label className="label cursor-pointer">
                    <span className="label-text">Experiment</span>
                    <input type="checkbox" className="toggle" onChange={(e) => {
                        setExperiment(e.target.checked)
                    }}/>
                </label>
            </div>

            <div className="w-40 px-2">
                <label className="label cursor-pointer">
                    <span className="label-text">Beta</span>
                    <input type="checkbox" className="toggle" onChange={(e) => {
                        setBeta(e.target.checked)
                    }}/>
                </label>
            </div>

            <div className="w-40 px-2">
                <label className="label cursor-pointer">
                    <span className="label-text">Alpha</span>
                    <input type="checkbox" className="toggle" onChange={(e) => {
                        setAlpha(e.target.checked)
                    }}/>
                </label>
            </div>
        </div>

        <div className="label">
            <span className="label-text">Select Minecraft Version</span>
        </div>

        <select className="select select-bordered w-full select-sm">
            <option disabled selected>Selected Version</option>
            <option value="minecraft">minecraft</option>
            <option value="minecraft">fabric</option>
            <option value="minecraft">forge</option>
            <option value="minecraft">neoforge</option>
        </select>
        {
            version != "Minecraft" ? <div className="duration-200">
                <div className="label">
                    <span className="label-text">Select {version} Version</span>
                </div>

                <select className="select select-bordered w-full select-sm">
                    <option disabled selected>Selected Version</option>
                    <option value="minecraft">minecraft</option>
                    <option value="minecraft">fabric</option>
                    <option value="minecraft">forge</option>
                    <option value="minecraft">neoforge</option>
                </select>
            </div>: null
        }


        <div className="flex justify-end label">
            <button className="btn btn-sm bg-base-100">Create!</button>
        </div>

    </div>
}

export function Create() {

    const [tabIndex, setTabIndex] = useState(0)


    const tab = "tab duration-200";
    const tabActive = "tab tab-active duration-200";
    const tabs = [
        {long_name: "Create", component: <NewInstance/>},
        {long_name: "Import", component: null},
    ];


    const tabClick = (index: number) => {
        setTabIndex(index)
    }

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
                           className="input input-xs input-bordered w-full h-full"/>

                    <select disabled className="select select-bordered w-full select-sm w-full h-full ">
                        <option disabled selected>Tag is Coming soon......</option>
                        <option value="minecraft">minecraft</option>
                        <option value="minecraft">fabric</option>
                        <option value="minecraft">forge</option>
                        <option value="minecraft">neoforge</option>
                    </select>
                </div>


            </div>
            <div className="h-full space-y-2 p-2">
                <div role="tablist" className="tabs tabs-bordered">
                    {tabs.map((i, index) => (
                        <div className={index == tabIndex ? tabActive : tab}
                             onClick={() => tabClick(index)}>{i.long_name}</div>
                    ))}
                </div>

                {tabs.map((i, index) => (
                    index == tabIndex ? i.component : null
                ))}

            </div>
        </div>
    )
}