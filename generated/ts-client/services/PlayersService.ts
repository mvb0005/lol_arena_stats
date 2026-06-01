/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class PlayersService {
    /**
     * Search players by Riot ID game name and tagline
     * @param gameName
     * @param tagline
     * @returns any Search results
     * @throws ApiError
     */
    public static searchPlayers(
        gameName: string,
        tagline: string,
    ): CancelablePromise<{
        results: Array<{
            puuid: string;
            game_name: string;
            tagline: string;
        }>;
    }> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/lol/players/search',
            query: {
                'game_name': gameName,
                'tagline': tagline,
            },
            errors: {
                502: `Upstream Riot/API failure`,
            },
        });
    }
    /**
     * Get player profile and recent Arena matches
     * @param puuid
     * @returns any Player profile with recent matches
     * @throws ApiError
     */
    public static getPlayerProfile(
        puuid: string,
    ): CancelablePromise<{
        player: {
            puuid: string;
            game_name: string;
            tagline: string;
            summoner_id: string;
            summoner_level: number;
            profile_icon_id: number;
            arena_rank?: {
                tier: string;
                rank: string;
                league_points: number;
            };
            active_arena?: {
                in_game: boolean;
                queue_id: number;
                game_start_time: number;
            };
        };
        recent_matches: Array<{
            match_id: string;
            queue_id: number;
            game_creation: number;
            game_end_timestamp: number;
            participants: Array<{
                puuid: string;
                placement: number;
                champion_id: number;
                kills: number;
                deaths: number;
                assists: number;
            }>;
        }>;
    }> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/lol/players/{puuid}',
            path: {
                'puuid': puuid,
            },
            errors: {
                404: `Player not found`,
                502: `Upstream Riot/API failure`,
            },
        });
    }
}
