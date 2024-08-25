export interface Project {
    name: string
    deployments: Deployment[]

}
export interface Deployment {
    name: string
    classes_file: string,
    machines: ProjectMachine[]
}
// Returned with Environemt Response
// TODO: Maybe bundle togetther rather than commenting
export interface ClassData {
    classes: string[],
    file_exists: boolean,
    last_modified:Date
}

export interface ResponseType<T>{
    data: T,
    error:string
}

export type SimpleSuccessResponse = ResponseType<string>

type ModifiedMachine = Omit<Machine, 'machineType'|'state'>;

export interface ProjectMachine extends ModifiedMachine{
    machine_type: string,
    ip_addr:string|null
}
/** Machine returned by server*/  
export interface Machine{
    id:string,
    name:string,
    machineType:string,
    state:string,
    ip_addr:string|null
}

export interface ConfigurationResponse{
    configuration:Configuration
}
export interface Configuration{
    default_machine:string | null
}
