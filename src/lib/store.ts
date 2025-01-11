import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { ResponseType, Project, ProjectResponse } from "$lib/global_types";
import { ProjectAPI } from "$lib";

export const projects = writable([] as Project[]);
export const projectDetailStore = writable({} as ProjectResponse)

class APIError extends Error {
    constructor(message: string) {
      super(message);
      this.name = 'DatabaseError';
    }
  }
  
export  async function loadProjects(){
    console.log("STORE - Loading All Projects")
    try{

        let response:string = await invoke("get_all_projects")
        let result:ResponseType<Project[]> = JSON.parse(response); 
        console.log("Why => ", result)
      
        if (result.data){
            projects.set(result.data)
            return
        }
        throw new APIError(result.error)
    }catch(err){
        console.error("[STORE] - Failed to load projects", err)
    }
}

export async function loadProjectDetail(projectName:string){
    console.log("Store - SingleProjectLoad")
    try{
        let res = await ProjectAPI.getProjectByName(projectName)
        projectDetailStore.set(res)
        return res
    }catch(err){
        throw err
    }
}