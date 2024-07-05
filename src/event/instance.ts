interface InstanceStatusChange{
    type: "Downloading" | "Checking" | "Preparing" | "Stopped" | "Failed" | "Running",
    now?: number,
    total?: number
}

interface ProgressChange{
    now:number,
    total:number
}