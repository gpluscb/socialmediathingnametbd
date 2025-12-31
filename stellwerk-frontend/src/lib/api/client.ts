import createClient, {type Client} from "openapi-fetch";
import type {paths, components} from "$lib/generated/openapi-schema";

// TODO: Configurable
const BASE_URL: string = "http://localhost:8080/";

export type Post = components["schemas"]["Post"];

export function apiClient(token: string): Client<paths> {
    return createClient<paths>({
        baseUrl: BASE_URL,
        headers: {
            "Authorization": `Bearer ${token}`,
        }
    });
}
