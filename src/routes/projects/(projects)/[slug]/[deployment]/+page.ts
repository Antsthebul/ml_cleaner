import { ProjectAPI } from '$lib';
import type { Deployment, ClassData } from "$lib/global_types";


export async function load({params}){
    let {slug, deployment:deployName} = params
    console.log(`Load ProjectDetail Deployment - slug: ${slug}, deployment: ${deployName} `)

    try{

        let res:{deployment:Deployment, classes_data:ClassData} = await ProjectAPI.getProjectDeployment(slug, deployName)
        let {deployment, classes_data} = res
        if (!deployment){
            throw Error(`project detail does not contain deployment. returned ${JSON.stringify(res)}`)
        }
        return {data:{deployment, classes_data: classes_data}}
    }catch(err){
        console.error("[Failed] Loading ProjectDetail -> Deployment. ",err)
    }

    return {data:{deployment:null, classes_data:{classes:[] as any, file_exists:false}}}
}