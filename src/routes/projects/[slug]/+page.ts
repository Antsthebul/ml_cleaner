import { invoke } from '@tauri-apps/api/tauri';

/** @type {import('./$types').PageLoad} */
export async function load(){
    let result = JSON.parse(await invoke("search_bucket"))
    if (result){
        
        return {"data":{ ...result.data, classes:result.data.classes.map((className:string)=> className.replaceAll("_", " "))
                .map((className:string)=>titleCase(className) )} }
    }
    return result
}

function titleCase(text:string){
    let result = '';
    for (let i=0; i < text.length; i++){
        if (i == 0 || text[i - 1] === " "){
            result += text[i].toUpperCase();
        }else {
            result += text[i];
        }
    }
    return result
}