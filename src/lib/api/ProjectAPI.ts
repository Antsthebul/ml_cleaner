import type { Deployment, Project, ResponseType, ClassData, ImageData } from "$lib/global_types";
import { invoke } from "@tauri-apps/api/tauri";

type ProjectResponse = {project:Project, classes:string[]}

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
    
    getProjectByName: async function(projectName:string): Promise<ProjectResponse>{
        try{

            let res: string = await invoke("get_project_by_project_name", {projectName})
            let data: ResponseType<ProjectResponse> = JSON.parse(res)
   
            return data.data
        }catch(err){
            throw err
        }
    },
    getDependentVarData: async function (projectName:string, depName:string):Promise<ImageData>{
        try{
            let res:string = await invoke("get_data_for_class", {projectName, depName})
            let data = JSON.parse(res)
            return data.data
        }catch(err){
            throw err
        }
    }
}