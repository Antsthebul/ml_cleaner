import { ProjectAPI } from '$lib';
import type { Deployment, ClassData } from "$lib/global_types";


export async function load({params}){
    let {slug, deployment:deployName} = params
    console.log(`Load ProjectDetail Deployment - slug: ${slug}, deployment: ${deployName} `)

    try{

        let deployment: Deployment = await ProjectAPI.getProjectDeployment(slug, deployName)
        return {data:deployment}
    }catch(err){
        console.error("[Failed] Loading ProjectDetail -> Deployment. ",err)
    }

    return {data:null}
}