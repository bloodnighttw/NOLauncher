import {useEffect} from "react";
import {invoke} from "@tauri-apps/api/core";

export function Create(){

    useEffect(() => {
        console.log("hi")

        invoke<MinecraftVersionInfo>("list_versions").then((res) => {
            console.log("hi"+res.quilt)
        })

    })

    return (
        <div>
            <h1>Create</h1>
        </div>
    )
}