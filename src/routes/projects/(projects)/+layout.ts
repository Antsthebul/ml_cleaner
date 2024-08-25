import { ProjectAPI } from '$lib';

export async function load({params}){
    let slug = params.slug
    console.log(`Load ProjectDetail Layout - slug: '${slug}'`)
    let res = await ProjectAPI.getProjectByName(slug as string)
    return {data:res}
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