export interface Project {
    name: string
    deployments: Deployment[],
    train_file: FileAttr,
    test_file: FileAttr

}
export interface FileAttr{
    path:string,
    exists:boolean
}
export type ProjectResponse = {project:Project}
export interface Deployment {
    name: string
    files: any,
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
    ip_address:string|null
}
/** Machine returned by server*/  
export interface Machine{
    id:string,
    name:string,
    machineType:string,
    state:string,
    ip_address:string|null
}

export interface ConfigurationResponse{
    configuration:Configuration
}
export interface Configuration{
    default_machine:string | null
}
export interface ImageDataCollection{
    images: ImageData[],
    previous_page:string | null,
    next_page:string | null
}
export interface ImageData{
    b64:string,
    file_path:string,
    verified?:boolean
}

export interface TrainingData {
    machineId: string,
    trainData:TrainingDataDataSeries

}

export interface TrainingDataDataSeries {
    epoch:string,
    train_loss:number,
    train_acc:string,
    val_loss: number,
    test_acc:number
}

export interface NewDeployment {
    name: string,
    description: string,
    tags: string[],
    project_id:number
}