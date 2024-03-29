CREATE DATABASE IF NOT EXISTS chatter_db; USE chatter_db;

CREATE TABLE IF NOT EXISTS users (
  u_id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(64) NOT NULL,
  pw VARCHAR(64) NOT NULL,
  status_tag TINYINT NOT NULL DEFAULT 0
)engine=innodb;

CREATE TABLE IF NOT EXISTS chatrooms (
  c_id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  user_1 VARCHAR(64),
  user_2 VARCHAR(64)
)engine=innodb;

CREATE TABLE IF NOT EXISTS messages (
  m_id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  c_id BIGINT NOT NULL,
  author VARCHAR(64) NOT NULL,
  tx_time DATETIME DEFAULT CURRENT_TIMESTAMP,
  chat_message TEXT,
  KEY c_id (c_id)
)engine=innodb;
