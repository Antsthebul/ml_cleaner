import { invoke } from "@tauri-apps/api/tauri";
import type { ConfigurationResponse,ResponseType, Configuration } from "../../global_types";

type ConfigurationAPIError = string;
let ConfigurationError:ConfigurationAPIError;

export default{
    getConfig:async function (): Promise<Configuration>{
        try{

            let raw_response:string = await invoke("get_config")
            let result:ResponseType<ConfigurationResponse> = JSON.parse(raw_response)
            if (result.data){
                return result.data.configuration 
            }
            ConfigurationError = result.error;
            throw ConfigurationError
        }catch(err){
            throw err
        }
    }
}
