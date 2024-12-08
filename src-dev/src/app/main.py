from fastapi import FastAPI, Depends
from fastapi.responses import JSONResponse
from fastapi.testclient import TestClient
import asyncio
from repository.machine_repository import MachineRepository, Machine, MachineState
from typing_extensions import Annotated
import httpx


app = FastAPI()

client = TestClient(app)

async def http_get(uri:str):
    async with httpx.AsyncClient() as c:
        await c.get("http://127.0.0.1:8000"+uri, timeout=30)

async def get_machine_repository():
    conn = MachineRepository()
    await conn.set_connection()
    yield conn
    await conn.close()

    
async def start_machine(machine:Machine):
    if machine.state in [MachineState.READY, MachineState.STARTING]:
        await asyncio.sleep(3)
        await http_get(f"/{machine.id}/state_update?state={MachineState.STARTING.value}")
        await asyncio.sleep(3)
        await http_get(f"/{machine.id}/state_update?state={MachineState.READY.value}")


async def shutdown_machine(machine:Machine):
    if machine.state not in [MachineState.OFF]:
        await asyncio.sleep(3)
        await http_get(f"/{machine.id}/state_update?state={ MachineState.STOPPING.value}")
        await asyncio.sleep(3)
        await http_get(f"/{machine.id}/state_update?state={MachineState.OFF.value}")


@app.get("/")
async def ok():
    return "ok"

@app.patch("/{machine_id}/start")
async def machine_run(machine_id:str, 
                      machine_repo:Annotated[MachineRepository, Depends(get_machine_repository)]):
    
    print(f"GET machine_run route hit; {machine_id=}")
    machine = await machine_repo.get_machine(machine_id)
    if machine:
        asyncio.create_task(start_machine(machine))
    else:
       machine = await machine_repo.create_machine(machine_id)
       asyncio.create_task(start_machine(machine))
    machine.state = MachineState.STARTING
    return {"data":machine.to_json()}

@app.patch("/{machine_id}/stop")
async def machine_shutdown(machine_id:str,
                      machine_repo:Annotated[MachineRepository, Depends(get_machine_repository)]):
                           
    print(f"PATCH machine_shutdown endpoint called. {machine_id=}")
    machine = await machine_repo.get_machine(machine_id)
    if machine:
        asyncio.create_task(shutdown_machine(machine))
        return {"data":machine.to_json()}
    
    return JSONResponse(status_code=404, content={"error":"Machine not found"})


@app.get("/{machine_id}/state_update")
async def update_state(machine_id:str, 
                       state:MachineState,
                      machine_repo:Annotated[MachineRepository, Depends(get_machine_repository)]):
        
        print(f"GET state_update route hit. {machine_id=}, {state=}", flush=True)
        machine = await machine_repo.get_machine(machine_id)
        ip_address = machine.publicIp
        if state == MachineState.OFF:           
            ip_address = None
        elif state == MachineState.STARTING:
            ip_address = "198.104.3.4"
        else:
            print("No work")
        await machine_repo.update_machine(state, machine.id, ip_address)
        return "ok"


@app.get("/{machine_id}")
async def retrieve_machine(machine_id:str, 
                      machine_repo:Annotated[MachineRepository, Depends(get_machine_repository)]):
                           
    print(f"GET retrieve_machine route hit. {machine_id=}")
    machine = await machine_repo.get_machine(machine_id)
    if machine:
        return machine.to_json()
    return JSONResponse(status_code=404, content={"error":"Machine not found"})

