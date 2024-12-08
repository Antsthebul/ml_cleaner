import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { ResponseType, Project } from "$lib/global_types";

export const projects = writable([] as Project[]);

class APIError extends Error {
    constructor(message: string) {
      super(message);
      this.name = 'DatabaseError';
    }
  }
  
export  async function loadProjects(){
    console.log("Loading All Projects")
    try{

        let response:string = await invoke("get_all_projects")
        let result:ResponseType<Project[]> = JSON.parse(response); 
        console.log("eh ", result.data)
        if (result.data){
            projects.set(result.data)
        }
        throw new APIError(result.error)
    }catch(err){
        throw err
    }
}
