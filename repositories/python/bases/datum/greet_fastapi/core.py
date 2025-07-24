from typing import Union

from fastapi import FastAPI

from components.datum import greeting
from components.datum.logger.core import get_logger

logger = get_logger("greet-FastAPI-logger")
app = FastAPI()

@app.get("/")
def root() -> dict:
    logger.info("The FastAPI root endpoint was called.")

    return {"message": greeting.hello_world()}


@app.get("/items/{item_id}")
def read_item(item_id: int, q: Union[str, None] = None):
    return {"item_id": item_id, "q": q}
