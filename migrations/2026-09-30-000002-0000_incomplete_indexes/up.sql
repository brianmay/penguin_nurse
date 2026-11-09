CREATE INDEX idx_wees_user_incomplete ON wees (user_id, complete) WHERE complete = false;
CREATE INDEX idx_poos_user_incomplete ON poos (user_id, complete) WHERE complete = false;
CREATE INDEX idx_exercises_user_incomplete ON exercises (user_id, complete) WHERE complete = false;
CREATE INDEX idx_consumptions_user_incomplete ON consumptions (user_id, complete) WHERE complete = false;
CREATE INDEX idx_refluxs_user_incomplete ON refluxs (user_id, complete) WHERE complete = false;
