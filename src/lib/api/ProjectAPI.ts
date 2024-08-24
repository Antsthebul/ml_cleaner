import type { Project, ResponseType } from "$lib/global_types";
import { invoke } from "@tauri-apps/api/tauri";

export default {
    getProjectEnvironment: async function(projectName:string,envName:string){
        let res = await invoke("get_project_environment", {projectName, envName})
        return res
    },
    getProjectByName: async function(projectName:string): Promise<ResponseType<Project>>{
        try{

            let res: string = await invoke("get_project_by_project_name", {projectName})
            let data: ResponseType<Project> = JSON.parse(res)
            return data
        }catch(err){
            throw err
        }
    }
}