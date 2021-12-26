CREATE TABLE likes (
  first VARCHAR NOT NULL,
  second VARCHAR NOT NULL,
  cookie VARCHAR NOT NULL,
  timestamp TIMESTAMP,
  PRIMARY KEY(first, second, cookie)
)

