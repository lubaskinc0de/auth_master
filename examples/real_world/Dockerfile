FROM python:3.13.3-alpine3.21

ENV APP_HOME=/home/app/
WORKDIR $APP_HOME

RUN pip install uv
COPY ./pyproject.toml $APP_HOME
RUN uv pip install . --system

COPY ./main.py $APP_HOME/main.py

ENTRYPOINT ["python", "main.py"]