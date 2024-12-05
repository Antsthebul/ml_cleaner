FROM python:3.8.20-bullseye
RUN pip install "fastapi[standard]"
WORKDIR /app
COPY ./src/app /app
CMD uvicorn main:app --reload --host 0.0.0.0 --port 8000