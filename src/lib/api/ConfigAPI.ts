import { invoke } from "@tauri-apps/api/tauri";
import type { ResponseType, Configuration } from "$lib/global_types";

type ConfigurationAPIError = string;
let ConfigurationError:ConfigurationAPIError;

export default{
    getConfig:async function (): Promise<Configuration>{
        console.log("Calling 'get_config'")
        try{

            let raw_response:string = await invoke("get_config")
            let result:ResponseType<Configuration> = JSON.parse(raw_response)
            if (result.data){
                return result.data 
            }
            console.log("Not sure ", result)
            ConfigurationError = result.error;
            throw ConfigurationError
        }catch(err){
            console.log("Failed here")
            throw err
        }
    }
}
