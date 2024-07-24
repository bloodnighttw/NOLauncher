import {useRef, useState } from "react";
import useCommand from "../../state-hook/hook/useCommand";
import { invoke } from "@tauri-apps/api/core";

enum DependOn{
    NONE,
    Vanilla,
    Intermediary
}

interface VersionPayload{
    type:string,
    version:string,
    require:string | null | undefined
}


interface Props{
    name:string
}

export function NewInstance(props:Props) {


    const PackageInfo:Array<[string,string,DependOn]> = [
        ["Minecraft","net.minecraft",DependOn.NONE],
        ["Fabric","net.fabricmc.fabric-loader",DependOn.Intermediary],
        ["NeoForge","net.neoforged",DependOn.Vanilla],
        ["Forge","net.minecraftforge",DependOn.Vanilla],
        ["Quilt","org.quiltmc.quilt-loader",DependOn.Intermediary],
        ["Liteloader","com.mumfrey.liteloader",DependOn.Vanilla]
    ]


    const [platform, setPlatform] = useState<[string,string,DependOn]>(PackageInfo[0])


    const setRef = useRef(new Set<string>())

    const [vanilla,_v] = useCommand<Array<VersionPayload>>("pkg_info",{uid:"net.minecraft"})
    const [mods,setMods] = useState<Array<VersionPayload>|null>(null);

    const [selectedVersion,setSelectedVersion] = useState<string>("unselected");
    const [selectedMod,setSelectedMod] = useState<string>("unselected");

    const [release,setRelease] = useState<boolean>(true);
    const [snapshot,setSnapshot] = useState<boolean>(false);
    const [experiment,setExperiment] = useState<boolean>(false);
    const [beta,setBeta] = useState<boolean>(false);
    const [alpha,setAlpha] = useState<boolean>(false);


    // const navigate = useNavigate();

    const platformInactive = "tab rounded-md duration-200 active:scale-90 hover:bg-base-200";
    const platformActive = "tab rounded-md duration-200 bg-base-200 active:scale-90";


    return <div className="py-1.5 w-full gap-2">
        <div className="label">
            <span className="label-text">Pick the platform you want to use:</span>
        </div>
        <div role="tablist" className="tabs bg-base-100 rounded-md p-0.5">
            {
                PackageInfo.map(([name,uid,dep])=>(
                    <div role="tab" className={platform[0] == name ? platformActive : platformInactive}
                        onClick={() => {
                            setPlatform([name,uid,dep])
                            setRef.current.clear()
                            setSelectedMod("unselected")
                            setSelectedVersion("unselected")
                            
                            setMods(null)

                            invoke<Array<VersionPayload>>("pkg_info",{uid:uid}).then((data)=>{
                                if(dep == DependOn.Intermediary){
                                    invoke<Array<VersionPayload>>("pkg_info",{uid:"net.fabricmc.intermediary"})
                                    .then((v)=>{
                                        v.map((v)=>{if(v.require) setRef.current.add(v.require)})
                                        setMods(data)
                                    })
                                }else{
                                    data.map((v)=>{if(v.require) setRef.current.add(v.require)})
                                    setMods(data)
                                }
                            })

                            
                        }}>{name}</div>
                ))
            }
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
                    setSelectedMod("unselected")
                }}
        >
            <option disabled selected value="unselected">Selected Version</option>
            {
                vanilla?.
                filter((v)=>{
                    if(setRef.current.size == 0) return true
                    return setRef.current.has(v.version)
                }).
                filter((x)=>{
                    if(release && x.type == "release") return true
                    if(snapshot && x.type == "snapshot") return true
                    if(experiment && x.type == "experiment") return true
                    if(beta && x.type == "beta") return true
                    if(alpha && x.type == "alpha") return true
                    return false
                }). 
                map((v)=>{
                    return <option value={v.version}>{v.version}</option>
                })
            }

        </select>
        
         {
            platform[2] !== DependOn.NONE ? <div className="duration-200">
                <div className="label">
                    <span className="label-text">Select {platform} Version</span>
                </div>

                <select className="select select-bordered w-full select-sm"
                        value={selectedMod}
                        onChange={(e) => {
                            setSelectedMod(e.target.value)
                        }}
                        disabled={selectedVersion=="unselected"}
                >
                    <option value="unselected" disabled>Select version to continue</option>
                    {
                        mods?.
                        filter((x)=>{ 
                            if (x.require == null) return true
                            return x.require === selectedVersion
                        }).
                        map((v)=>(
                            <option value={v.version} className="text-green-800">{v.version}</option>
                        ))
                    }
                </select>
            </div> : null
        }


        <div className="flex justify-end label">
            <button className="btn btn-sm bg-base-100 duration-400"
                    onClick={() => {
                        // let request:InstanceCreateRequest = {
                        //     name: props.name,
                        //     ptype: platform,
                        //     version: selectedVersion,
                        //     mod_version: selectedMod
                        // };
                        // invoke("create_instance", {
                        //     request:request
                        // }).then(()=>navigate("/")).catch(console.error)
                    }}
            >Create!</button>
        </div>

        
        {/* <div>{vanilla?.toString()}</div> */}
        


    </div>
}