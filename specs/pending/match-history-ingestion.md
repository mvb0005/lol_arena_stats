# Match History Ingestion

## Goal
Ingest recent Arena match history per player and store normalized match records for trend analysis.

## Scope
- Pull recent Arena matches by player identifier.
- Normalize participants, placements, and augments into backend-friendly models.
- Store ingestion timestamps for freshness checks.

## Acceptance Criteria
- Backend can fetch and persist recent Arena matches for one player.
- Data model supports placement history and teammate/opponent references.
- Endpoint responses can read ingested records without additional transformation.
