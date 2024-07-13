import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";
import type { ResponseType, Project } from "./global_types";

export const projects = writable([] as Project[]);

export  async function loadProjects(){
    console.log("Fetching Data for '/' route")
    let response:string = await invoke("get_all_projects")
    let result:ResponseType<Project[]> = JSON.parse(response); 
    projects.set(result.data)
}
