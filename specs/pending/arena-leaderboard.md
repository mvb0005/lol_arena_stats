# Arena Leaderboard

## Goal
Provide a leaderboard view to rank players by Arena performance metrics.

## Scope
- Expose backend endpoint for ranked player summaries.
- Support pagination and region filtering.
- Sort by selected metric (e.g., top-1 rate, average placement, games played).

## Acceptance Criteria
- Endpoint returns deterministic ordering for the selected metric.
- Pagination metadata is included and accurate.
- Region filtering limits results correctly.
