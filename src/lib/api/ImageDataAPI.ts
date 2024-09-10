import type { ImageData } from "$lib/global_types";
import { invoke } from "@tauri-apps/api/tauri";

export default {
    getAllClasses:async function(projectName:string): Promise<string[]>{
        try{
            let response:string = await invoke("get_class_names", {projectName})
            let res:{data:string[]} = JSON.parse(response)
            if (res.data){
                return res.data
            }
            throw Error(`ImageDataAPIFailed: no data returned. ${res}`)
        }catch(err){
            throw err
        }
    },
    getUnverifiedImages: async function (projectName:string, className:string): Promise<ImageData[]>{
        try{
            let response:string = await invoke("get_unverified_images_for_class", {projectName, className})
            let result = JSON.parse(response)
            if (result.data){
                return result.data
            }
            throw Error(`GetUnverifiedImagesFailed. Did not return data from API. ${result}`)
        }catch(err){
            throw err
        }
    }
}