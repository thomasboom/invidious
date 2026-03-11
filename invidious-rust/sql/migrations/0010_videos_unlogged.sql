-- Migration 0010: Make videos table unlogged

ALTER TABLE public.videos SET UNLOGGED;
