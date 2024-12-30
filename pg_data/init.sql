
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
    'off','starting','stopping','restarting','serviceready','ready','upgrading','provisioning', 'training'
);

CREATE TABLE machines (
    machine_id VARCHAR UNIQUE ,
    state machine_state NOT NULL,
    ip_address VARCHAR NULL
);

CREATE TABLE server_machines (
    id VARCHAR UNIQUE,
    state machine_state NOT NULL,
    ip_address VARCHAR NULL
);

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL SET DEFAULT NOW()

);

CREATE TABLE deployments(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL SET DEFAULT NOW(),
    project_id INT NOT NULL,

    CONSTRAINT fk_projects FOREIGN KEY (project_id) REFERENCES projects(id)

);