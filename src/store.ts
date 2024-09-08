import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";
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
    console.log("Store - Loading All Projects")
    try{

        let response:string = await invoke("get_all_projects")
        let result:ResponseType<Project[]> = JSON.parse(response); 
        if (result.data){
            projects.set(result.data)
        }
        throw new APIError(result.error)
    }catch(err){
        throw err
    }
}

export async function loadProjectDetail(slug:string){
    console.log("Store - SingleProjectLoad")
    try{
        let res = await ProjectAPI.getProjectByName(slug)
        projectDetailStore.set(res)
        return res
    }catch(err){
        throw err
    }
}