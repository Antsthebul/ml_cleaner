export interface Project {
    name: string
    classes_file: string,
    machine: ProjectMachine | null
}

export interface ResponseType<T>{
    data: T,
    error:string
}

export type SimpleSuccessResponse = ResponseType<string>

type ModifiedMachine = Omit<Machine, 'machineType'|'state'>;
export interface ProjectMachine extends ModifiedMachine{
    machine_type: string
}
/** Machine returned by server*/  
export interface Machine{
    id:string,
    name:string,
    machineType:string,
    state:string
}