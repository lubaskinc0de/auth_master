from typing import Annotated
from uuid import UUID

from fastapi import FastAPI, Header
from fastapi.exceptions import HTTPException

import uvicorn

app = FastAPI()


@app.get("/")
def root(x_auth_user: Annotated[UUID | None, Header()] = None):
    if x_auth_user is None:
        raise HTTPException(status_code=401, detail="X-Auth-User header is not present")
    return {"user": x_auth_user, "authenticated": True}


if __name__ == "__main__":
    uvicorn.run(app, port=8080, host="0.0.0.0")
