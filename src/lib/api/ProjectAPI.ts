import type { Deployment, Project, ProjectResponse, ResponseType, ClassData, ImageData } from "$lib/global_types";
import { invoke } from "@tauri-apps/api/tauri";

export default {
    getProjectDeployment: async function(projectName:string,deployName:string){
        try{

            let res: string = await invoke("get_project_deployment", {projectName, deployName})
            let data: ResponseType<Deployment> = JSON.parse(res) 
            return data.data
        }catch(err){
            throw err
        }
    },
    
    getProjectByName: async function(projectName:string): Promise<ProjectResponse>{
        try{

            let res: string = await invoke("get_project_by_project_name", {projectName})
            let data: ResponseType<ProjectResponse> = JSON.parse(res)
   
            return data.data
        }catch(err){
            throw err
        }
    },
    
    /**Retreives data from repo directly */
    getDependentVarData: async function (projectName:string, depName:string, page?:string):Promise<{images:ImageData[], next_page?:string, previous_page?:string}>{
        try{
            let res:string = await invoke("get_data_for_class", {projectName, depName, page})
            let data = JSON.parse(res)
            return data.data
        }catch(err){
            throw err
        }
    }
}