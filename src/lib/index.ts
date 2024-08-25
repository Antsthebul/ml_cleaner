// place files you want to import through the `$lib` alias in this folder.
export {default as Nav} from "./Nav/components/Nav.svelte"
export {default as Machines} from "./Machines/components/Machines.svelte"
export {default as MachineAPI} from "./api/MachineAPI"
export {default as ProjectAPI} from "./api/ProjectAPI"
export {default as DependVarWindow} from "./components/Project/DependVarWindow.svelte";
export {default as ProjectEnvironment} from "./components/Project/ProjectEnvironment.svelte"

export *  as store from "./store";
export * as types from "./global_types";