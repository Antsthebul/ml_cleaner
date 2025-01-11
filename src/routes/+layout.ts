import { loadProjects } from "$lib/store"

export const prerender = true
export const ssr = false

export async function load(){
    console.log("MAIN LOAD")
    await loadProjects()
}