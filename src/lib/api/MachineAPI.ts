import { invoke } from "@tauri-apps/api/core"
import type { Machine, ResponseType, SimpleSuccessResponse, Configuration} from "../../global_types"


type MachineAPILoadError= string;
type MachineAPIStartError = string;
type MachineAPIUpdateError = string;
let MachineLoadError:MachineAPILoadError;
let MachineStartError:MachineAPIStartError;
let MachineUpdateError:MachineAPIUpdateError

export default {
    listMachines:async function(): Promise<Machine[]>{
        console.log("GET machine list")
        try{
    
            let data:string = await invoke("list_machines")
            let result:ResponseType<Machine[]> = JSON.parse(data)
            if (result.data){
                return result.data
            }
                MachineLoadError = result.error
                throw MachineLoadError

        }catch(err){
            throw err
        }
    },
    /**Returns entire machine information */
    getMachineByMachineId:async function (machineId:string){

        console.log("GET machine status")
        try{

            let data:string = await invoke("get_machine_by_machine_id", {machineId})
            let result:ResponseType<Machine> = JSON.parse(data)
            if (result.error){
                return result.data
            }else{
                MachineLoadError = result.error
                throw MachineLoadError
            }
        }catch(err){
            throw err
        }

    },
    startMachine:async function (machineId:string):Promise<void>{
        console.log(`Starting machine ${machineId}`)
        try{

            let data:string = await invoke("start_machine", {machineId})
            let  result:SimpleSuccessResponse = JSON.parse(data)
            if (result.error){
                MachineStartError = result.error
                throw MachineStartError
            }

        }catch(err){
            throw err
        }
    },
    updateDefaultMachine:async function (machineId:string, configuration:Configuration): Promise<Configuration>{
        let machineValue = machineId === configuration.default_machine ? "resetDefaultMachine":machineId
        try{

            let data:string = await invoke("update_configuration_file_command", {machineId:machineValue})
            let result: ResponseType<Configuration> = JSON.parse(data)
            if (result.data){
                return result.data
            }
            MachineUpdateError = result.error
            throw MachineUpdateError
        }catch(err){
            throw err
        }
    
    }

}