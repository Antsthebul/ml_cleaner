import type { Deployment, Project, ResponseType, ClassData } from "$lib/global_types";
import { invoke } from "@tauri-apps/api/tauri";

export default {
    getProjectDeployment: async function(projectName:string,deployName:string){
        try{

            let res: string = await invoke("get_project_deployment", {projectName, deployName})
            let data: ResponseType<{deployment:Deployment, classes_data:ClassData}> = JSON.parse(res) 
            return data.data
        }catch(err){
            throw err
        }
    },
    
    getProjectByName: async function(projectName:string): Promise<Project>{
        try{

            let res: string = await invoke("get_project_by_project_name", {projectName})
            let data: ResponseType<Project> = JSON.parse(res)
   
            return data.data
        }catch(err){
            throw err
        }
    }
}