//**This API handler solely deals with relating to the un/verified images*/

import type { ImageData, ImageDataCollection } from "$lib/global_types";
import { invoke } from "@tauri-apps/api/core";

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
    getUnverifiedImages: async function (projectName:string, className:string, page:string|null): Promise<ImageDataCollection>{
        if (!page){
            page = ""
        }
        try{
            let response:string = await invoke("get_unverified_images_for_class", {projectName, className, page})
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