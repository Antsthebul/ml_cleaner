import { ImageDataAPI, ProjectAPI } from '$lib'
import type { ImageData } from '$lib/global_types.js'

export async function load({params, url}){
    let slug = params.slug

    let projName = url.searchParams.get("project")
    let page = ""
    try{
        let res = await ImageDataAPI.getUnverifiedImages(projName as string, slug)
        return {data:{images:res}}
    }catch(err){
        console.error("[Failed] Load dependent data. ", err)
    }
    
    return {data:{images:[]} as {images: ImageData[], next_page?:string, previous_page?:string}}
}