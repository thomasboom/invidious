-- Migration 0008: Create playlists table and privacy enum

-- Create enum type if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'privacy') THEN
        CREATE TYPE public.privacy AS ENUM ('Public', 'Unlisted', 'Private');
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS public.playlists
(
    title text,
    id text primary key,
    author text,
    description text,
    video_count integer,
    created timestamptz,
    updated timestamptz,
    privacy privacy,
    index int8[]
);

GRANT ALL ON public.playlists TO current_user;
