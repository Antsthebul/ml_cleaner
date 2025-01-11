from .base import BaseRepository
from enum import Enum
from dataclasses import dataclass, asdict
from typing import Optional

class MachineState(Enum):
    READY = "ready"
    STOPPING = "stopping"
    OFF = "off"
    STARTING = "starting"
    
@dataclass
class Machine:
    id:str
    name: str
    state: MachineState
    machineType: str
    publicIp: Optional[str] = None

    def to_json(self) -> dict:
        data = asdict(self)
        data['state'] = data['state'].value
        return data


class MachineRepository(BaseRepository):

    async def get_machine(self, machine_id:str) -> Optional[Machine]:
        conn  = await self.get_connection()
        sql = """
                SELECT * from server_machines
                WHERE id=$1;
        """
        row = await conn.fetchrow(sql, machine_id)
        if row:
            return Machine(
                id=row['id'],
                state=MachineState(row['state']),
                publicIp=row['ip_address'],
                name="FOODENIE",
                machineType="A4000"
            )


    async def create_machine(self, machine_id:str) -> Machine:
        state =  MachineState.OFF.value
        conn  = await self.get_connection()
        sql = """INSERT INTO server_machines
            (id, state, ip_address) 
            VALUES ($1, $2, null);"""
        
        await conn.execute(sql,machine_id,state)
        return Machine(
            id=machine_id,
            machineType="A4000",
            name="FODDENIE",
            state=state
        )



    async def update_machine(self, 
                       state:MachineState, 
                       machine_id:str,
                       ip_address: Optional[str]=None) -> Machine:
        conn  =  await self.get_connection()
        sql = """UPDATE server_machines
            SET state=$1, ip_address=$2 
            WHERE id=$3;"""
        
        await conn.execute(sql, state.value, ip_address, machine_id)

        return Machine(
            id=machine_id,
            machineType="A4000",
            name="FODDENIE",
            state=state
        )
    
