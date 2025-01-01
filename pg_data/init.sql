
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
    ip_address VARCHAR NULL,
    type VARCHAR NOT NULL,
    price REAL NOT NULL
);

CREATE TABLE server_machines (
    id VARCHAR UNIQUE,
    state machine_state NOT NULL,
    ip_address VARCHAR NULL
);

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()

);

CREATE TABLE deployments(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    project_id INT NOT NULL,

    CONSTRAINT fk_projects FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE machine_events(
    id SERIAL PRIMARY KEY,
    action TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    machine_id VARCHAR NOT NULL,

    CONSTRAINT fk_machines FOREIGN KEY (machine_id) REFERENCES machines(machine_id)
);

CREATE TABLE activity_log(
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    action TEXT NOT NULL  
);