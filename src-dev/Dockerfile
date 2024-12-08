FROM python:3.8.20-bullseye

COPY  ./entrypoint.sh /entrypoint.sh
COPY ./pub_key.pem ./pub_key.pem

RUN apt update && apt install -y openssh-server && \
    pip install "fastapi[standard]" asyncpg &&  \
    chmod +x /entrypoint.sh && \
    mkdir -p /root/.ssh && \
    touch /root/.ssh/authorized_keys && \
    cat ./pub_key.pem >> /root/.ssh/authorized_keys && \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config && \
    sed -i 's/#ListenAddress 0.0.0.0/ListenAddress 0.0.0.0/' /etc/ssh/sshd_config

ENTRYPOINT ["/entrypoint.sh"]
WORKDIR /app
CMD uvicorn main:app --reload --host 0.0.0.0 --port 8000