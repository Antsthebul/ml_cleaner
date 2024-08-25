import ConfigAPI from "$lib/api/ConfigAPI"

export async function load(){
    try{

        let config = await ConfigAPI.getConfig()
        return {data:config}
    }catch(err){
        console.error(`[Failed] Retrieving config unsuccessful. ${err}`)
    }
    return {data:null}
}