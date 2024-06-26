import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {CenterView} from "../component/Compose.tsx";

export function Create(){

    const [version, setVersion] = useState<MinecraftVersionInfo | null>(null)

    useEffect(() => {
        console.log("hi")

        invoke<MinecraftVersionInfo>("list_versions").then((res) => {
            setVersion(res)
        })

    },[setVersion])

    return (
            <CenterView>
                <div role="tablist" className="tabs tabs-bordered">
                    <input type="radio" name="my_tabs_1" role="tab" className="tab" aria-label="Tab 1"/>
                    <div role="tabpanel" className="tab-content p-10">Tab content 1</div>

                    <input
                        type="radio"
                        name="my_tabs_1"
                        role="tab"
                        className="tab"
                        aria-label="Tab 2"
                        defaultChecked/>
                    <div role="tabpanel" className="tab-content p-10"  data-tauri-drag-region={true}>
                        <button className="btn" onClick={() => document.getElementById('my_modal_2').showModal()}>open
                            modal
                        </button>
                        <dialog id="my_modal_2" className="modal" >
                            <div className="modal-box">
                                <h3 className="font-bold text-lg">Hello!</h3>
                                <p className="py-4">Press ESC key or click outside to close</p>
                            </div>
                            <form method="dialog" className="modal-backdrop">
                                <button>close</button>
                            </form>
                        </dialog>
                    </div>

                    <input type="radio" name="my_tabs_1" role="tab" className="tab" aria-label="Tab 3"/>
                    <div role="tabpanel" className="tab-content p-10">Tab content 3</div>
                </div>
            </CenterView>
    )
}