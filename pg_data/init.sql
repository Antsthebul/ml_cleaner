
CREATE TABLE classes(
    id SERIAL PRIMARY KEY,
    name VARCHAR UNIQUE
);

CREATE TABLE verified_images ( 
    id SERIAL PRIMARY KEY,
    class_id INT NOT NULL,
    file_path TEXT NOT NULL UNIQUE,
    verified BOOLEAN DEFAULT false,

    CONSTRAINT fk_class FOREIGN KEY (class_id) REFERENCES classes(id)
);

CREATE TYPE machine_state AS ENUM(
    'off','starting','stopping','restarting','serviceready','ready','upgrading','provisioning'
);

CREATE TABLE machines (
    machine_id VARCHAR UNIQUE ,
    state machine_state NOT NULL,
    ip_address VARCHAR NOT NULL
);