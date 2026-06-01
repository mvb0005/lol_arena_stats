/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class ArenaService {
    /**
     * Fetch aggregated arena stats for a player
     * @param playerName
     * @param tagLine
     * @param region
     * @returns any Aggregated arena stats snapshot
     * @throws ApiError
     */
    public static getArenaStats(
        playerName: string,
        tagLine?: string,
        region: string = 'americas',
    ): CancelablePromise<{
        averagePlacement: number;
        lastUpdated: string;
        matchesPlayed: number;
        playerName: string;
        region: string;
        tagLine: string;
        topFourRate: number;
        winRate: number;
    }> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/v1/arena/stats',
            query: {
                'playerName': playerName,
                'tagLine': tagLine,
                'region': region,
            },
        });
    }
}
