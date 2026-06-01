/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class PlayersService {
    /**
     * Resolve a player identity from Riot ID or summoner lookup
     * @param playerName
     * @param tagLine
     * @param region
     * @returns any Resolved player identity
     * @throws ApiError
     */
    public static searchPlayer(
        playerName: string,
        tagLine?: string,
        region: string = 'americas',
    ): CancelablePromise<{
        playerName: string;
        tagLine: string;
        region: string;
        puuid: string;
        lastUpdated: string;
    }> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/v1/players/search',
            query: {
                'playerName': playerName,
                'tagLine': tagLine,
                'region': region,
            },
            errors: {
                400: `Invalid player search query`,
                404: `Player not found`,
            },
        });
    }
    /**
     * Fetch Arena-focused profile summary for a player
     * @param puuid
     * @param region
     * @returns any Arena profile summary
     * @throws ApiError
     */
    public static getPlayerProfile(
        puuid: string,
        region: string = 'americas',
    ): CancelablePromise<{
        playerName: string;
        tagLine: string;
        region: string;
        puuid: string;
        recentPlacements: Array<number>;
        totalGames: number;
        winRate: number;
        lastUpdated: string;
    }> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/v1/players/{puuid}/profile',
            path: {
                'puuid': puuid,
            },
            query: {
                'region': region,
            },
            errors: {
                404: `Player profile not found`,
            },
        });
    }
}
