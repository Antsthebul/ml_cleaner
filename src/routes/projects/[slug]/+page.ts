import { invoke } from '@tauri-apps/api/tauri';

/** @type {import('./$types').PageLoad} */
export async function load(){
    let result = JSON.parse(await invoke("search_bucket"))
    return result
}
