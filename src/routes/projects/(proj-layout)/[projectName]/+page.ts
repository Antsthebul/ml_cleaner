import { ImageDataAPI } from "$lib"

// export async function load({params}){
//     try{
//         let data = await ImageDataAPI.getAllClasses(params.projectName)
//         return {classes:data}
//     }catch(err){
//         console.error(`ProjectHomeLoadFailed: Unable to load classes data. ${err}`)
//         return {classes:[]}
//     }
// }
import { loadProjectDetail } from '$lib/store';

export async function load({params}){
    console.log("PROJECT NAME PAGE LOAD")
    let projectName = params.projectName
    let res = await loadProjectDetail(projectName as string)
    return res
}
