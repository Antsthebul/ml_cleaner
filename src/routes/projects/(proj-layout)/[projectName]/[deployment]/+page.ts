import { ProjectAPI } from '$lib';
import type { Deployment, ClassData } from "$lib/global_types";


export async function load({params}){
    let {projectName, deployment:deployName} = params
    console.log(`Load ProjectDetail Deployment - projectName: ${projectName}, deployment: ${deployName} `)

    try{

        let deployment: Deployment = await ProjectAPI.getProjectDeployment(projectName, deployName)
        return {data:deployment}
    }catch(err){
        console.error("[Failed] Loading ProjectDetail -> Deployment. ",err)
    }

    return {data:null}
}