interface Skin{
    id:string,
    state:string,
    url:string,
    texture_key:string,
    variant_key:string,
}

interface Caps{
    id:string,
    state:string,
    url:string,
    alias:string,
}

interface Profile{
    id:string,
    username:string,
    skins:Array<Skin>,
    caps:Array<Caps>,
}