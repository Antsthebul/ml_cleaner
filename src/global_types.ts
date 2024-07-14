export interface Project {
    name: string
    classes_file: string,
    machine: Machine | null
}

export interface ResponseType<T>{
    data: T,
    error:string
}

export type SimpleSuccessResponse = ResponseType<string>

export interface Machine{
    id:string,
    state:string,
    name:string
}