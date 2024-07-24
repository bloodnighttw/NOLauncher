import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {useNavigate} from "react-router-dom";

interface Props{
    name:string
}

function NewInstance(props:Props) {
    const [platform, setPlatform] = useState<string>("Minecraft")
    const [versionInfo,setVersionInfo] = useState<MinecraftVersionInfo|null>(null);

    const [showRelease, setRelease] = useState(true);
    const [showSnapshot, setSnapshot] = useState(false);
    const [showBeta, setBeta] = useState(false);
    const [showAlpha, setAlpha] = useState(false);
    const [showExperiment, setExperiment] = useState(false);

    const [cacheIntermediary,setCacheIntermediary] = useState<Set<string | null>>(new Set())
    const [cacheForge,setCacheForge] = useState<Set<string | null>>(new Set())  // dep to version
    const [cacheLite,setCacheLite] = useState<Set<string | null>>(new Set())  // dep to version
    const [cacheNeoForge,setCacheNeoForge] = useState<Set<string | null>>(new Set()) // dep to version

    const [selectedVersion,setSelectedVersion] = useState<string>("unselected");
    const [selectedMod,setSelectedMod] = useState<string>("unselected");

    const navigate = useNavigate();


    const typeFilter = (type: string | null) => {

        switch (type) {
            case "release":
                return showRelease;
            case "old_snapshot":
            case "snapshot":
                return showSnapshot;
            case "old_beta":
                return showBeta;
            case "old_alpha":
                return showAlpha;
            case "experiment":
                return showExperiment;
        }

        return false
    }

    const modFilter = (version: string | null) => {

        switch (platform) {
            case "Quilt":
            case "Fabric":
                return cacheIntermediary.has(version);
            case "Forge":
                return cacheForge.has(version);
            case "NeoForge":
                return cacheNeoForge.has(version);
            case "Liteloader":
                return cacheLite.has(version);
            default:
                return true;
        }
    }

    const modVersion = (version: string) => {
        switch (version){
            case "Quilt":
                return versionInfo?.quilt
            case "Fabric":
                return versionInfo?.fabric_loader
            case "Forge":
                return versionInfo?.forge
            case "NeoForge":
                return versionInfo?.neoforge
            case "Liteloader":
                return versionInfo?.liteloader
            default:
                return null
        }
    }

    const dataValid = () => selectedVersion != "unselected" && (platform == "Minecraft" ||  selectedMod != "unselected") && props.name != ""

    useEffect(()=>{
        invoke<MinecraftVersionInfo>("list_versions").then((res)=>{
            setVersionInfo(res)
            setCacheIntermediary(new Set(res.intermediary.map((v)=> v.dep)))
            setCacheForge(new Set(res.forge.map((v)=> v.dep)))
            setCacheLite(new Set(res.liteloader.map((v)=> v.dep)))
            setCacheNeoForge(new Set(res.neoforge.map((v)=> v.dep)))
        }).catch(console.error)
    },[setVersionInfo,setCacheIntermediary,setCacheForge,setCacheLite,setCacheNeoForge])

    const loader = [
        "Minecraft",
        "Fabric",
        "NeoForge",
        "Forge",
        "Quilt",
        "Liteloader",
    ]

    const platformUnactive = "tab rounded-md duration-200 active:scale-90 hover:bg-base-200";
    const platformActive = "tab rounded-md duration-200 bg-base-200 active:scale-90";

    return <div className="py-1.5 w-full gap-2">
        <div className="label">
            <span className="label-text">Pick the platform you want to use:</span>
        </div>
        <div role="tablist" className="tabs bg-base-100 rounded-md p-0.5">
            {loader.map((name) => {
                return <div role="tab" className={platform == name ? platformActive : platformUnactive}
                            onClick={() => {
                                setSelectedVersion("unselected")
                                setSelectedMod("unselected")
                                setPlatform(name)
                            }}>{name}</div>
            })}
        </div>

        <div className="label">
            <span className="label-text">Minecraft Version Filter</span>
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

        <select className="select select-bordered w-full select-sm"
                value={selectedVersion}
                onChange={(e) => {
                    setSelectedVersion(e.target.value)
                }}
        >
            <option disabled selected value="unselected">Selected Version</option>
            {versionInfo?.minecraft
                .filter((v) => typeFilter(v.rtype) && modFilter(v.version))
                .map((v) => {
                    return <option value={v.version}>{v.version}</option>
                })}
        </select>
        {
            platform != "Minecraft" ? <div className="duration-200">
                <div className="label">
                    <span className="label-text">Select {platform} Version</span>
                </div>

                <select className="select select-bordered w-full select-sm"
                        disabled={selectedVersion == "unselected"}
                        value={selectedMod}
                        onChange={(e) => {
                            setSelectedMod(e.target.value)
                        }}
                >
                    <option value="unselected" disabled>Select version to continue</option>

                    {selectedVersion == "unselected" ?
                        <option disabled selected>Please Select Minecraft Version First</option> : null}
                    {modVersion(platform)?.filter((v) => {
                        if (v.dep == null) return true
                        return v.dep === selectedVersion
                    }).map((v) => {
                        return <option value={v.version}>{v.version}</option>
                    })}
                </select>
            </div> : null
        }


        <div className="flex justify-end label">
            <button className="btn btn-sm bg-base-100 duration-400"
                    disabled={!dataValid()}
                    onClick={() => {
                        let request:InstanceCreateRequest = {
                            name: props.name,
                            ptype: platform,
                            version: selectedVersion,
                            mod_version: selectedMod
                        };
                        invoke("create_instance", {
                            request:request
                        }).then(()=>navigate("/")).catch(console.error)
                    }}
            >Create!</button>
        </div>


    </div>
}

export function Create() {

    const [tabIndex, setTabIndex] = useState(0)
    const [name,setName] = useState<string>("")


    const tab = "tab duration-200";
    const tabActive = "tab tab-active duration-200";
    const tabs = [
        {long_name: "Create", component: <NewInstance name={name}/>},
        {long_name: "Import", component: null},
    ];

    useEffect(()=>{
        invoke("pkg_info",{uid:"net.minecraftforge"}).then(console.log)
    })



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