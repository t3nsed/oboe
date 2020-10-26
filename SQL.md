create table threads (
threadid integer not null,
poster varchar(255) not null,
title varchar(255) not null,
body varchar(255) not null,
img varchar(255),
time varchar(64) not null,
date varchar(64) not null
);

create table posts (
threadid integer not null,
poster varchar(255) not null,
body varchar(255) not null,
img varchar(255),
time varchar(64) not null,
date varchar(64) not null,
postid integer not null
);