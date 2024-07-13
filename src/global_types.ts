export interface Project {
    classKey:string|null
}

export interface ResponseType<T>{
    data: T,
    error:string
}