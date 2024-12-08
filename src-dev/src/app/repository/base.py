import asyncpg


class BaseRepository:

    
    async def set_connection(self) -> None:
        self._connection = await asyncpg.connect("postgresql://ml_cleaner:ml_cleaner@host.docker.internal/local_db")

    async def get_connection(self) -> asyncpg.Connection:
        if self._connection is None:
            raise Exception("`set_connection.()` is required to be called before accessing any methods")
        return self._connection
    
    
    async def close (cls) -> None:
        if cls._connection:
            await cls._connection.close()
            cls._connection = None