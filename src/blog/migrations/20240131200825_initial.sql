CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    author TEXT,
    content TEXT
);

INSERT INTO posts (title, author, content) VALUES ('post 1', 'test user', 'content 1 posted on 2024-10-28');
INSERT INTO posts (title, author, content) VALUES ('post 2', 'test user', 'content 2 posted on 2024-10-28');
INSERT INTO posts (title, author, content) VALUES ('post 3', 'test user', 'content 3 posted on 2024-10-29');
INSERT INTO posts (title, author, content) VALUES ('post 4', 'test user', 'content 4 posted on 2024-10-30');
INSERT INTO posts (title, author, content) VALUES ('post 5', 'test user', 'content 5 posted on 2024-10-30');