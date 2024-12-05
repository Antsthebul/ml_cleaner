from fastapi import FastAPI

app = FastAPI()
@app.get("/")
async def ok():
    return "ok"

@app.patch("/{machine_id}/start")
async def start_machine(machine_id):
    return{
        "id": machine_id,
        "name":"Test Machine",
        "state":"stopping",
        "machineType":"A4000",
        "publicIpAddress": "1.1.1.1"
    }

@app.get("/{machine_id}")
async def get_machine(machine_id:str):
    response = {
        "id": machine_id,
        "name":"Test Machine",
        "state":"stopping",
        "machineType":"A4000",
        "publicIpAddress": "1.1.1.1"
    }
    return response