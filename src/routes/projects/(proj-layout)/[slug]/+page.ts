import { ImageDataAPI } from "$lib"

export async function load({params, parent}){
    console.log("Project Detail PAGE LOAD")
    try{
        let data = await ImageDataAPI.getAllClasses(params.slug)
        // let data = []
        return {classes:data}
    }catch(err){
        console.error(`ProjectHomeLoadFailed: Unable to load classes data. ${err}`)
        return {classes:[]}
    }
}