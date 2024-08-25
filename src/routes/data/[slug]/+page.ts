import { ProjectAPI } from '$lib'

export async function load({params}){
    let slug = params.slug
    try{

        let res = await ProjectAPI.getDependentVarData(slug)
        return {data:res}
    }catch(err){
        console.error("[Failed] Load dependent data. ", err)
    }
    
    return {data:[]}
}