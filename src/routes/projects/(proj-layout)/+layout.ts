import { loadProjectDetail } from '../../../store.js';

export async function load({params}){
    let slug = params.slug
    console.log(`Loading  Project Detail Layout - slug: '${slug}'`)
    let res = await loadProjectDetail(slug as string)
    return res
}