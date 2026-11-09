ALTER TABLE exercises ALTER COLUMN duration DROP NOT NULL, ADD COLUMN complete BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE refluxs ALTER COLUMN duration DROP NOT NULL, ADD COLUMN complete BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE wees ALTER COLUMN duration DROP NOT NULL, ADD COLUMN complete BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE poos ALTER COLUMN duration DROP NOT NULL, ADD COLUMN complete BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE consumptions ALTER COLUMN duration DROP NOT NULL, ADD COLUMN complete BOOLEAN NOT NULL DEFAULT false;

UPDATE exercises SET complete = true WHERE true;
UPDATE refluxs SET complete = true WHERE true;
UPDATE wees SET complete = true WHERE true;
UPDATE poos SET complete = true WHERE true;
UPDATE consumptions SET complete = true WHERE true;

ALTER TABLE exercises ADD CONSTRAINT exercises_complete_duration_check CHECK (complete = false OR duration IS NOT NULL);
ALTER TABLE refluxs ADD CONSTRAINT refluxs_complete_duration_check CHECK (complete = false OR duration IS NOT NULL);
ALTER TABLE wees ADD CONSTRAINT wees_complete_duration_check CHECK (complete = false OR duration IS NOT NULL);
ALTER TABLE poos ADD CONSTRAINT poos_complete_duration_check CHECK (complete = false OR duration IS NOT NULL);
ALTER TABLE consumptions ADD CONSTRAINT consumptions_complete_duration_check CHECK (complete = false OR duration IS NOT NULL);
