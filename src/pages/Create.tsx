import {useEffect} from "react";
import {invoke} from "@tauri-apps/api/core";

export function Create(){

    useEffect(() => {
        console.log("hi")

        invoke("list_minecraft_version").then((res) => {
            console.log()
            console.log("hi"+res)
        })

    })

    return (
        <div>
            <h1>Create</h1>
        </div>
    )
}