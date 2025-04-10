CREATE TYPE consumption_type AS ENUM (
    'digest',
    'inhale_nose',
    'inhale_mouth',
    'spit_out',
    'inject',
    'apply_skin'
);
ALTER TABLE consumptions
ADD COLUMN consumption_type consumption_type NOT NULL DEFAULT 'digest';
