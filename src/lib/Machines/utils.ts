import type { Machine } from "../../global_types";

export function isRunning(machineId:string, machines:Machine[]):boolean{
    let machine = machines.find(machine=>machine.id === machineId)
    return Boolean(machine?.state === "ready")
}
// async function getStatus(){
//     console.log("getting machine status")
//     let result = await invoke("get_machine_status")
//     console.log("machine st res ",result)
// }


// }





