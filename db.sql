CREATE DATABASE IF NOT EXISTS chatter_db; USE chatter_db;

CREATE TABLE IF NOT EXISTS users (
  id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(15) NOT NULL,
);