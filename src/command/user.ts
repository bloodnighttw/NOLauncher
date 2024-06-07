interface Skin{
    id:String,
    state:String,
    url:String,
    texture_key:String,
    variant_key:String,
}

interface Caps{
    id:String,
    state:String,
    url:String,
    alias:String,
}

interface Profile{
    id:String,
    username:String,
    skins:Array<Skin>,
    caps:Array<Caps>,
}

interface LoginAccount{
    profile:Profile
}