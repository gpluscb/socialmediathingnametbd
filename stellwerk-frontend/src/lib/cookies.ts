import type { Cookies } from '@sveltejs/kit';

const TEMP_SESS_ID_COOKIE_NAME = 'temp_sess_id';
const REDIRECT_COOKIE_NAME = 'redirect';
const TEMP_AUTH_COOKIE_PATH = '/auth';

const TOKEN_COOKIE_NAME = 'api_token';
const TOKEN_COOKIE_PATH = '/';

export function getTempAuthInfo(
    cookies: Cookies,
): { temp_sess_id: string; redirect: string | undefined } | undefined {
    let temp_sess_id = cookies.get(TEMP_SESS_ID_COOKIE_NAME);
    let redirect = cookies.get(REDIRECT_COOKIE_NAME);

    if (!temp_sess_id) return undefined;

    return {
        temp_sess_id,
        redirect,
    };
}

export function setTempAuthInfo(
    cookies: Cookies,
    temp_sess_id: string,
    redirect: string | undefined,
) {
    cookies.set(TEMP_SESS_ID_COOKIE_NAME, temp_sess_id, {
        path: TEMP_AUTH_COOKIE_PATH,
        maxAge: 30 * 60,
        httpOnly: true,
        sameSite: 'lax',
        // TODO: Make secure once tls is set up
        // secure: true,
    });

    if (redirect) {
        cookies.set('redirect', redirect, {
            path: TEMP_AUTH_COOKIE_PATH,
            // TODO: Configurable for the server
            maxAge: 30 * 60,
            httpOnly: true,
            sameSite: 'lax',
            // TODO: Make secure once tls is set up
            // secure: true,
        });
    }
}

export function deleteTempAuthInfo(cookies: Cookies) {
    cookies.delete(TEMP_SESS_ID_COOKIE_NAME, { path: TEMP_AUTH_COOKIE_PATH });
    cookies.delete(REDIRECT_COOKIE_NAME, { path: TEMP_AUTH_COOKIE_PATH });
}

export function getApiToken(cookies: Cookies): string | undefined {
    return cookies.get(TOKEN_COOKIE_NAME);
}

export function setApiToken(cookies: Cookies, token: string, expires: Date | undefined) {
    cookies.set(TOKEN_COOKIE_NAME, token, {
        path: TOKEN_COOKIE_PATH,
        expires,
        httpOnly: true,
        sameSite: true,
        // TODO: Make secure once tls is set up
        // secure: true,
    });
}
