import { ProjectAPI } from '$lib';

export async function load({params}){
    let slug = params.slug
    let res = await ProjectAPI.getProjectByName(slug)
    return {data:res.data}
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