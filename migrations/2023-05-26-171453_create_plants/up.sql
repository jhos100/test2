-- Your SQL goes here

CREATE DATABASE IF NOT EXISTS infraestructure_josh;
USE infraestructure_josh;

CREATE TABLE users (
    id INT NOT NULL AUTO_INCREMENT,
    username VARCHAR(255) NOT NULL,
    pass VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);