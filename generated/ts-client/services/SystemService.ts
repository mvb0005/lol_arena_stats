/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class SystemService {
    /**
     * Backend health check
     * @returns any Service health
     * @throws ApiError
     */
    public static getHealth(): CancelablePromise<{
        service: string;
        status: string;
        version: string;
    }> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/health',
        });
    }
}
