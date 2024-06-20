CREATE TABLE IF NOT EXISTS subscription(
  id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,

  subscribed_at timestamptz NOT NULL
)
