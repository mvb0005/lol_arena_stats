# Player Search and Profile

## Goal
Allow users to search for a player and view a profile summary focused on Arena performance.

## Scope
- Search by Riot ID or summoner name/region combination.
- Return profile summary with recent Arena placements and key stats.
- Include last-updated metadata and basic error states.

## Acceptance Criteria
- Search endpoint resolves a valid player identity.
- Profile response contains at least recent placements, total games, and win-rate.
- Invalid player queries return clear 4xx errors.
