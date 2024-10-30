CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    author TEXT,
    content TEXT
);

INSERT INTO posts (title, author, content) VALUES ('post 1', 'test user', 'this is test content 1');
INSERT INTO posts (title, author, content) VALUES ('post 2', 'test user', 'this is test content 2');
INSERT INTO posts (title, author, content) VALUES ('post 3', 'test user', 'this is test content 3');
INSERT INTO posts (title, author, content) VALUES ('post 4', 'test user', 'this is test content 4');
INSERT INTO posts (title, author, content) VALUES ('post 5', 'test user', 'this is test content 5');

CREATE TABLE about (
    content TEXT UNIQUE
);

INSERT INTO about (content) VALUES ('Information about the site and organization');