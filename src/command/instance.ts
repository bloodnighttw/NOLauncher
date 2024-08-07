interface SimpleInfo {
    version: string;
    rtype: string | null;
    dep: string | null;
}

interface MinecraftVersionInfo{
     up_to_date:boolean,
     minecraft:Array<SimpleInfo>,
     fabric_loader:Array<SimpleInfo>,
     intermediary:Array<SimpleInfo>,
     forge:Array<SimpleInfo>,
     liteloader:Array<SimpleInfo>,
     neoforge:Array<SimpleInfo>,
     quilt:Array<SimpleInfo>
}

interface InstanceCreateRequest {
    name:string, // instance name
    ptype: string, // platform
    version:string, // minecraft version
    mod_version:string | null // mod loader version, vanilla is None
}

interface InstanceInfo{
    id:string,
    name:string
}