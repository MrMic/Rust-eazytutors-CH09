/* Drop table if already existe */
DROP TABLE IF EXISTS ezy_course_c6;

/* Create table */
/* Don't put a comma after the last field */
CREATE TABLE ezy_course_c6
(
  course_id serial primary key,
  tutor_id int NOT NULL,
  course_name varchar(140) NOT NULL,
  course_description varchar(2000),
  course_format varchar(30),
  course_structure varchar(200),
  course_duration varchar(30),
  course_price INT,
  course_language varchar(30),
  course_level varchar(30),
  posted_time TIMESTAMP DEFAULT now()
);

/* Load seed data for testing */
-- insert into ezy_course_c5 
--   (course_id, tutor_id, course_name, posted_time) 
-- values (1, 1, 'First course', '2021-03-17 05:40:00');
-- insert into ezy_course_c5 
--   (course_id, tutor_id, course_name, posted_time) 
-- values (2, 1, 'Second course', '2021-03-18 05:45:00');
