import { ProjectAPI } from '$lib'

export async function load({params, url}){
    let slug = params.slug
    let projName = url.searchParams.get("project")
    try{

        let res = await ProjectAPI.getDependentVarData(projName as string, slug)
        return {data:res}
    }catch(err){
        console.error("[Failed] Load dependent data. ", err)
    }
    
    return {data:[]}
}