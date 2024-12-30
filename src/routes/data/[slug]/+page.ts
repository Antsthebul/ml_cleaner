import { ImageDataAPI, ProjectAPI } from '$lib'
import type { ImageData, ImageDataCollection } from '$lib/global_types.js'

export async function load({params, url}){
    let projectName = params.projectName

    let projName = url.searchParams.get("project")
    try{
        let res = await ImageDataAPI.getUnverifiedImages(projName as string, projectName, null)
        return {data:res}
    }catch(err){
        console.error("[Failed] Load dependent data. ", err)
    }
    let initImageData:ImageDataCollection = {images:[] as ImageData[], previous_page:null, next_page:null}
    return {data: initImageData}
}