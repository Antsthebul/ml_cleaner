import { ProjectAPI } from '$lib';
import { loadProjectDetail } from '../../../store.js';

export async function load({params}){
    let slug = params.slug
    console.log(`Loading  Project Detail Layout - slug: '${slug}'`)
    let res = await loadProjectDetail(slug as string)
    return {...res}
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