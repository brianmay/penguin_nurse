CREATE TYPE sex AS ENUM ('male', 'female');

ALTER TABLE users
ADD COLUMN date_of_birth DATE,
ADD COLUMN sex sex;
