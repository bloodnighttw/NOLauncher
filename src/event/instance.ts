interface InstanceStatusChange{
    status: "Downloading" | "Checking" | "Preparing" | "Stopped" | "Failed" | "Running",
}

interface ProgressChange{
    now:number,
    total:number
}