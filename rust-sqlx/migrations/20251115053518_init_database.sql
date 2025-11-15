-- Add migration script here
create table say_hello (
    id integer primary key autoincrement,
    message text not null
) strict;

create virtual table posts using fts5(author, content);
