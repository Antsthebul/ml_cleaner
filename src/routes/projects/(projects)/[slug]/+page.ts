import { ImageDataAPI } from "$lib"

export async function load({params, parent}){
    console.log("Loading - Project Detail Page")
    await parent()
    try{
        console.log('fetching images')
        let data = await ImageDataAPI.getAllClasses(params.slug)
        console.log("got em")
        // let data = []
        return {classes:data}
    }catch(err){
        console.error(`ProjectHomeLoadFailed: Unable to load classes data. ${err}`)
        return {classes:[]}
    }
}