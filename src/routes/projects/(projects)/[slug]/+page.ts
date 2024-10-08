import { ImageDataAPI } from "$lib"

export async function load({params}){
    console.log("Loading - Project Detail Page")
    try{
        let data = await ImageDataAPI.getAllClasses(params.slug)
        return {classes:data}
    }catch(err){
        console.error(`ProjectHomeLoadFailed: Unable to load classes data. ${err}`)
        return {classes:[]}
    }
}