import type { NewDeployment, ResponseType } from "$lib/global_types";
import { invoke } from "@tauri-apps/api/core";


export default {
    createDeployment: async function (data: NewDeployment) {
        try{

            let raw_response:string = await invoke("create_deployment", 
                    {name:data.name, description:data.description, 
                        tags:data.tags,
                        project_id:data.project_id})
            let result:ResponseType<string> = JSON.parse(raw_response)
            if (result.data){
                return result.data 
            }
            
        } catch(err){
            console.log("DeplyomentAPIError: ", err)
        }
    }
}