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
    /**
     * Fetch ranked arena player summaries
     * @param region
     * @param sortBy
     * @param sortOrder
     * @param page
     * @param pageSize
     * @returns any Ranked and paginated arena leaderboard
     * @throws ApiError
     */
    public static getArenaLeaderboard(
        region: string = 'americas',
        sortBy: 'winRate' | 'topFourRate' | 'averagePlacement' | 'matchesPlayed' = 'winRate',
        sortOrder: 'asc' | 'desc' = 'desc',
        page: number = 1,
        pageSize: number = 20,
    ): CancelablePromise<{
        entries: Array<{
            rank: number;
            averagePlacement: number;
            lastUpdated: string;
            matchesPlayed: number;
            playerName: string;
            region: string;
            tagLine: string;
            topFourRate: number;
            winRate: number;
        }>;
        page: number;
        pageSize: number;
        sortBy: 'winRate' | 'topFourRate' | 'averagePlacement' | 'matchesPlayed';
        sortOrder: 'asc' | 'desc';
        totalItems: number;
        totalPages: number;
    }> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/v1/arena/leaderboard',
            query: {
                'region': region,
                'sortBy': sortBy,
                'sortOrder': sortOrder,
                'page': page,
                'pageSize': pageSize,
            },
        });
    }
}
